<script lang="ts">
	import { api } from '$lib/api';

	let {
		demo,
		publishState = $bindable('draft'),
	}: {
		demo: any;
		publishState?: string;
	} = $props();

	let open = $state(true);
	let readiness = $state<{ ready: boolean; checks: Array<{ field: string; ok: boolean; message: string }> } | null>(null);
	let loading = $state(false);

	async function fetchReadiness() {
		try {
			const res = await fetch(api(`/readiness/demo/${demo.id}`));
			if (res.ok) readiness = await res.json();
		} catch { /* ignore */ }
	}

	async function transitionState(newState: string) {
		loading = true;
		try {
			const res = await fetch(api('/publish'), {
				method: 'PATCH',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ entity_type: 'demo', id: demo.id, new_state: newState }),
			});
			if (res.ok) {
				publishState = newState;
				await fetchReadiness();
			}
		} catch { /* ignore */ }
		loading = false;
	}

	$effect(() => {
		publishState = demo.publish_state ?? 'draft';
		fetchReadiness();
	});
</script>

<div class="panel">
	<button class="panel-header" onclick={() => open = !open} aria-expanded={open}>
		<span>
			<span class="publish-dot {publishState}"></span>
			Publish
		</span>
		<span class="caret">{open ? '▴' : '▾'}</span>
	</button>
	{#if open}
		<div class="panel-body">
			<div class="publish-state-row">
				<span class="publish-badge {publishState}">{publishState}</span>
				{#if publishState === 'draft'}
					<button class="publish-btn" disabled={loading} onclick={() => transitionState('review')}>→ Review</button>
				{:else if publishState === 'review'}
					<button class="publish-btn" disabled={loading || !(readiness?.ready)} onclick={() => transitionState('public')}>→ Public</button>
					<button class="publish-btn-secondary" disabled={loading} onclick={() => transitionState('draft')}>← Draft</button>
				{:else}
					<button class="publish-btn-secondary" disabled={loading} onclick={() => transitionState('review')}>← Review</button>
				{/if}
			</div>
			{#if readiness}
				<div class="readiness-list">
					{#each readiness.checks as check}
						<div class="readiness-item" class:ok={check.ok} class:fail={!check.ok}>
							<span class="readiness-icon">{check.ok ? '✓' : '✗'}</span>
							<span class="readiness-label">{check.field}</span>
							{#if !check.ok}
								<span class="readiness-msg">{check.message}</span>
							{/if}
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>

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
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--text-primary);
	}
	.panel-header:hover { background: rgba(255,255,255,0.03); }
	.caret { font-size: 0.625rem; color: var(--text-muted); }
	.panel-body {
		padding: var(--space-3) var(--space-4);
		border-top: 1px solid var(--border);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}
	.publish-dot {
		display: inline-block;
		width: 8px; height: 8px;
		border-radius: 50%;
		margin-right: var(--space-1);
	}
	.publish-dot.draft  { background: #ef4444; }
	.publish-dot.review { background: #f59e0b; }
	.publish-dot.public { background: #22c55e; }
	.publish-state-row {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		flex-wrap: wrap;
	}
	.publish-badge {
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.04em;
		padding: 2px 8px;
		border-radius: 999px;
	}
	.publish-badge.draft  { color: #ef4444; background: rgba(239,68,68,0.15); }
	.publish-badge.review { color: #f59e0b; background: rgba(245,158,11,0.15); }
	.publish-badge.public { color: #22c55e; background: rgba(34,197,94,0.15); }
	.publish-btn, .publish-btn-secondary {
		font-size: 0.6875rem;
		font-weight: 500;
		padding: 3px 10px;
		border-radius: var(--radius-sm);
		border: none;
		cursor: pointer;
		font-family: var(--font-sans);
		transition: opacity 0.15s;
	}
	.publish-btn { background: var(--accent); color: #111; }
	.publish-btn-secondary { background: rgba(255,255,255,0.08); color: #9391a8; }
	.publish-btn:disabled, .publish-btn-secondary:disabled { opacity: 0.4; cursor: not-allowed; }
	.readiness-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
		margin-top: var(--space-1);
	}
	.readiness-item {
		display: flex;
		align-items: baseline;
		gap: var(--space-2);
		font-size: 0.75rem;
	}
	.readiness-icon { font-weight: 700; flex-shrink: 0; }
	.readiness-item.ok .readiness-icon { color: #22c55e; }
	.readiness-item.fail .readiness-icon { color: #ef4444; }
	.readiness-label { color: #9391a8; }
	.readiness-msg { color: #ef4444; font-size: 0.6875rem; }
</style>
