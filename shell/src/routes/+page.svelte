<script lang="ts">
	import { fly } from 'svelte/transition';

	// Static placeholder data — wired to API in Phase 1
	const families = [
		{ slug: 'movement',         label: 'Movement',            icon: '↑' },
		{ slug: 'collision-response', label: 'Collision',          icon: '◈' },
		{ slug: 'scoring-pressure', label: 'Scoring & Pressure',  icon: '▲' },
		{ slug: 'state-transitions', label: 'State Transitions',  icon: '⬡' },
		{ slug: 'economy',          label: 'Economy',             icon: '◎' },
		{ slug: 'timing-windows',   label: 'Timing & Rhythm',     icon: '◷' },
		{ slug: 'spatial-rules',    label: 'Spatial Rules',       icon: '⬛' },
		{ slug: 'ai-behavior',      label: 'AI Behavior',         icon: '◉' },
		{ slug: 'progression',      label: 'Progression',         icon: '▶' },
		{ slug: 'information',      label: 'Information',         icon: '◌' },
	];

	const demos = [
		{
			id: 'pong-76',
			title: 'Pong',
			era: '1972',
			platform: 'Arcade',
			mechanic_tags: ['collision-response', 'scoring-pressure'],
			fidelity: 'faithful',
			description: 'Ball and paddle — the mechanic that started everything. No friction, no gravity, just deflection angle and speed.',
		},
		{
			id: 'maze-80',
			title: 'Maze Chase',
			era: '1980',
			platform: 'Arcade',
			mechanic_tags: ['ai-behavior', 'state-transitions'],
			fidelity: 'interpreted',
			description: 'Four distinct AI patterns operating simultaneously. Each ghost uses a different spatial algorithm to create combined pressure.',
		},
		{
			id: 'jump-feel',
			title: 'Jump Arc Study',
			era: '1985',
			platform: 'NES',
			mechanic_tags: ['movement', 'timing-windows'],
			fidelity: 'experimental',
			description: 'Variable jump height controlled by hold duration. The feel that defined a decade of platformers — exposed and tunable.',
		},
	];

	let activeFamily: string | null = $state(null);

	const filteredDemos = $derived(
		activeFamily
			? demos.filter(d => d.mechanic_tags.includes(activeFamily!))
			: demos
	);

	function toggleFamily(slug: string) {
		activeFamily = activeFamily === slug ? null : slug;
	}
</script>

<svelte:head>
	<title>Discover — Ludeme</title>
</svelte:head>

<!-- ===================== Hero ===================== -->
<section class="hero">
	<div class="container">
		<div class="hero-inner">
			<div class="hero-text animate-fade-up">
				<p class="hero-eyebrow">Playable Mechanics Atlas</p>
				<h1 class="hero-headline">
					The atomic units<br />of game design,<br />
					<em>made playable.</em>
				</h1>
				<p class="hero-body">
					Micro-demos. Mechanic research. Lineage maps. Every surface connects play to understanding.
				</p>
				<div class="hero-actions">
					<a href="/mechanics" class="btn btn-primary">Browse mechanics</a>
					<a href="/collections" class="btn btn-ghost">Start a trail →</a>
				</div>
			</div>

			<div class="hero-gfx" aria-hidden="true">
				<div class="gfx-ring ring-1"></div>
				<div class="gfx-ring ring-2"></div>
				<div class="gfx-ring ring-3"></div>
				<div class="gfx-icon">⬡</div>
			</div>
		</div>
	</div>
</section>

