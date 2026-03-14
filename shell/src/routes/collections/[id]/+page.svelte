<script lang="ts">
	let { data } = $props();
	const { collection } = data;
</script>

<svelte:head>
	<title>{collection.title} — Ludeme</title>
	<meta name="description" content={collection.learning_goal ?? `Explore the ${collection.title} collection.`} />
</svelte:head>

<div class="collection-detail container">
	<!-- Breadcrumb -->
	<nav class="breadcrumb" aria-label="Breadcrumbs">
		<a href="/collections">Collections</a>
		<span class="sep">›</span>
		<span>{collection.title}</span>
	</nav>

	<header class="detail-header">
		<h1>{collection.title}</h1>
		{#if collection.learning_goal}
			<p class="learning-goal">{collection.learning_goal}</p>
		{/if}
	</header>

	<!-- Trail items -->
	<section class="trail">
		<h2 class="trail-heading">Trail <span class="trail-count">{collection.items.length}</span></h2>

		{#if collection.items.length === 0}
			<p class="empty">No demos in this trail yet.</p>
		{:else}
			<ol class="trail-list">
				{#each collection.items as demo, i}
					<li class="trail-item">
						<div class="step-number">{i + 1}</div>
						<a href="/demo/{demo.id}" class="trail-card">
							<div class="trail-card-body">
								<h3 class="trail-card-title">{demo.title}</h3>
								{#if demo.description}
									<p class="trail-card-desc">{demo.description}</p>
								{/if}
								<div class="trail-card-tags">
									{#each JSON.parse(demo.mechanic_tags || '[]') as tag}
										<span class="tag">{tag}</span>
									{/each}
									<span class="fidelity">● {demo.fidelity_level}</span>
								</div>
							</div>
							<span class="trail-card-arrow">→</span>
						</a>
					</li>
				{/each}
			</ol>
		{/if}
	</section>
</div>

<style>
	.collection-detail { padding-top: var(--space-6); max-width: 720px; }

	/* Breadcrumb */
	.breadcrumb {
		font-size: 0.8125rem;
		color: #5a586e;
		margin-bottom: var(--space-6);
	}
	.breadcrumb a {
		color: #9391a8;
		text-decoration: none;
	}
	.breadcrumb a:hover { color: #f1f0f7; }
	.sep { margin: 0 var(--space-2); }

	/* Header */
	.detail-header { margin-bottom: var(--space-8); }
	h1 {
		font-size: 2rem;
		font-weight: 700;
		letter-spacing: -0.03em;
	}
	.learning-goal {
		color: #9391a8;
		margin-top: var(--space-2);
		font-size: 1.0625rem;
		line-height: 1.6;
	}

	/* Trail */
	.trail-heading {
		font-size: 1rem;
		font-weight: 600;
		margin-bottom: var(--space-4);
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}
	.trail-count {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--accent);
		background: var(--accent-muted);
		border-radius: 999px;
		padding: 2px 8px;
	}

	.trail-list {
		list-style: none;
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.trail-item {
		display: flex;
		gap: var(--space-4);
		align-items: flex-start;
	}

	.step-number {
		flex-shrink: 0;
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.8125rem;
		font-weight: 700;
		color: var(--accent);
		background: var(--accent-muted);
		border-radius: 50%;
		margin-top: var(--space-3);
	}

	.trail-card {
		flex: 1;
		display: flex;
		align-items: center;
		gap: var(--space-4);
		padding: var(--space-4) var(--space-5);
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		text-decoration: none;
		transition: border-color var(--duration-fast) var(--ease),
		            transform var(--duration-fast) var(--ease);
	}
	.trail-card:hover {
		border-color: var(--accent-dim);
		transform: translateX(4px);
	}

	.trail-card-body {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.trail-card-title {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f0f7;
	}

	.trail-card-desc {
		font-size: 0.8125rem;
		color: #9391a8;
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.trail-card-tags {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
		margin-top: var(--space-1);
	}
	.tag {
		font-size: 0.6875rem;
		color: var(--accent);
		border: 1px solid var(--accent-dim);
		border-radius: 999px;
		padding: 1px 8px;
		text-transform: uppercase;
		letter-spacing: 0.04em;
	}
	.fidelity {
		font-size: 0.6875rem;
		color: #5a586e;
	}

	.trail-card-arrow {
		font-size: 1.25rem;
		color: #5a586e;
		flex-shrink: 0;
	}
	.trail-card:hover .trail-card-arrow { color: var(--accent); }

	.empty { color: #5a586e; }
</style>
