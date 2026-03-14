<script lang="ts">
	import { page } from '$app/stores';
</script>

<svelte:head>
	<title>
		{$page.status === 404 ? 'Not found' : 'Error'} — Ludeme
	</title>
</svelte:head>

<div class="error-shell">
	<div class="error-inner">
		<div class="error-code">{$page.status}</div>
		<h1 class="error-title">
			{#if $page.status === 404}
				That page doesn't exist yet.
			{:else if $page.status === 403}
				Access denied.
			{:else}
				Something went wrong.
			{/if}
		</h1>
		<p class="error-body">
			{#if $page.status === 404}
				The demo, mechanic, or work you're looking for hasn't been added yet,
				or the URL might be wrong.
			{:else}
				{$page.error?.message ?? 'An unexpected error occurred.'}
			{/if}
		</p>
		<div class="error-actions">
			<a href="/" class="btn btn-primary">Back to Discover</a>
			<a href="/mechanics" class="btn btn-ghost">Browse mechanics</a>
		</div>
	</div>
</div>

<style>
	.error-shell {
		min-height: calc(100vh - var(--nav-height));
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-8);
	}
	.error-inner {
		text-align: center;
		max-width: 480px;
	}
	.error-code {
		font-size: 5rem;
		font-weight: 700;
		color: var(--border);
		letter-spacing: -0.05em;
		line-height: 1;
		margin-bottom: var(--space-4);
		font-family: var(--font-mono);
	}
	.error-title {
		font-size: 1.5rem;
		margin-bottom: var(--space-3);
	}
	.error-body {
		font-size: 0.9375rem;
		color: var(--text-muted);
		line-height: 1.7;
		margin-bottom: var(--space-8);
	}
	.error-actions {
		display: flex;
		gap: var(--space-3);
		justify-content: center;
		flex-wrap: wrap;
	}
</style>
