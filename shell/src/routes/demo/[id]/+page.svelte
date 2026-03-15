<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { session } from '$lib/session.svelte.js';
	import { registerShellAPI, loadDemo, setParam } from '$lib/ludeme-shell.js';
	import type { PageData } from './$types.js';
	import PublishPanel      from '$lib/components/PublishPanel.svelte';
	import ParamTunerPanel   from '$lib/components/ParamTunerPanel.svelte';
	import ExperimentPanel   from '$lib/components/ExperimentPanel.svelte';
	import ObservationsPanel from '$lib/components/ObservationsPanel.svelte';
	import BookmarkPanel     from '$lib/components/BookmarkPanel.svelte';

	let { data } = $props();
	let demo          = $derived(data.demo);
	let linkedWork    = $derived(data.linkedWork);
	let linkedMechanics = $derived(data.linkedMechanics ?? []);

	// Bookmark capture state (lives here because it's tied to the canvas + keyboard shortcut)
	let bookmarkLabel = $state('');
	let bookmarkTags  = $state<string[]>([]);
	$effect(() => { bookmarkTags = [...demo.mechanic_tags]; });

	let canvasEl  = $state<HTMLCanvasElement | null>(null);

	const FIDELITY_LABELS: Record<string, string> = {
		faithful:     'Faithful',
		interpreted:  'Interpreted',
		experimental: 'Experimental',
	};

	// ---------------------------------------------------------------------------
	// Lifecycle
	// ---------------------------------------------------------------------------

	onMount(() => {
		registerShellAPI();
		session.setDemo(demo);

		if (demo.wasm_path) loadDemo(demo.wasm_path);
	});

	onDestroy(() => session.reset());

	// ---------------------------------------------------------------------------
	// Param + bookmark handlers (passed down/used locally)
	// ---------------------------------------------------------------------------

	function handleParamChange(key: string, value: number) {
		setParam(key, value);
	}

	function confirmBookmark() {
		session.confirmBookmark(bookmarkLabel, bookmarkTags);
		bookmarkLabel = '';
	}

	// Keyboard shortcuts: B = capture bookmark, Escape = dismiss
	function handleKeydown(e: KeyboardEvent) {
		if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;
		if (e.key === 'b' || e.key === 'B') {
			e.preventDefault();
			session.captureBookmark();
		} else if (e.key === 'Escape' && session.isPaused) {
			session.dismissMoment();
		}
	}
</script>

<svelte:head>
	<title>{demo.title} — Ludeme</title>
	<meta name="description" content={demo.description ?? `Play and study the ${demo.title} mechanic demo.`} />
</svelte:head>

<svelte:window onkeydown={handleKeydown} />

