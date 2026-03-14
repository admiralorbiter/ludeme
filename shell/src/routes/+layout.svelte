<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';

	let { children } = $props();

	const navLinks = [
		{ href: '/',            label: 'Discover' },
		{ href: '/mechanics',   label: 'Mechanics' },
		{ href: '/works',       label: 'Works' },
		{ href: '/collections', label: 'Collections' },
	];

	// Search state
	let searchQuery = $state('');
	let searchResults = $state<Array<{entity_type: string; entity_id: string; title: string; snippet: string}>>([]);
	let showResults = $state(false);
	let searchInputEl = $state<HTMLInputElement | null>(null);
	let debounceTimer: ReturnType<typeof setTimeout> | null = null;

	const TYPE_ICONS: Record<string, string> = {
		demo:       '▶',
		mechanic:   '⚙',
		work:       '📕',
		collection: '📚',
	};
	const TYPE_LABELS: Record<string, string> = {
		demo:       'Demo',
		mechanic:   'Mechanic',
		work:       'Work',
		collection: 'Collection',
	};

	function handleSearchInput(e: Event) {
		const q = (e.target as HTMLInputElement).value;
		searchQuery = q;
		if (debounceTimer) clearTimeout(debounceTimer);
		if (q.trim().length < 2) {
			searchResults = [];
			showResults = false;
			return;
		}
		debounceTimer = setTimeout(() => doSearch(q), 200);
	}

	async function doSearch(q: string) {
		try {
			const res = await fetch(`/api/search?q=${encodeURIComponent(q)}`);
			if (res.ok) {
				searchResults = await res.json();
				showResults = searchResults.length > 0;
			}
		} catch { /* ignore */ }
	}

	function navigateToResult(result: typeof searchResults[0]) {
		showResults = false;
		searchQuery = '';
		const path = result.entity_type === 'demo'
			? `/demo/${result.entity_id}`
			: result.entity_type === 'mechanic'
			? `/mechanics/${result.entity_id}`
			: result.entity_type === 'collection'
			? `/collections/${result.entity_id}`
			: `/works/${result.entity_id}`;
		goto(path);
	}

	function handleSearchKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			showResults = false;
			searchInputEl?.blur();
		}
	}

	function handleGlobalKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
			e.preventDefault();
			searchInputEl?.focus();
		}
	}
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<svelte:head>
	<title>Ludeme — Playable Mechanics Atlas</title>
</svelte:head>

