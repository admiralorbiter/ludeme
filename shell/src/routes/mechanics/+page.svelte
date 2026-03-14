<script lang="ts">
	import type { PageData } from './$types.js';

	let { data }: { data: PageData } = $props();
	const { mechanics, families } = data;

	let activeFamily: string | null = $state(null);

	const grouped = $derived(() => {
		const filtered = activeFamily
			? mechanics.filter((m: any) => m.family === activeFamily)
			: mechanics;

		const groups: Record<string, any[]> = {};
		for (const m of filtered) {
			const fam = m.family || 'uncategorized';
			if (!groups[fam]) groups[fam] = [];
			groups[fam].push(m);
		}
		return groups;
	});

	function familyLabel(slug: string): string {
		const f = families.find((fam: any) => fam.slug === slug);
		return f?.label || slug.replace(/-/g, ' ');
	}

	function toggleFamily(slug: string) {
		activeFamily = activeFamily === slug ? null : slug;
	}
</script>

<svelte:head>
	<title>Mechanics — Ludeme</title>
	<meta name="description" content="Browse the taxonomy of game mechanics — from collision response to AI behavior." />
</svelte:head>

<div class="mechanics-page container">
	<header class="page-header">
		<h1>Mechanics</h1>
		<p class="subtitle">
			The atomic units of game design. Each mechanic is a single behaviour loop that shapes how a game feels.
		</p>
	</header>

	<!-- Family filter pills -->
	<div class="family-filters">
		<button
			class="filter-pill"
			class:active={activeFamily === null}
			onclick={() => (activeFamily = null)}
		>All</button>
		{#each families as fam}
			<button
				class="filter-pill"
				class:active={activeFamily === fam.slug}
				onclick={() => toggleFamily(fam.slug)}
			>{fam.label}</button>
		{/each}
	</div>

	{#if mechanics.length === 0}
		<div class="empty-state">
			<p>No mechanics found.</p>
		</div>
	{:else}
		{#each Object.entries(grouped()) as [family, mechs]}
			<section class="family-section">
				<h2 class="family-heading">{familyLabel(family)}</h2>
				<div class="mechanic-grid">
					{#each mechs as m}
						<a href="/mechanics/{m.id}" class="mechanic-card">
							<div class="card-header">
								<span class="family-badge">{family}</span>
							</div>
							<h3 class="card-title">{m.name}</h3>
							<p class="card-def">{m.short_definition}</p>
							{#if m.verbs}
								<div class="card-verbs">
									{#each JSON.parse(m.verbs) as verb}
										<span class="verb-chip">{verb}</span>
									{/each}
								</div>
							{/if}
						</a>
					{/each}
				</div>
			</section>
		{/each}
	{/if}
</div>

<style>
	.mechanics-page { padding-top: var(--space-10); }

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

	/* Family filter pills */
	.family-filters {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
		margin-bottom: var(--space-8);
	}
	.filter-pill {
		padding: var(--space-2) var(--space-4);
		border-radius: 999px;
		border: 1px solid var(--border);
		background: transparent;
		color: var(--text-secondary);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all var(--duration-fast) var(--ease);
		text-transform: capitalize;
	}
	.filter-pill:hover {
		border-color: var(--accent-dim);
		color: var(--text-primary);
	}
	.filter-pill.active {
		background: var(--accent);
		border-color: var(--accent);
		color: var(--bg-base);
	}

	.empty-state {
		text-align: center;
		color: #5a586e;
		padding: var(--space-16) 0;
	}

	/* Family sections */
	.family-section { margin-bottom: var(--space-10); }
	.family-heading {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--accent);
		text-transform: capitalize;
		letter-spacing: 0.01em;
		margin-bottom: var(--space-4);
		padding-bottom: var(--space-2);
		border-bottom: 1px solid var(--border);
	}

	/* Mechanic cards */
	.mechanic-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: var(--space-5);
	}
	.mechanic-card {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		padding: var(--space-5);
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		text-decoration: none;
		transition: border-color var(--duration-fast) var(--ease),
		            box-shadow var(--duration-fast) var(--ease),
		            transform var(--duration-fast) var(--ease);
	}
	.mechanic-card:hover {
		border-color: var(--accent-dim);
		box-shadow: 0 4px 24px rgba(0, 0, 0, 0.3);
		transform: translateY(-2px);
	}
	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.family-badge {
		font-size: 0.6875rem;
		padding: 2px 8px;
		border-radius: 999px;
		border: 1px solid var(--accent-dim);
		color: var(--accent);
		text-transform: uppercase;
		letter-spacing: 0.04em;
		font-weight: 500;
	}
	.card-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary);
	}
	.card-def {
		font-size: 0.875rem;
		color: var(--text-secondary);
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
	.card-verbs {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-1);
		margin-top: auto;
	}
	.verb-chip {
		font-size: 0.6875rem;
		padding: 2px 6px;
		border-radius: 4px;
		background: rgba(255,255,255,0.05);
		color: #9391a8;
		font-family: 'JetBrains Mono', monospace;
	}
</style>