<div class="play-shell">

	<!-- ===================== Header ===================== -->
	<header class="demo-header">
		<div class="demo-breadcrumb">
			<a href="/">Discover</a>
			{#if linkedWork}
				<span class="sep">›</span>
				<a href="/works/{linkedWork.id}">{linkedWork.title}</a>
			{/if}
			<span class="sep">›</span>
			<span>{demo.title}</span>
		</div>
		<div class="demo-header-right">
			{#each linkedMechanics as mech}
				<a href="/mechanics/{mech.id}" class="chip teal chip-link">{mech.family.replace(/-/g, ' ')}</a>
			{/each}
			{#if linkedMechanics.length === 0}
				{#each demo.mechanic_tags as tag}
					<span class="chip teal">{tag.replace(/-/g, ' ')}</span>
				{/each}
			{/if}
			<span class="badge {demo.fidelity_level}">
				{FIDELITY_LABELS[demo.fidelity_level]}
			</span>
		</div>
	</header>

	<!-- ===================== Main play area ===================== -->
	<div class="play-area">

		<!-- Canvas column -->
		<div class="canvas-col">
			<div class="canvas-wrap">

				{#if session.status === 'loading'}
					<div class="canvas-overlay">
						<div class="spinner" aria-label="Loading demo"></div>
						<p class="overlay-label">Loading demo…</p>
					</div>

				{:else if session.status === 'error'}
					<div class="canvas-overlay canvas-overlay--error">
						<span class="overlay-icon">⚠</span>
						<p class="overlay-label">{session.error?.message ?? 'Unknown error'}</p>
						<p class="overlay-sub">Check the console for details.</p>
					</div>

				{:else if session.status === 'idle' || !demo.wasm_path}
					<div class="canvas-overlay canvas-overlay--idle">
						<span class="overlay-icon">▶</span>
						<p class="overlay-label">{demo.title}</p>
						<p class="overlay-sub">Demo crate not yet compiled.</p>
						<p class="overlay-sub">WASM output will embed here in Phase 1.</p>
					</div>

				{:else if session.status === 'paused'}
					<div class="canvas-overlay canvas-overlay--paused">
						<span class="overlay-icon">⏸</span>
						<p class="overlay-label">Session paused — capturing moment</p>
					</div>
				{/if}

				<!-- Canvas — always present for WASM to target -->
				<canvas
					bind:this={canvasEl}
					id="ludeme-canvas"
					class="game-canvas"
					class:dim={session.isPaused}
					aria-label="{demo.title} interactive demo"
				></canvas>

				<!-- Controls hint bar -->
				<div class="controls-bar">
					<span class="control-hint">
						<kbd>B</kbd> bookmark moment
					</span>
					{#if session.paramManifest?.params.length}
						<span class="control-hint">
							<kbd>P</kbd> toggle tuner
						</span>
					{/if}
					<span class="status-chip" class:active={session.isActive}>
						{session.isActive ? `Frame ${session.frameCount}` : session.status}
					</span>
				</div>
			</div>

			<!-- Fidelity note -->
			{#if demo.fidelity_level === 'interpreted' && demo.notable_interpretations?.length}
				<details class="fidelity-note">
					<summary>
						<span class="badge interpreted">Interpreted</span>
						— documented departures from the original
					</summary>
					<ul>
						{#each demo.notable_interpretations as note}
							<li>{note}</li>
						{/each}
					</ul>
				</details>
			{:else if demo.fidelity_level === 'experimental' && demo.hypothesis}
				<div class="fidelity-note fidelity-note--experimental">
					<span class="badge experimental">Experimental</span>
					<p class="hypothesis">{demo.hypothesis}</p>
				</div>
			{/if}

			<div class="demo-description">
				<p>{demo.description}</p>
			</div>
		</div>

		<!-- ===================== Sidebar ===================== -->
		<aside class="sidebar">

			<PublishPanel {demo} />

			<ParamTunerPanel {session} onParamChange={handleParamChange} />

			<ExperimentPanel {session} {demo} />

			<!-- State machine panel (lightweight — kept inline, no drawer needed) -->
			{#if demo.state_graph?.states.length}
				<div class="panel">
					<div class="panel-header-static">
						<span class="panel-title">State machine</span>
					</div>
					<div class="panel-body state-panel">
						{#each demo.state_graph.states as state}
							<div
								class="state-node"
								class:state-node--active={session.currentStates.includes(state.id)}
							>
								<span class="state-dot"></span>
								<span class="state-label">{state.label}</span>
							</div>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Pending bookmark capture (user-triggered B key) -->
			{#if session.isPaused && session.pendingMoment}
				<div class="panel panel--highlight">
					<div class="panel-header-static">
						<span class="panel-title">📌 Capture moment</span>
					</div>
					<div class="panel-body">
						<p class="moment-frame">Frame {session.pendingMoment.frame}</p>
						<input
							type="text"
							class="moment-input"
							placeholder="Label this moment…"
							bind:value={bookmarkLabel}
						/>
						<div class="moment-tags">
							{#each session.pendingMoment.auto_tags as tag}
								<span class="chip teal">{tag}</span>
							{/each}
						</div>
						<div class="moment-actions">
							<button class="btn btn-primary btn-sm" onclick={confirmBookmark}>
								Save bookmark
							</button>
							<button class="btn btn-ghost btn-sm" onclick={() => session.dismissMoment()}>
								Dismiss
							</button>
						</div>
					</div>
				</div>
			{/if}

			<BookmarkPanel {session} />

			<ObservationsPanel {demo} />

		</aside>
	</div>
</div>

<style>
	/* ---- Shell layout ---- */
	.play-shell {
		display: flex;
		flex-direction: column;
		min-height: calc(100vh - var(--nav-height));
		background: var(--bg-base);
	}

	/* ---- Header ---- */
	.demo-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-4) var(--space-6);
		border-bottom: 1px solid var(--border);
		gap: var(--space-4);
		flex-wrap: wrap;
		max-width: var(--content-max);
		width: 100%;
		margin: 0 auto;
	}
	.demo-breadcrumb {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: 0.875rem;
		color: var(--text-secondary);
	}
	.demo-breadcrumb a { color: var(--text-secondary); }
	.demo-breadcrumb a:hover { color: var(--text-primary); }
	.sep { color: var(--text-muted); }
	.demo-header-right {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		flex-wrap: wrap;
	}

	/* ---- Play area ---- */
	.play-area {
		display: grid;
		grid-template-columns: 1fr var(--sidebar-width);
		gap: var(--space-6);
		padding: var(--space-6);
		flex: 1;
		max-width: var(--content-max);
		width: 100%;
		margin: 0 auto;
		/* stretch: both columns grow to the taller of the two */
		align-items: start;
	}

	/* ---- Sidebar ---- */
	.sidebar {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		/* Sticky so the top panels stay in view while the canvas is playing,
		   but ONLY up to the point the sidebar itself would go off-screen.
		   We use a tall max-height so it never clips. The page scrolls. */
		position: sticky;
		top: calc(var(--nav-height) + var(--space-4));
		/* Allow the sidebar to be as tall as needed — page scrolls, not sidebar */
		max-height: none;
		overflow-y: visible;
		padding-bottom: var(--space-8);
	}

	/* ---- Canvas column ---- */
	.canvas-col {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}
	.canvas-wrap {
		position: relative;
		background: #000;
		border-radius: var(--radius-lg);
		overflow: hidden;
		border: 1px solid var(--border);
		aspect-ratio: 4 / 3;
	}
	.game-canvas {
		display: block;
		width: 100%;
		height: 100%;
		object-fit: contain;
		transition: filter var(--duration) var(--ease);
	}
	.game-canvas.dim { filter: brightness(0.4) blur(2px); }

	/* Overlay states */
	.canvas-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: var(--space-3);
		z-index: 10;
		background: rgba(10, 10, 15, 0.7);
		backdrop-filter: blur(4px);
	}
	.canvas-overlay--error  { background: rgba(30, 0, 0, 0.8); }
	.canvas-overlay--idle   { background: rgba(10, 10, 15, 0.85); }
	.canvas-overlay--paused { background: rgba(10, 10, 15, 0.6); }
	.overlay-icon { font-size: 2.5rem; color: var(--accent); opacity: 0.8; }
	.canvas-overlay--error .overlay-icon { color: #f87171; }
	.overlay-label { font-size: 1rem; font-weight: 600; color: var(--text-primary); }
	.overlay-sub {
		font-size: 0.8125rem;
		color: var(--text-muted);
		text-align: center;
		max-width: 320px;
	}

	/* Spinner */
	.spinner {
		width: 32px; height: 32px;
		border: 2px solid var(--border);
		border-top-color: var(--accent);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}
	@keyframes spin { to { transform: rotate(360deg); } }

	/* Controls bar */
	.controls-bar {
		position: absolute;
		bottom: 0; left: 0; right: 0;
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-2) var(--space-3);
		background: rgba(10, 10, 15, 0.85);
		backdrop-filter: blur(8px);
		border-top: 1px solid var(--border);
		z-index: 5;
	}
	.control-hint {
		font-size: 0.75rem;
		color: var(--text-muted);
		display: flex;
		align-items: center;
		gap: var(--space-1);
	}
	kbd {
		background: var(--bg-subtle);
		border: 1px solid var(--border);
		border-radius: 3px;
		padding: 0 4px;
		font-family: var(--font-mono);
		font-size: 0.7rem;
		color: var(--text-secondary);
	}
	.status-chip {
		margin-left: auto;
		font-size: 0.7rem;
		font-family: var(--font-mono);
		color: var(--text-muted);
		padding: 2px 6px;
		background: var(--bg-subtle);
		border-radius: var(--radius-sm);
	}
	.status-chip.active { color: var(--teal); }

	/* Fidelity note */
	.fidelity-note {
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		padding: var(--space-3) var(--space-4);
		font-size: 0.875rem;
		color: var(--text-secondary);
	}
	.fidelity-note summary {
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}
	.fidelity-note ul {
		margin-top: var(--space-3);
		padding-left: var(--space-5);
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}
	.fidelity-note--experimental { display: flex; flex-direction: column; gap: var(--space-2); }
	.hypothesis { font-size: 0.875rem; color: var(--text-secondary); font-style: italic; }
	.demo-description p { font-size: 0.9375rem; color: var(--text-secondary); line-height: 1.7; }

	/* ---- Shared panel shell (state machine + bookmark capture stay inline) ---- */
	.panel {
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		overflow: hidden;
	}
	.panel--highlight {
		border-color: var(--border-accent);
		box-shadow: var(--shadow-glow);
	}
	.panel-header-static {
		padding: var(--space-3) var(--space-4);
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--text-primary);
	}
	.panel-title {
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--text-primary);
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}
	.panel-body {
		padding: var(--space-3) var(--space-4);
		border-top: 1px solid var(--border);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	/* State machine */
	.state-panel { gap: var(--space-2); }
	.state-node {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-1) var(--space-2);
		border-radius: var(--radius-sm);
		transition: background var(--duration-fast) var(--ease);
	}
	.state-node--active { background: var(--teal-muted); }
	.state-dot {
		width: 8px; height: 8px;
		border-radius: 50%;
		background: var(--border);
		flex-shrink: 0;
		transition: background var(--duration-fast) var(--ease);
	}
	.state-node--active .state-dot { background: var(--teal); }
	.state-label { font-size: 0.8125rem; color: var(--text-secondary); }
	.state-node--active .state-label { color: var(--teal); font-weight: 500; }

	/* Bookmark capture */
	.moment-frame { font-family: var(--font-mono); font-size: 0.8125rem; color: var(--text-muted); }
	.moment-input {
		background: var(--bg-overlay);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		color: var(--text-primary);
		font-family: var(--font-sans);
		font-size: 0.875rem;
		padding: var(--space-2) var(--space-3);
		outline: none;
		width: 100%;
		transition: border-color 120ms ease, box-shadow 120ms ease;
	}
	.moment-input:focus {
		border-color: var(--accent-dim);
		box-shadow: 0 0 0 3px var(--accent-glow);
	}
	.moment-input::placeholder { color: var(--text-muted); }
	.moment-tags { display: flex; flex-wrap: wrap; gap: var(--space-1); }
	.moment-actions { display: flex; gap: var(--space-2); flex-wrap: wrap; }
	.btn-sm { font-size: 0.75rem; padding: var(--space-1) var(--space-3); }

	/* Clickable chips */
	.chip-link {
		text-decoration: none;
		cursor: pointer;
		transition: background var(--duration-fast) var(--ease), transform var(--duration-fast) var(--ease);
	}
	.chip-link:hover { background: var(--teal-muted); transform: translateY(-1px); }

	/* ---- Responsive ---- */
	@media (max-width: 900px) {
		.play-area {
			grid-template-columns: 1fr;
		}
		.sidebar {
			position: static;
			max-height: none;
		}
	}
</style>
