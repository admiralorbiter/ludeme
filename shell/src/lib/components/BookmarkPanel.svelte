<script lang="ts">
	let { session }: { session: any } = $props();

	let open = $state(false);
</script>

{#if session.recentMoments.length || session.sessionBookmarks.length}
	<div class="panel">
		<button class="panel-header" onclick={() => open = !open} aria-expanded={open}>
			<span class="panel-title">
				📌 Moments
				{#if session.sessionBookmarks.length}
					<span class="panel-badge">{session.sessionBookmarks.length}</span>
				{/if}
			</span>
			<span class="panel-toggle">{open ? '▲' : '▼'}</span>
		</button>
		{#if open}
			<div class="panel-body">
				{#if session.sessionBookmarks.length}
					<div class="section-label">Saved</div>
					{#each session.sessionBookmarks as bm}
						<div class="bookmark-row">
							<span class="bookmark-frame">f{bm.frame}</span>
							<span class="bookmark-label">{bm.player_label ?? 'Untitled'}</span>
						</div>
					{/each}
				{/if}
				{#if session.recentMoments.length}
					<div class="section-label">Auto-emitted</div>
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
				{/if}
			</div>
		{/if}
	</div>
{/if}

<style>
	.panel {
		background: var(--bg-raised);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		overflow: hidden;
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
	.panel-header:hover { background: rgba(255,255,255,0.03); }
	.panel-title {
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--text-primary);
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}
	.panel-toggle { font-size: 0.625rem; color: var(--text-muted); }
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
		gap: var(--space-2);
	}
	.section-label {
		font-size: 0.6875rem;
		text-transform: uppercase;
		letter-spacing: 0.06em;
		color: var(--text-muted);
		font-weight: 600;
		margin-top: var(--space-1);
	}
	.section-label:first-child { margin-top: 0; }
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
	.bookmark-tags {
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
</style>
