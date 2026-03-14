<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { session } from '$lib/session.svelte.js';
	import { registerShellAPI, loadDemo, setParam } from '$lib/ludeme-shell.js';
	import type { PageData } from './$types.js';

	let { data }: { data: PageData } = $props();
	const demo = data.demo;

	// Panel visibility
	let showParamTuner  = $state(true);
	let showStatePanel  = $state(true);
	let showBookmarks   = $state(false);

	// Bookmark form
	let bookmarkLabel = $state('');
	let bookmarkTags  = $state<string[]>([...demo.mechanic_tags]);

	// Canvas ref for screenshot on bookmark
	let canvasEl = $state<HTMLCanvasElement | null>(null);

	// Helper: fidelity label
	const FIDELITY_LABELS: Record<string, string> = {
		faithful:     'Faithful',
		interpreted:  'Interpreted',
		experimental: 'Experimental',
	};

	// ---------------------------------------------------------------------------
	// Lifecycle
	// ---------------------------------------------------------------------------

	onMount(() => {
		// Always set up the shell API on mount — even if no WASM yet
		registerShellAPI();
		session.setDemo(demo);

		if (demo.wasm_path) {
			loadDemo(demo.wasm_path);
		}
		// If no wasm_path: session stays 'idle', canvas shows placeholder
	});

	onDestroy(() => {
		session.reset();
	});

	// ---------------------------------------------------------------------------
	// Param tuner interaction
	// ---------------------------------------------------------------------------

	function handleParamChange(key: string, value: number) {
		setParam(key, value);
	}

	// ---------------------------------------------------------------------------
	// Bookmark capture
	// ---------------------------------------------------------------------------

	function confirmBookmark() {
		session.confirmBookmark(bookmarkLabel, bookmarkTags);
		bookmarkLabel = '';
	}
</script>

<svelte:head>
	<title>{demo.title} — Ludeme</title>
	<meta name="description" content={demo.description ?? `Play and study the ${demo.title} mechanic demo.`} />
</svelte:head>

