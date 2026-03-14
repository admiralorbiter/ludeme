<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';

	let { children } = $props();

	const navLinks = [
		{ href: '/',            label: 'Discover' },
		{ href: '/mechanics',   label: 'Mechanics' },
		{ href: '/works',       label: 'Works' },
		{ href: '/collections', label: 'Collections' },
	];
</script>

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
						type="search"
						class="nav-search"
						placeholder="Search mechanics, works, demos…"
						aria-label="Search"
					/>
					<kbd class="search-kbd">⌘K</kbd>
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

	/* ---- Main content ---- */
	.main-content {
		flex: 1;
		width: 100%;
	}
</style>