<!-- ===================== Filter by Family ===================== -->
<section class="section-families">
	<div class="container">
		<div class="section-header">
			<h2>Mechanic families</h2>
			<p>Filter demos by the type of rule or feel they demonstrate.</p>
			{#if activeFamily}
				<button class="clear-filter" onclick={() => activeFamily = null}>
					Clear filter ×
				</button>
			{/if}
		</div>
		<div class="family-grid" role="toolbar" aria-label="Filter by mechanic family">
			{#each families as f}
				<button
					class="family-chip"
					class:active={activeFamily === f.slug}
					onclick={() => toggleFamily(f.slug)}
					aria-pressed={activeFamily === f.slug}
				>
					<span class="family-icon">{f.icon}</span>
					<span class="family-label">{f.label}</span>
				</button>
			{/each}
		</div>
	</div>
</section>

<!-- ===================== Demo Cards ===================== -->
<section class="section-demos">
	<div class="container">
		<div class="section-header">
			<h2>
				{activeFamily
					? `${families.find(f => f.slug === activeFamily)?.label} demos`
					: 'Featured demos'}
			</h2>
			<span class="count">{filteredDemos.length} demo{filteredDemos.length !== 1 ? 's' : ''}</span>
		</div>

		{#if filteredDemos.length === 0}
			<div class="empty-state">
				<p>No demos yet for this mechanic family.</p>
				<p class="empty-sub">They'll appear here as content is added.</p>
			</div>
		{:else}
			<div class="demo-grid">
				{#each filteredDemos as demo, i (demo.id)}
					<a
						href="/demo/{demo.id}"
						class="demo-card card"
						style="animation-delay: {i * 60}ms"
						in:fly={{ y: 16, duration: 320, delay: i * 60 }}
					>
						<div class="demo-canvas-placeholder" aria-hidden="true">
							<span class="demo-play-icon">▶</span>
						</div>
						<div class="demo-body">
							<div class="demo-meta">
								<span class="chip">{demo.era}</span>
								<span class="chip neutral">{demo.platform}</span>
								<span class="badge {demo.fidelity}">{demo.fidelity}</span>
							</div>
							<h3 class="demo-title">{demo.title}</h3>
							<p class="demo-desc">{demo.description}</p>
							<div class="demo-tags">
								{#each demo.mechanic_tags as tag}
									<span class="chip teal">{tag.replace('-', ' ')}</span>
								{/each}
							</div>
						</div>
					</a>
				{/each}
			</div>
		{/if}
	</div>
</section>

<style>
	/* ---- Hero ---- */
	.hero {
		padding: var(--space-16) 0 var(--space-12);
		position: relative;
		overflow: hidden;
	}
	.hero::before {
		content: '';
		position: absolute;
		inset: 0;
		background: radial-gradient(ellipse 60% 50% at 70% 50%, rgba(245, 158, 11, 0.06) 0%, transparent 70%);
		pointer-events: none;
	}

	.hero-inner {
		display: grid;
		grid-template-columns: 1fr 380px;
		gap: var(--space-16);
		align-items: center;
	}

	.hero-eyebrow {
		font-size: 0.75rem;
		font-weight: 700;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--accent);
		margin-bottom: var(--space-4);
	}

	.hero-headline {
		font-size: clamp(2rem, 4vw, 3rem);
		font-weight: 700;
		line-height: 1.1;
		letter-spacing: -0.03em;
		margin-bottom: var(--space-5);
	}
	.hero-headline em {
		font-style: normal;
		color: var(--accent);
	}

	.hero-body {
		font-size: 1.0625rem;
		color: var(--text-secondary);
		max-width: 480px;
		margin-bottom: var(--space-8);
		line-height: 1.7;
	}

	.hero-actions {
		display: flex;
		gap: var(--space-3);
		flex-wrap: wrap;
	}

	/* Graphic */
	.hero-gfx {
		position: relative;
		height: 300px;
		display: flex;
		align-items: center;
		justify-content: center;
	}
	.gfx-ring {
		position: absolute;
		border-radius: 50%;
		border: 1px solid var(--border);
	}
	.ring-1 { width: 120px; height: 120px; border-color: rgba(245,158,11,0.2); animation: spin 20s linear infinite; }
	.ring-2 { width: 220px; height: 220px; border-color: rgba(245,158,11,0.1); animation: spin 35s linear infinite reverse; }
	.ring-3 { width: 300px; height: 300px; border-color: rgba(255,255,255,0.05); }
	.gfx-icon {
		font-size: 3rem;
		color: var(--accent);
		position: relative;
		z-index: 1;
		filter: drop-shadow(0 0 24px rgba(245,158,11,0.4));
	}
	@keyframes spin { to { transform: rotate(360deg); } }

	/* ---- Section common ---- */
	.section-families,
	.section-demos {
		padding: var(--space-10) 0;
	}
	.section-header {
		display: flex;
		align-items: baseline;
		gap: var(--space-4);
		margin-bottom: var(--space-6);
		flex-wrap: wrap;
	}
	.section-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
	}
	.section-header p {
		font-size: 0.875rem;
		color: var(--text-muted);
	}
	.count {
		font-size: 0.8125rem;
		color: var(--text-muted);
		margin-left: auto;
	}
	.clear-filter {
		background: none;
		border: none;
		color: var(--accent);
		font-size: 0.8125rem;
		cursor: pointer;
		padding: 0;
		margin-left: auto;
	}
	.clear-filter:hover { text-decoration: underline; }

	/* ---- Family chips ---- */
	.family-grid {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
	}
	.family-chip {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-4);
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		color: var(--text-secondary);
		font-family: var(--font-sans);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all var(--duration-fast) var(--ease);
	}
	.family-chip:hover {
		border-color: var(--border-hover);
		color: var(--text-primary);
		background: var(--bg-overlay);
	}
	.family-chip.active {
		background: var(--accent-muted);
		border-color: var(--border-accent);
		color: var(--accent);
	}
	.family-icon {
		font-size: 1rem;
		line-height: 1;
		opacity: 0.7;
	}
	.family-chip.active .family-icon { opacity: 1; }

	/* ---- Demo grid ---- */
	.demo-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
		gap: var(--space-5);
	}

	.demo-card {
		display: flex;
		flex-direction: column;
		text-decoration: none;
		color: inherit;
	}

	.demo-canvas-placeholder {
		height: 180px;
		background: var(--bg-subtle);
		border-radius: var(--radius-lg) var(--radius-lg) 0 0;
		display: flex;
		align-items: center;
		justify-content: center;
		border-bottom: 1px solid var(--border);
		transition: background var(--duration) var(--ease);
		position: relative;
		overflow: hidden;
	}
	.demo-canvas-placeholder::before {
		content: '';
		position: absolute;
		inset: 0;
		background: radial-gradient(circle at center, rgba(245,158,11,0.05) 0%, transparent 70%);
		opacity: 0;
		transition: opacity var(--duration) var(--ease);
	}
	.demo-card:hover .demo-canvas-placeholder::before { opacity: 1; }

	.demo-play-icon {
		font-size: 2rem;
		color: var(--text-muted);
		transition: color var(--duration) var(--ease), transform var(--duration) var(--ease);
	}
	.demo-card:hover .demo-play-icon {
		color: var(--accent);
		transform: scale(1.15);
	}

	.demo-body {
		padding: var(--space-5);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		flex: 1;
	}

	.demo-meta {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		flex-wrap: wrap;
	}

	.demo-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary);
	}

	.demo-desc {
		font-size: 0.875rem;
		color: var(--text-secondary);
		line-height: 1.6;
		flex: 1;
	}

	.demo-tags {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-1);
		margin-top: var(--space-1);
	}

	/* ---- Empty state ---- */
	.empty-state {
		text-align: center;
		padding: var(--space-16) 0;
		color: var(--text-muted);
	}
	.empty-sub {
		margin-top: var(--space-2);
		font-size: 0.875rem;
	}

	/* ---- Responsive ---- */
	@media (max-width: 768px) {
		.hero-inner {
			grid-template-columns: 1fr;
		}
		.hero-gfx { display: none; }
		.demo-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