<div class="play-shell">

	<!-- ===================== Header ===================== -->
	<header class="demo-header">
		<div class="demo-breadcrumb">
			<a href="/">Discover</a>
			<span class="sep">›</span>
			<span>{demo.title}</span>
		</div>
		<div class="demo-header-right">
			{#each demo.mechanic_tags as tag}
				<span class="chip teal">{tag.replace(/-/g, ' ')}</span>
			{/each}
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
					<!-- Loading state -->
					<div class="canvas-overlay">
						<div class="spinner" aria-label="Loading demo"></div>
						<p class="overlay-label">Loading demo…</p>
					</div>

				{:else if session.status === 'error'}
					<!-- Error state -->
					<div class="canvas-overlay canvas-overlay--error">
						<span class="overlay-icon">⚠</span>
						<p class="overlay-label">{session.error?.message ?? 'Unknown error'}</p>
						<p class="overlay-sub">Check the console for details.</p>
					</div>

				{:else if session.status === 'idle' || !demo.wasm_path}
					<!-- No WASM yet — placeholder -->
					<div class="canvas-overlay canvas-overlay--idle">
						<span class="overlay-icon">▶</span>
						<p class="overlay-label">{demo.title}</p>
						<p class="overlay-sub">Demo crate not yet compiled.</p>
						<p class="overlay-sub">WASM output will embed here in Phase 1.</p>
					</div>

				{:else if session.status === 'paused'}
					<!-- Paused for bookmark capture -->
					<div class="canvas-overlay canvas-overlay--paused">
						<span class="overlay-icon">⏸</span>
						<p class="overlay-label">Session paused — capturing moment</p>
					</div>
				{/if}

				<!-- The actual canvas — always present for WASM to target -->
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

			<!-- Param tuner panel -->
			{#if session.paramManifest?.params.length}
				<div class="panel">
					<button
						class="panel-header"
						onclick={() => showParamTuner = !showParamTuner}
						aria-expanded={showParamTuner}
					>
						<span class="panel-title">Parameter tuner</span>
						<span class="panel-toggle">{showParamTuner ? '▲' : '▼'}</span>
					</button>
					{#if showParamTuner}
						<div class="panel-body">
							{#each session.paramManifest.params as param}
								<div class="param-row">
									<label class="param-label" for="param-{param.key}">
										{param.label}
										<span class="param-value">
											{session.paramValues[param.key]?.toFixed(param.kind === 'float' ? 2 : 0) ?? param.default}
										</span>
									</label>
									{#if param.kind === 'toggle'}
										<input
											type="checkbox"
											id="param-{param.key}"
											checked={Boolean(session.paramValues[param.key])}
											onchange={(e) => handleParamChange(param.key, e.currentTarget.checked ? 1 : 0)}
											class="param-toggle"
										/>
									{:else}
										<input
											type="range"
											id="param-{param.key}"
											min={param.min}
											max={param.max}
											step={param.step}
											value={session.paramValues[param.key] ?? param.default}
											oninput={(e) => handleParamChange(param.key, Number(e.currentTarget.value))}
											class="param-slider"
										/>
									{/if}
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/if}

			<!-- State machine panel -->
			{#if demo.state_graph?.states.length}
				<div class="panel">
					<button
						class="panel-header"
						onclick={() => showStatePanel = !showStatePanel}
						aria-expanded={showStatePanel}
					>
						<span class="panel-title">State machine</span>
						<span class="panel-toggle">{showStatePanel ? '▲' : '▼'}</span>
					</button>
					{#if showStatePanel}
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
					{/if}
				</div>
			{/if}

			<!-- Moment bookmark capture -->
			{#if session.isPaused && session.pendingMoment}
				<div class="panel panel--highlight">
					<div class="panel-header no-btn">
						<span class="panel-title">📌 Capture moment</span>
					</div>
					<div class="panel-body">
						<p class="moment-frame">Frame {session.pendingMoment.frame}</p>
						<input
							type="text"
							class="input moment-label-input"
							placeholder="Label this moment…"
							bind:value={bookmarkLabel}
						/>
						<div class="moment-tags">
							{#each session.pendingMoment.auto_tags as tag}
								<span class="chip teal">{tag}</span>
							{/each}
						</div>
						<div class="moment-actions">
							<button class="btn btn-primary" onclick={confirmBookmark}>
								Save bookmark
							</button>
							<button class="btn btn-ghost" onclick={() => session.dismissMoment()}>
								Dismiss
							</button>
						</div>
					</div>
				</div>
			{/if}

			<!-- Saved bookmarks -->
			{#if session.sessionBookmarks.length}
				<div class="panel">
					<button
						class="panel-header"
						onclick={() => showBookmarks = !showBookmarks}
						aria-expanded={showBookmarks}
					>
						<span class="panel-title">
							Bookmarks
							<span class="panel-badge">{session.sessionBookmarks.length}</span>
						</span>
						<span class="panel-toggle">{showBookmarks ? '▲' : '▼'}</span>
					</button>
					{#if showBookmarks}
						<div class="panel-body">
							{#each session.sessionBookmarks as bm}
								<div class="bookmark-row">
									<span class="bookmark-frame">f{bm.frame}</span>
									<span class="bookmark-label">{bm.player_label ?? 'Untitled'}</span>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/if}

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
		grid-template-columns: 1fr 300px;
		gap: var(--space-6);
		padding: var(--space-6);
		flex: 1;
		max-width: var(--content-max);
		width: 100%;
		margin: 0 auto;
		align-items: start;
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
	.canvas-overlay--error { background: rgba(30, 0, 0, 0.8); }
	.canvas-overlay--idle  { background: rgba(10, 10, 15, 0.85); }
	.canvas-overlay--paused { background: rgba(10, 10, 15, 0.6); }

	.overlay-icon {
		font-size: 2.5rem;
		color: var(--accent);
		opacity: 0.8;
	}
	.canvas-overlay--error .overlay-icon { color: #f87171; }

	.overlay-label {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary);
	}
	.overlay-sub {
		font-size: 0.8125rem;
		color: var(--text-muted);
		text-align: center;
		max-width: 320px;
	}

	/* Spinner */
	.spinner {
		width: 32px;
		height: 32px;
		border: 2px solid var(--border);
		border-top-color: var(--accent);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}
	@keyframes spin { to { transform: rotate(360deg); } }

	/* Controls bar */
	.controls-bar {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
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
	.fidelity-note--experimental {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}
	.hypothesis {
		font-size: 0.875rem;
		color: var(--text-secondary);
		font-style: italic;
	}

	.demo-description p {
		font-size: 0.9375rem;
		color: var(--text-secondary);
		line-height: 1.7;
	}

	/* ---- Sidebar ---- */
	.sidebar {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		position: sticky;
		top: calc(var(--nav-height) + var(--space-4));
		max-height: calc(100vh - var(--nav-height) - var(--space-8));
		overflow-y: auto;
	}

	/* Panel */
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
	.panel-header {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-4);
		background: none;
		border: none;
		cursor: pointer;
		text-align: left;
	}
	.panel-header.no-btn { cursor: default; }
	.panel-header:hover { background: rgba(255,255,255,0.03); }
	.panel-title {
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--text-primary);
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}
	.panel-toggle {
		font-size: 0.625rem;
		color: var(--text-muted);
	}
	.panel-badge {
		background: var(--accent-muted);
		color: var(--accent);
		border-radius: 10px;
		padding: 0 6px;
		font-size: 0.7rem;
	}
	.panel-body {
		padding: var(--space-3) var(--space-4);
		border-top: 1px solid var(--border);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	/* Param rows */
	.param-row {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}
	.param-label {
		display: flex;
		justify-content: space-between;
		font-size: 0.8125rem;
		color: var(--text-secondary);
	}
	.param-value {
		font-family: var(--font-mono);
		font-size: 0.75rem;
		color: var(--accent);
	}
	.param-slider {
		width: 100%;
		accent-color: var(--accent);
		cursor: pointer;
	}
	.param-toggle {
		width: 16px;
		height: 16px;
		accent-color: var(--accent);
		cursor: pointer;
	}

	/* State panel */
	.state-panel { gap: var(--space-2); }
	.state-node {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-1) var(--space-2);
		border-radius: var(--radius-sm);
		transition: background var(--duration-fast) var(--ease);
	}
	.state-node--active {
		background: var(--teal-muted);
	}
	.state-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--border);
		flex-shrink: 0;
		transition: background var(--duration-fast) var(--ease);
	}
	.state-node--active .state-dot { background: var(--teal); }
	.state-label {
		font-size: 0.8125rem;
		color: var(--text-secondary);
	}
	.state-node--active .state-label { color: var(--teal); font-weight: 500; }

	/* Moment capture */
	.moment-frame {
		font-family: var(--font-mono);
		font-size: 0.8125rem;
		color: var(--text-muted);
	}
	.moment-label-input { font-size: 0.875rem; }
	.moment-tags {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-1);
	}
	.moment-actions {
		display: flex;
		gap: var(--space-2);
		flex-wrap: wrap;
	}

	/* Bookmarks */
	.bookmark-row {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		font-size: 0.8125rem;
	}
	.bookmark-frame {
		font-family: var(--font-mono);
		font-size: 0.75rem;
		color: var(--text-muted);
		flex-shrink: 0;
	}
	.bookmark-label { color: var(--text-secondary); }

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
