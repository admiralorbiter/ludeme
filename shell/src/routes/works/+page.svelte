<script lang="ts">
	import type { PageData } from './$types.js';

	let { data }: { data: PageData } = $props();
	const { works } = data;

	// Group by genre
	const grouped = $derived(() => {
		const groups: Record<string, any[]> = {};
		for (const w of works) {
			const genre = w.genre || 'Other';
			if (!groups[genre]) groups[genre] = [];
			groups[genre].push(w);
		}
		// Sort each group by year
		for (const g of Object.values(groups)) {
			g.sort((a: any, b: any) => (a.year || 0) - (b.year || 0));
		}
		return groups;
	});
</script>

<svelte:head>
	<title>Works — Ludeme</title>
	<meta name="description" content="Historical games and design artifacts that shaped the medium." />
</svelte:head>

<div class="works-page container">
	<header class="page-header">
		<h1>Works</h1>
		<p class="subtitle">
			Historical games and design artifacts. Each work is the context that gave rise to mechanics worth studying.
		</p>
	</header>

	{#if works.length === 0}
		<div class="empty-state">
			<p>No works found.</p>
		</div>
	{:else}
		{#each Object.entries(grouped()) as [genre, genreWorks]}
			<section class="genre-section">
				<h2 class="genre-heading">{genre}</h2>
				<div class="work-grid">
					{#each genreWorks as w}
						<a href="/works/{w.id}" class="work-card">
							<div class="card-top">
								<span class="work-year">{w.year || '—'}</span>
								<span class="work-platform">{w.platform || ''}</span>
							</div>
							<h3 class="card-title">{w.title}</h3>
							{#if w.significance}
								<p class="card-sig">{w.significance}</p>
							{/if}
							{#if w.publish_state}
								<span class="publish-chip">{w.publish_state}</span>
							{/if}
						</a>
					{/each}
				</div>
			</section>
		{/each}
	{/if}
</div>

<style>
	.works-page { padding-top: var(--space-10); }

	.page-header { margin-bottom: var(--space-8); }
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
	}

	/* Genre sections */
	.genre-section { margin-bottom: var(--space-10); }
	.genre-heading {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--accent);
		letter-spacing: 0.01em;
		margin-bottom: var(--space-4);
		padding-bottom: var(--space-2);
		border-bottom: 1px solid var(--border);
	}

	/* Work cards */
	.work-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: var(--space-5);
	}
	.work-card {
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
	.work-card:hover {
		border-color: var(--accent-dim);
		box-shadow: 0 4px 24px rgba(0, 0, 0, 0.3);
		transform: translateY(-2px);
	}
	.card-top {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.work-year {
		font-size: 0.875rem;
		color: var(--accent);
		font-weight: 600;
		font-family: 'JetBrains Mono', monospace;
	}
	.work-platform {
		font-size: 0.6875rem;
		color: #5a586e;
		text-transform: uppercase;
		letter-spacing: 0.04em;
	}
	.card-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary);
	}
	.card-sig {
		font-size: 0.8125rem;
		color: var(--text-secondary);
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
	.publish-chip {
		align-self: flex-start;
		font-size: 0.625rem;
		padding: 2px 6px;
		border-radius: 4px;
		text-transform: uppercase;
		letter-spacing: 0.04em;
		background: rgba(255,255,255,0.05);
		color: #5a586e;
		margin-top: auto;
	}
</style>
