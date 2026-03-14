<script lang="ts">
	import type { PageData } from './$types.js';

	let { data }: { data: PageData } = $props();
	const { mechanic, linkedDemos } = data;

	const verbs: string[] = mechanic.verbs ? JSON.parse(mechanic.verbs) : [];
	const familyLabel = (mechanic.family || '').replace(/-/g, ' ');
</script>

<svelte:head>
	<title>{mechanic.name} — Ludeme</title>
	<meta name="description" content={mechanic.short_definition} />
</svelte:head>

<div class="detail-page container">
	<!-- Breadcrumb -->
	<nav class="breadcrumb" aria-label="Breadcrumb">
		<a href="/mechanics">Mechanics</a>
		<span class="sep">›</span>
		<span class="current">{mechanic.name}</span>
	</nav>

	<!-- Header -->
	<header class="mech-header">
		<div class="header-meta">
			<span class="family-badge">{mechanic.family}</span>
			{#if mechanic.publish_state}
				<span class="publish-badge" class:public={mechanic.publish_state === 'public'}>
					{mechanic.publish_state}
				</span>
			{/if}
		</div>
		<h1>{mechanic.name}</h1>
		<p class="definition">{mechanic.short_definition}</p>
	</header>

	<div class="content-grid">
		<!-- Left: main content -->
		<main class="main-content">
			<!-- Verbs -->
			{#if verbs.length > 0}
				<section class="section">
					<h2 class="section-title">Core verbs</h2>
					<div class="verb-list">
						{#each verbs as verb}
							<span class="verb-chip">{verb}</span>
						{/each}
					</div>
				</section>
			{/if}

			<!-- Failure & Mastery -->
			<div class="pattern-grid">
				{#if mechanic.failure_pattern}
					<section class="pattern-card failure">
						<h3 class="pattern-label">
							<span class="pattern-icon">✕</span> Failure pattern
						</h3>
						<p>{mechanic.failure_pattern}</p>
					</section>
				{/if}
				{#if mechanic.mastery_pattern}
					<section class="pattern-card mastery">
						<h3 class="pattern-label">
							<span class="pattern-icon">✓</span> Mastery pattern
						</h3>
						<p>{mechanic.mastery_pattern}</p>
					</section>
				{/if}
			</div>

			<!-- Linked Demos -->
			{#if linkedDemos.length > 0}
				<section class="section">
					<h2 class="section-title">Demonstrated in</h2>
					<div class="demo-grid">
						{#each linkedDemos as demo}
							<a href="/demo/{demo.id}" class="demo-card">
								<div class="demo-card-top">
									<span class="demo-era">{demo.era || '—'}</span>
									<span class="demo-platform">{demo.platform || ''}</span>
								</div>
								<h3 class="demo-title">{demo.title}</h3>
								{#if demo.description}
									<p class="demo-desc">{demo.description}</p>
								{/if}
								{#if demo.mechanic_tags}
									<div class="demo-tags">
										{#each JSON.parse(demo.mechanic_tags) as tag}
											<span class="tag-chip" class:highlight={tag === mechanic.family}>{tag}</span>
										{/each}
									</div>
								{/if}
							</a>
						{/each}
					</div>
				</section>
			{/if}
		</main>

		<!-- Right sidebar -->
		<aside class="sidebar">
			<div class="sidebar-card">
				<h3 class="sidebar-heading">Family</h3>
				<p class="sidebar-value family-name">{familyLabel}</p>
			</div>
			{#if mechanic.era}
				<div class="sidebar-card">
					<h3 class="sidebar-heading">Era</h3>
					<p class="sidebar-value">{mechanic.era}</p>
				</div>
			{/if}
			<div class="sidebar-card">
				<h3 class="sidebar-heading">Linked demos</h3>
				<p class="sidebar-value count">{linkedDemos.length}</p>
			</div>
		</aside>
	</div>
</div>

<style>
	.detail-page { padding-top: var(--space-6); padding-bottom: var(--space-16); }

	/* Breadcrumb */
	.breadcrumb {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: 0.8125rem;
		color: #5a586e;
		margin-bottom: var(--space-6);
	}
	.breadcrumb a {
		color: var(--text-secondary);
		text-decoration: none;
		transition: color var(--duration-fast);
	}
	.breadcrumb a:hover { color: var(--accent); }
	.breadcrumb .sep { color: #3a384e; }
	.breadcrumb .current { color: var(--text-primary); }

	/* Header */
	.mech-header { margin-bottom: var(--space-8); }
	.header-meta {
		display: flex;
		gap: var(--space-3);
		margin-bottom: var(--space-3);
	}
	.family-badge {
		font-size: 0.6875rem;
		padding: 3px 10px;
		border-radius: 999px;
		border: 1px solid var(--accent-dim);
		color: var(--accent);
		text-transform: uppercase;
		letter-spacing: 0.04em;
		font-weight: 500;
	}
	.publish-badge {
		font-size: 0.6875rem;
		padding: 3px 10px;
		border-radius: 999px;
		border: 1px solid #3a384e;
		color: #5a586e;
		text-transform: uppercase;
		letter-spacing: 0.04em;
	}
	.publish-badge.public {
		border-color: #22c55e40;
		color: #22c55e;
	}
	h1 {
		font-size: 2.25rem;
		font-weight: 700;
		letter-spacing: -0.03em;
		line-height: 1.2;
	}
	.definition {
		margin-top: var(--space-3);
		font-size: 1.125rem;
		color: var(--text-secondary);
		line-height: 1.7;
		max-width: 640px;
	}

	/* Content grid */
	.content-grid {
		display: grid;
		grid-template-columns: 1fr 260px;
		gap: var(--space-8);
		align-items: start;
	}
	@media (max-width: 768px) {
		.content-grid { grid-template-columns: 1fr; }
	}

	/* Sections */
	.section { margin-bottom: var(--space-8); }
	.section-title {
		font-size: 0.875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.06em;
		color: #5a586e;
		margin-bottom: var(--space-4);
	}

	/* Verbs */
	.verb-list {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
	}
	.verb-chip {
		font-size: 0.875rem;
		padding: var(--space-2) var(--space-4);
		border-radius: var(--radius-md);
		background: rgba(255,255,255,0.06);
		color: var(--text-primary);
		font-family: 'JetBrains Mono', monospace;
		border: 1px solid rgba(255,255,255,0.08);
	}

	/* Pattern cards */
	.pattern-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: var(--space-4);
		margin-bottom: var(--space-8);
	}
	@media (max-width: 640px) {
		.pattern-grid { grid-template-columns: 1fr; }
	}
	.pattern-card {
		padding: var(--space-5);
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
	}
	.pattern-card.failure { border-left: 3px solid #ef4444; }
	.pattern-card.mastery { border-left: 3px solid #22c55e; }
	.pattern-label {
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.06em;
		margin-bottom: var(--space-3);
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}
	.failure .pattern-label { color: #ef4444; }
	.mastery .pattern-label { color: #22c55e; }
	.pattern-icon { font-size: 0.875rem; }
	.pattern-card p {
		font-size: 0.875rem;
		color: var(--text-secondary);
		line-height: 1.6;
	}

	/* Demo cards */
	.demo-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: var(--space-4);
	}
	.demo-card {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		padding: var(--space-5);
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		text-decoration: none;
		transition: border-color var(--duration-fast) var(--ease),
		            box-shadow var(--duration-fast) var(--ease),
		            transform var(--duration-fast) var(--ease);
	}
	.demo-card:hover {
		border-color: var(--accent-dim);
		box-shadow: 0 4px 24px rgba(0, 0, 0, 0.3);
		transform: translateY(-2px);
	}
	.demo-card-top {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.demo-era {
		font-size: 0.75rem;
		color: var(--accent);
		font-weight: 600;
		font-family: 'JetBrains Mono', monospace;
	}
	.demo-platform {
		font-size: 0.6875rem;
		color: #5a586e;
		text-transform: uppercase;
		letter-spacing: 0.04em;
	}
	.demo-title {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary);
	}
	.demo-desc {
		font-size: 0.8125rem;
		color: var(--text-secondary);
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
	.demo-tags {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-1);
		margin-top: auto;
	}
	.tag-chip {
		font-size: 0.625rem;
		padding: 2px 6px;
		border-radius: 4px;
		background: rgba(255,255,255,0.05);
		color: #5a586e;
		text-transform: uppercase;
		letter-spacing: 0.04em;
	}
	.tag-chip.highlight {
		background: var(--accent-dim);
		color: var(--accent);
	}

	/* Sidebar */
	.sidebar {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		position: sticky;
		top: var(--space-6);
	}
	.sidebar-card {
		padding: var(--space-4);
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
	}
	.sidebar-heading {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.06em;
		color: #5a586e;
		margin-bottom: var(--space-2);
	}
	.sidebar-value {
		font-size: 1rem;
		color: var(--text-primary);
	}
	.sidebar-value.family-name { text-transform: capitalize; }
	.sidebar-value.count {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--accent);
	}
</style>
