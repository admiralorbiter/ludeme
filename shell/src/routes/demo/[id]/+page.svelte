<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { session } from '$lib/session.svelte.js';
	import { registerShellAPI, loadDemo, setParam } from '$lib/ludeme-shell.js';
	import type { PageData } from './$types.js';
	import { api } from '$lib/api';

	let { data } = $props();
	let demo = $derived(data.demo);
	let linkedWork = $derived(data.linkedWork);
	let linkedMechanics = $derived(data.linkedMechanics ?? []);

	// Panel visibility
	let showParamTuner  = $state(true);
	let showStatePanel  = $state(true);
	let showBookmarks   = $state(false);
	let showPublish     = $state(true);

	// Experiment form
	let showExperimentForm = $state(false);
	let experimentHypothesis = $state('');
	let experimentExpected = $state('');
	let experimentSaving = $state(false);
	let experimentSaved = $state(false);

	// Observation form
	let showObservations = $state(false);
	let showObsForm = $state(false);
	let obsClaim = $state('');
	let obsConfidence = $state('tentative');
	let obsWhy = $state('');
	let obsFollowUp = $state('');
	let obsSaving = $state(false);
	let obsSaved = $state(false);
	let savedObservations = $state<any[]>([]);

	// Sidebar scroll
	let sidebarEl = $state<HTMLElement | null>(null);

	// Publish state management
	let publishState = $state('draft');
	let readiness = $state<{ready: boolean; checks: Array<{field: string; ok: boolean; message: string}>} | null>(null);
	let publishLoading = $state(false);
	$effect(() => { publishState = demo.publish_state ?? 'draft'; });

	// Bookmark form
	let bookmarkLabel = $state('');
	let bookmarkTags  = $state<string[]>([]);
	$effect(() => { bookmarkTags = [...demo.mechanic_tags]; });

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

		// Load readiness checklist
		fetchReadiness();

		// Load saved observations for this demo
		loadObservations();

		// Sidebar height management
		function setSidebarHeight() {
			if (!sidebarEl) return;
			const top = sidebarEl.getBoundingClientRect().top;
			sidebarEl.style.maxHeight = `${window.innerHeight - top - 24}px`;
		}
		setSidebarHeight();
		window.addEventListener('resize', setSidebarHeight);
		// Re-measure when sidebar content changes
		const observer = new ResizeObserver(setSidebarHeight);
		if (sidebarEl) observer.observe(sidebarEl);

		return () => {
			window.removeEventListener('resize', setSidebarHeight);
			observer.disconnect();
		};
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
	// Experiment save
	// ---------------------------------------------------------------------------

	async function saveExperiment() {
		if (!experimentHypothesis.trim()) return;
		experimentSaving = true;
		try {
			const snapshot: Record<string, number> = {};
			if (session.paramManifest?.params) {
				for (const p of session.paramManifest.params) {
					snapshot[p.key] = session.paramValues[p.key] ?? p.default;
				}
			}
			const res = await fetch(api('/experiments'), {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					parent_demo: demo.id,
					hypothesis: experimentHypothesis.trim(),
					expected_effect: experimentExpected.trim() || null,
					param_snapshot: snapshot,
				}),
			});
			if (res.ok) {
				experimentSaved = true;
				setTimeout(() => {
					experimentSaved = false;
					showExperimentForm = false;
					experimentHypothesis = '';
					experimentExpected = '';
				}, 2000);
			}
		} catch { /* ignore */ }
		experimentSaving = false;
	}

	// ---------------------------------------------------------------------------
	// Observations
	// ---------------------------------------------------------------------------

	async function loadObservations() {
		try {
			const res = await fetch(api(`/observations?demo_id=${demo.id}`));
			if (res.ok) savedObservations = await res.json();
		} catch { /* ignore */ }
	}

	async function saveObservation() {
		if (!obsClaim.trim()) return;
		obsSaving = true;
		try {
			const res = await fetch(api('/observations'), {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					claim: obsClaim.trim(),
					linked_entities: [{ id: demo.id, type: 'demo' }],
					confidence: obsConfidence,
					why_it_matters: obsWhy.trim() || null,
					follow_up_question: obsFollowUp.trim() || null,
				}),
			});
			if (res.ok) {
				obsSaved = true;
				await loadObservations();
				setTimeout(() => {
					obsSaved = false;
					showObsForm = false;
					obsClaim = '';
					obsWhy = '';
					obsFollowUp = '';
					obsConfidence = 'tentative';
				}, 2000);
			}
		} catch { /* ignore */ }
		obsSaving = false;
	}

	// ---------------------------------------------------------------------------
	// Bookmark capture
	// ---------------------------------------------------------------------------

	function confirmBookmark() {
		session.confirmBookmark(bookmarkLabel, bookmarkTags);
		bookmarkLabel = '';
	}

	async function fetchReadiness() {
		try {
			const res = await fetch(api(`/readiness/demo/${demo.id}`));
			if (res.ok) readiness = await res.json();
		} catch { /* ignore */ }
	}

	async function transitionState(newState: string) {
		publishLoading = true;
		try {
			const res = await fetch(api('/publish'), {
				method: 'PATCH',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ entity_type: 'demo', id: demo.id, new_state: newState }),
			});
			if (res.ok) {
				publishState = newState;
				await fetchReadiness();
			}
		} catch { /* ignore */ }
		publishLoading = false;
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
		<aside class="sidebar" bind:this={sidebarEl}>

			<!-- Publish state panel -->
			<div class="panel">
				<button
					class="panel-header"
					onclick={() => showPublish = !showPublish}
					aria-expanded={showPublish}
				>
					<span>
						<span class="publish-dot {publishState}"></span>
						Publish
					</span>
					<span class="caret">{showPublish ? '▴' : '▾'}</span>
				</button>
				{#if showPublish}
					<div class="panel-body">
						<div class="publish-state-row">
							<span class="publish-badge {publishState}">{publishState}</span>
							{#if publishState === 'draft'}
								<button class="publish-btn" disabled={publishLoading} onclick={() => transitionState('review')}>→ Review</button>
							{:else if publishState === 'review'}
								<button class="publish-btn" disabled={publishLoading || !(readiness?.ready)} onclick={() => transitionState('public')}>→ Public</button>
								<button class="publish-btn-secondary" disabled={publishLoading} onclick={() => transitionState('draft')}>← Draft</button>
							{:else}
								<button class="publish-btn-secondary" disabled={publishLoading} onclick={() => transitionState('review')}>← Review</button>
							{/if}
						</div>

						{#if readiness}
							<div class="readiness-list">
								{#each readiness.checks as check}
									<div class="readiness-item" class:ok={check.ok} class:fail={!check.ok}>
										<span class="readiness-icon">{check.ok ? '✓' : '✗'}</span>
										<span class="readiness-label">{check.field}</span>
										{#if !check.ok}
											<span class="readiness-msg">{check.message}</span>
										{/if}
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/if}
			</div>

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

							<!-- Save as experiment -->
							{#if !showExperimentForm}
								<button class="experiment-trigger" onclick={() => showExperimentForm = true}>
									🧪 Save as experiment
								</button>
							{:else}
								<div class="experiment-form">
									<h4 class="experiment-title">🧪 New experiment</h4>
									<input
										type="text"
										class="input"
										placeholder="What's your hypothesis?"
										bind:value={experimentHypothesis}
									/>
									<input
										type="text"
										class="input"
										placeholder="Expected effect (optional)"
										bind:value={experimentExpected}
									/>
									<div class="experiment-params-preview">
										<span class="preview-label">Snapshot:</span>
										{#each session.paramManifest.params as p}
											<span class="preview-item">{p.label} = {session.paramValues[p.key]?.toFixed(p.kind === 'float' ? 2 : 0) ?? p.default}</span>
										{/each}
									</div>
									<div class="experiment-actions">
										{#if experimentSaved}
											<span class="experiment-success">✓ Saved!</span>
										{:else}
											<button
												class="btn btn-primary btn-sm"
												disabled={experimentSaving || !experimentHypothesis.trim()}
												onclick={saveExperiment}
											>Save</button>
											<button
												class="btn btn-ghost btn-sm"
												onclick={() => showExperimentForm = false}
											>Cancel</button>
										{/if}
									</div>
								</div>
							{/if}
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

			<!-- Recent moments (non-blocking — auto-emitted by the game) -->
			{#if session.recentMoments.length}
				<div class="panel">
					<button
						class="panel-header"
						onclick={() => showBookmarks = !showBookmarks}
						aria-expanded={showBookmarks}
					>
						<span class="panel-title">
							📌 Moments
							<span class="panel-badge">{session.recentMoments.length}</span>
						</span>
						<span class="panel-toggle">{showBookmarks ? '▲' : '▼'}</span>
					</button>
					{#if showBookmarks}
						<div class="panel-body">
							{#each [...session.recentMoments].reverse() as m}
								<div class="bookmark-row">
									<span class="bookmark-frame">f{m.frame}</span>
									<div class="bookmark-tags">
										{#each m.auto_tags as tag}
											<span class="chip chip-sm">{tag}</span>
										{/each}
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/if}

			<!-- Explicit pending capture (user-triggered, shows pause overlay) -->
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

			<!-- Observations panel -->
			<div class="panel">
				<button
					class="panel-header"
					onclick={() => showObservations = !showObservations}
					aria-expanded={showObservations}
				>
					<span class="panel-title">
						📝 Observations
						{#if savedObservations.length}
							<span class="panel-badge">{savedObservations.length}</span>
						{/if}
					</span>
					<span class="panel-toggle">{showObservations ? '▲' : '▼'}</span>
				</button>
				{#if showObservations}
					<div class="panel-body">
						<!-- Saved observations -->
						{#each savedObservations as obs}
							<div class="obs-card">
								<div class="obs-header">
									<span class="obs-confidence {obs.confidence}">{obs.confidence}</span>
								</div>
								<p class="obs-claim">{obs.claim}</p>
								{#if obs.why_it_matters}
									<p class="obs-why">{obs.why_it_matters}</p>
								{/if}
								{#if obs.follow_up_question}
									<p class="obs-followup">❓ {obs.follow_up_question}</p>
								{/if}
							</div>
						{/each}

						<!-- New observation form -->
						{#if !showObsForm}
							<button class="experiment-trigger" onclick={() => showObsForm = true}>
								+ New observation
							</button>
						{:else}
							<div class="obs-form">
								<input
									type="text"
									class="input"
									placeholder="What did you observe?"
									bind:value={obsClaim}
								/>
								<div class="obs-confidence-row">
									<span class="obs-field-label">Confidence</span>
									<select class="obs-select" bind:value={obsConfidence}>
										<option value="speculative">Speculative</option>
										<option value="tentative">Tentative</option>
										<option value="supported">Supported</option>
										<option value="established">Established</option>
									</select>
								</div>
								<input
									type="text"
									class="input"
									placeholder="Why does this matter? (optional)"
									bind:value={obsWhy}
								/>
								<input
									type="text"
									class="input"
									placeholder="Follow-up question? (optional)"
									bind:value={obsFollowUp}
								/>
								<div class="experiment-actions">
									{#if obsSaved}
										<span class="experiment-success">✓ Saved!</span>
									{:else}
										<button
											class="btn btn-primary btn-sm"
											disabled={obsSaving || !obsClaim.trim()}
											onclick={saveObservation}
										>Save</button>
										<button
											class="btn btn-ghost btn-sm"
											onclick={() => showObsForm = false}
										>Cancel</button>
									{/if}
								</div>
							</div>
						{/if}
					</div>
				{/if}
			</div>

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
		grid-template-rows: 1fr;
		gap: var(--space-6);
		padding: var(--space-6);
		flex: 1;
		max-width: var(--content-max);
		width: 100%;
		margin: 0 auto;
		align-items: start;
		max-height: calc(100vh - var(--nav-height));
	}

	/* ---- Sidebar ---- */
	.sidebar {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		min-height: 0;
		max-height: calc(100vh - var(--nav-height) - var(--space-12));
		overflow-y: auto;
		padding-bottom: var(--space-6);
		scrollbar-width: thin;
		scrollbar-color: rgba(255,255,255,0.15) transparent;
	}
	.sidebar::-webkit-scrollbar { width: 4px; }
	.sidebar::-webkit-scrollbar-track { background: transparent; }
	.sidebar::-webkit-scrollbar-thumb {
		background: rgba(255,255,255,0.15);
		border-radius: 2px;
	}
	.sidebar::-webkit-scrollbar-thumb:hover { background: rgba(255,255,255,0.3); }

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
	.moment-tags, .bookmark-tags {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-1);
	}
	.chip-sm {
		font-size: 0.65rem;
		padding: 1px 5px;
		border-radius: 3px;
		background: var(--bg-subtle);
		color: var(--text-muted);
		border: 1px solid var(--border);
		text-transform: uppercase;
		letter-spacing: 0.03em;
	}
	.moment-actions {
		display: flex;
		gap: var(--space-2);
		flex-wrap: wrap;
	}

	/* Experiment form */
	.experiment-trigger {
		width: 100%;
		padding: var(--space-2) var(--space-3);
		background: rgba(255,255,255,0.03);
		border: 1px dashed var(--border);
		border-radius: var(--radius-md);
		color: var(--text-secondary);
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all var(--duration-fast) var(--ease);
		margin-top: var(--space-2);
	}
	.experiment-trigger:hover {
		border-color: var(--accent-dim);
		color: var(--accent);
		background: rgba(255,255,255,0.05);
	}
	.experiment-form {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		padding: var(--space-3);
		background: rgba(255,255,255,0.03);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		margin-top: var(--space-2);
	}
	.experiment-title {
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--text-primary);
	}
	.experiment-params-preview {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-1);
		font-size: 0.6875rem;
	}
	.preview-label {
		color: var(--text-muted);
		font-weight: 600;
		width: 100%;
		margin-bottom: 2px;
	}
	.preview-item {
		padding: 1px 5px;
		background: var(--bg-subtle);
		border: 1px solid var(--border);
		border-radius: 3px;
		color: var(--text-secondary);
		font-family: var(--font-mono);
	}
	.experiment-actions {
		display: flex;
		gap: var(--space-2);
		align-items: center;
	}
	.experiment-success {
		font-size: 0.8125rem;
		color: #22c55e;
		font-weight: 600;
	}
	.btn-sm {
		font-size: 0.75rem;
		padding: var(--space-1) var(--space-3);
	}

	/* Observations */
	.obs-card {
		padding: var(--space-3);
		background: rgba(255,255,255,0.03);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}
	.obs-header { display: flex; align-items: center; }
	.obs-confidence {
		font-size: 0.625rem;
		padding: 2px 6px;
		border-radius: 999px;
		text-transform: uppercase;
		letter-spacing: 0.04em;
		font-weight: 500;
		border: 1px solid;
	}
	.obs-confidence.speculative { border-color: #ef444440; color: #ef4444; }
	.obs-confidence.tentative   { border-color: #f59e0b40; color: #f59e0b; }
	.obs-confidence.supported   { border-color: #3b82f640; color: #3b82f6; }
	.obs-confidence.established { border-color: #22c55e40; color: #22c55e; }
	.obs-claim {
		font-size: 0.8125rem;
		color: var(--text-primary);
		line-height: 1.5;
	}
	.obs-why {
		font-size: 0.75rem;
		color: var(--text-secondary);
		font-style: italic;
	}
	.obs-followup {
		font-size: 0.75rem;
		color: var(--accent);
	}
	.obs-form {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		padding: var(--space-3);
		background: rgba(255,255,255,0.03);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
	}
	.obs-confidence-row {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}
	.obs-field-label {
		font-size: 0.75rem;
		color: var(--text-muted);
		font-weight: 500;
	}
	.obs-select {
		flex: 1;
		background: var(--bg-subtle);
		border: 1px solid var(--border);
		border-radius: var(--radius-sm);
		color: var(--text-primary);
		font-size: 0.75rem;
		padding: var(--space-1) var(--space-2);
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

	/* Publish state */
	.publish-dot {
		display: inline-block;
		width: 8px; height: 8px;
		border-radius: 50%;
		margin-right: var(--space-1);
	}
	.publish-dot.draft  { background: #ef4444; }
	.publish-dot.review { background: #f59e0b; }
	.publish-dot.public { background: #22c55e; }

	.publish-state-row {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		flex-wrap: wrap;
	}
	.publish-badge {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.04em;
		padding: 2px 8px;
		border-radius: 999px;
	}
	.publish-badge.draft  { color: #ef4444; background: rgba(239, 68, 68, 0.15); }
	.publish-badge.review { color: #f59e0b; background: rgba(245, 158, 11, 0.15); }
	.publish-badge.public { color: #22c55e; background: rgba(34, 197, 94, 0.15); }

	.publish-btn, .publish-btn-secondary {
		font-size: 0.6875rem;
		font-weight: 500;
		padding: 3px 10px;
		border-radius: var(--radius-sm);
		border: none;
		cursor: pointer;
		font-family: var(--font-sans);
		transition: opacity 0.15s;
	}
	.publish-btn { background: var(--accent); color: #111; }
	.publish-btn-secondary { background: rgba(255,255,255,0.08); color: #9391a8; }
	.publish-btn:disabled, .publish-btn-secondary:disabled { opacity: 0.4; cursor: not-allowed; }

	.readiness-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
		margin-top: var(--space-3);
	}
	.readiness-item {
		display: flex;
		align-items: baseline;
		gap: var(--space-2);
		font-size: 0.75rem;
	}
	.readiness-icon { font-weight: 700; flex-shrink: 0; }
	.readiness-item.ok .readiness-icon { color: #22c55e; }
	.readiness-item.fail .readiness-icon { color: #ef4444; }
	.readiness-label { color: #9391a8; }
	.readiness-msg { color: #ef4444; font-size: 0.6875rem; }

	/* Clickable entity chips */
	.chip-link {
		text-decoration: none;
		cursor: pointer;
		transition: background var(--duration-fast) var(--ease),
		            transform var(--duration-fast) var(--ease);
	}
	.chip-link:hover {
		background: var(--teal-muted);
		transform: translateY(-1px);
	}

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