<div class="app-shell">
	<nav class="topnav" aria-label="Main navigation">
		<div class="topnav-inner">
			<!-- Wordmark -->
			<a href="/" class="wordmark" aria-label="Ludeme home">
				<span class="wordmark-icon">⬡</span>
				<span class="wordmark-text">Ludeme</span>
			</a>

			<!-- Nav links -->
			<ul class="nav-links" role="list">
				{#each navLinks as link}
					<li>
						<a
							href={link.href}
							class="nav-link"
							class:active={$page.url.pathname === link.href ||
								           ($page.url.pathname.startsWith(link.href) && link.href !== '/')}
						>
							{link.label}
						</a>
					</li>
				{/each}
			</ul>

			<!-- Right side -->
			<div class="nav-right">
				<div class="search-wrap">
					<svg class="search-icon" width="15" height="15" viewBox="0 0 15 15" fill="none">
						<circle cx="6.5" cy="6.5" r="5" stroke="currentColor" stroke-width="1.5"/>
						<path d="M10.5 10.5L13.5 13.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
					</svg>
					<input
						bind:this={searchInputEl}
						type="search"
						class="nav-search"
						placeholder="Search mechanics, works, demos…"
						aria-label="Search"
						value={searchQuery}
						oninput={handleSearchInput}
						onkeydown={handleSearchKeydown}
						onfocus={() => { if (searchResults.length) showResults = true; }}
						onblur={() => { setTimeout(() => showResults = false, 200); }}
					/>
					<kbd class="search-kbd">⌘K</kbd>

					{#if showResults}
						<div class="search-dropdown" role="listbox">
							{#each searchResults as result}
								<button
									class="search-result"
									role="option"
									onclick={() => navigateToResult(result)}
								>
									<span class="result-icon">{TYPE_ICONS[result.entity_type] ?? '•'}</span>
									<div class="result-body">
										<span class="result-title">{result.title}</span>
										<span class="result-type">{TYPE_LABELS[result.entity_type] ?? result.entity_type}</span>
									</div>
								</button>
							{/each}
						</div>
					{/if}
				</div>
			</div>
		</div>
	</nav>

	<main class="main-content">
		{@render children()}
	</main>
</div>

<style>
	.app-shell {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	/* ---- Top Nav ---- */
	.topnav {
		position: sticky;
		top: 0;
		z-index: 100;
		height: var(--nav-height);
		background: rgba(10, 10, 15, 0.85);
		backdrop-filter: blur(16px);
		-webkit-backdrop-filter: blur(16px);
		border-bottom: 1px solid var(--border);
	}

	.topnav-inner {
		max-width: var(--content-max);
		margin: 0 auto;
		padding: 0 var(--space-6);
		height: 100%;
		display: flex;
		align-items: center;
		gap: var(--space-8);
	}

	/* Wordmark */
	.wordmark {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		text-decoration: none;
		flex-shrink: 0;
	}
	.wordmark-icon {
		font-size: 1.25rem;
		color: var(--accent);
		line-height: 1;
	}
	.wordmark-text {
		font-size: 1.0625rem;
		font-weight: 700;
		color: var(--text-primary);
		letter-spacing: -0.03em;
	}

	/* Nav links */
	.nav-links {
		display: flex;
		list-style: none;
		gap: var(--space-1);
		flex-shrink: 0;
	}
	.nav-link {
		padding: var(--space-2) var(--space-3);
		border-radius: var(--radius-md);
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-secondary);
		text-decoration: none;
		transition: color var(--duration-fast) var(--ease),
		            background var(--duration-fast) var(--ease);
	}
	.nav-link:hover {
		color: var(--text-primary);
		background: rgba(255, 255, 255, 0.05);
	}
	.nav-link.active {
		color: var(--text-primary);
		background: rgba(255, 255, 255, 0.08);
	}

	/* Right side search */
	.nav-right { margin-left: auto; }

	.search-wrap {
		position: relative;
		display: flex;
		align-items: center;
	}
	.search-icon {
		position: absolute;
		left: var(--space-3);
		color: var(--text-muted);
		pointer-events: none;
	}
	.nav-search {
		background: var(--bg-overlay);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		color: var(--text-primary);
		font-family: var(--font-sans);
		font-size: 0.8125rem;
		padding: var(--space-2) var(--space-8) var(--space-2) 2.25rem;
		outline: none;
		width: 260px;
		transition: border-color var(--duration-fast) var(--ease),
		            box-shadow var(--duration-fast) var(--ease);
	}
	.nav-search:focus {
		border-color: var(--accent-dim);
		box-shadow: 0 0 0 3px var(--accent-glow);
	}
	.nav-search::placeholder { color: var(--text-muted); }
	.search-kbd {
		position: absolute;
		right: var(--space-2);
		font-family: var(--font-mono);
		font-size: 0.6875rem;
		color: var(--text-muted);
		background: var(--bg-subtle);
		border: 1px solid var(--border);
		border-radius: 4px;
		padding: 1px 5px;
	}

	/* Search dropdown */
	.search-dropdown {
		position: absolute;
		top: calc(100% + 6px);
		left: 0;
		right: 0;
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		box-shadow: 0 12px 32px rgba(0, 0, 0, 0.5);
		max-height: 360px;
		overflow-y: auto;
		z-index: 200;
		padding: var(--space-1);
	}

	.search-result {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		width: 100%;
		padding: var(--space-2) var(--space-3);
		background: none;
		border: none;
		border-radius: var(--radius-md);
		cursor: pointer;
		text-align: left;
		color: #f1f0f7;
		font-family: var(--font-sans);
		transition: background var(--duration-fast) var(--ease);
	}
	.search-result:hover {
		background: rgba(255, 255, 255, 0.06);
	}

	.result-icon {
		flex-shrink: 0;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.875rem;
		color: var(--accent);
		background: var(--accent-muted);
		border-radius: var(--radius-sm);
	}

	.result-body {
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.result-title {
		font-size: 0.8125rem;
		font-weight: 500;
		color: #f1f0f7;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.result-type {
		font-size: 0.6875rem;
		color: #9391a8;
		text-transform: uppercase;
		letter-spacing: 0.04em;
	}

	/* ---- Main content ---- */
	.main-content {
		flex: 1;
		width: 100%;
	}
</style>
