<script lang="ts">
	let { data } = $props();
	const { collections } = data;
</script>

<svelte:head>
	<title>Collections — Ludeme</title>
	<meta name="description" content="Curated learning trails through the mechanics of play." />
</svelte:head>

<div class="collections-page container">
	<header class="page-header">
		<h1>Collections</h1>
		<p class="subtitle">Curated learning trails that connect demos, mechanics, and history into a guided journey.</p>
	</header>

	{#if collections.length === 0}
		<div class="empty-state">
			<p>No collections yet. Check back soon.</p>
		</div>
	{:else}
		<div class="collection-grid">
			{#each collections as c}
				<a href="/collections/{c.id}" class="collection-card">
					<div class="card-icon">📚</div>
					<div class="card-body">
						<h2 class="card-title">{c.title}</h2>
						{#if c.learning_goal}
							<p class="card-goal">{c.learning_goal}</p>
						{/if}
						<div class="card-meta">
							<span class="item-count">
								{JSON.parse(c.ordered_items).length} demo{JSON.parse(c.ordered_items).length !== 1 ? 's' : ''}
							</span>
							<span class="publish-state">{c.publish_state}</span>
						</div>
					</div>
				</a>
			{/each}
		</div>
	{/if}
</div>

<style>
	.collections-page { padding-top: var(--space-10); }

	.page-header {
		margin-bottom: var(--space-10);
	}
	h1 {
		font-size: 2rem;
		font-weight: 700;
		letter-spacing: -0.03em;
	}
	.subtitle {
		color: #9391a8;
		margin-top: var(--space-2);
		font-size: 1.0625rem;
		line-height: 1.6;
	}

	.empty-state {
		text-align: center;
		color: #5a586e;
		padding: var(--space-16) 0;
		font-size: 1.0625rem;
	}

	.collection-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
		gap: var(--space-6);
	}

	.collection-card {
		display: flex;
		gap: var(--space-4);
		padding: var(--space-6);
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		text-decoration: none;
		transition: border-color var(--duration-fast) var(--ease),
		            box-shadow var(--duration-fast) var(--ease),
		            transform var(--duration-fast) var(--ease);
	}
	.collection-card:hover {
		border-color: var(--accent-dim);
		box-shadow: 0 4px 24px rgba(0, 0, 0, 0.3);
		transform: translateY(-2px);
	}

	.card-icon {
		font-size: 1.75rem;
		flex-shrink: 0;
		padding-top: var(--space-1);
	}

	.card-body {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		min-width: 0;
	}

	.card-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f0f7;
	}

	.card-goal {
		font-size: 0.875rem;
		color: #9391a8;
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.card-meta {
		display: flex;
		gap: var(--space-3);
		margin-top: var(--space-1);
	}
	.item-count {
		font-size: 0.75rem;
		color: var(--accent);
		font-weight: 500;
	}
	.publish-state {
		font-size: 0.6875rem;
		color: #5a586e;
		text-transform: uppercase;
		letter-spacing: 0.04em;
	}
</style>
