<script lang="ts">
	import ShellDrawer from './ShellDrawer.svelte';
	import { api } from '$lib/api';

	let { session, demo }: { session: any; demo: any } = $props();

	let open = $state(true);
	let drawerOpen = $state(false);
	let hypothesis = $state('');
	let expected = $state('');
	let saving = $state(false);
	let saved = $state(false);

	// Compute current param snapshot for display
	let paramSnapshot = $derived(() => {
		if (!session.paramManifest?.params) return [];
		return session.paramManifest.params.map((p: any) => ({
			label: p.label,
			value: session.paramValues[p.key]?.toFixed(p.kind === 'float' ? 2 : 0) ?? p.default,
		}));
	});

	function openDrawer() {
		hypothesis = '';
		expected = '';
		saved = false;
		drawerOpen = true;
	}

	async function saveExperiment() {
		if (!hypothesis.trim()) return;
		saving = true;
		try {
			const snapshot: Record<string, number> = {};
			if (session.paramManifest?.params) {
				for (const p of session.paramManifest.params) {
					snapshot[p.key] = session.paramValues[p.key] ?? p.default;
				}
			}
			const res = await fetch(api('/experiments'), {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					parent_demo: demo.id,
					hypothesis: hypothesis.trim(),
					expected_effect: expected.trim() || null,
					param_snapshot: snapshot,
				}),
			});
			if (res.ok) {
				saved = true;
				setTimeout(() => {
					saved = false;
					drawerOpen = false;
					hypothesis = '';
					expected = '';
				}, 1800);
			}
		} catch { /* ignore */ }
		saving = false;
	}
</script>

{#if session.paramManifest?.params.length}
	<div class="panel">
		<button class="panel-header" onclick={() => open = !open} aria-expanded={open}>
			<span class="panel-title">🧪 Experiments</span>
			<span class="panel-toggle">{open ? '▲' : '▼'}</span>
		</button>
		{#if open}
			<div class="panel-body">
				<!-- Current param snapshot summary -->
				<div class="snapshot-row">
					{#each paramSnapshot() as item}
						<span class="snapshot-item">
							<span class="snap-label">{item.label}</span>
							<span class="snap-value">{item.value}</span>
						</span>
					{/each}
				</div>
				<button class="trigger-btn" onclick={openDrawer}>
					Save as experiment →
				</button>
			</div>
		{/if}
	</div>
{/if}

<!-- Drawer renders over canvas, sidebar stays visible -->
<ShellDrawer bind:open={drawerOpen} title="🧪 New experiment">
	<div class="field">
		<label class="field-label" for="exp-hypothesis">Hypothesis</label>
		<textarea
			id="exp-hypothesis"
			class="field-input"
			rows="3"
			placeholder="What are you testing? What do you expect to change about the feel?"
			bind:value={hypothesis}
		></textarea>
	</div>

	<div class="field">
		<label class="field-label" for="exp-expected">Expected effect <span class="optional">(optional)</span></label>
		<textarea
			id="exp-expected"
			class="field-input"
			rows="2"
			placeholder="How should the mechanic feel different?"
			bind:value={expected}
		></textarea>
	</div>

	<div class="param-snapshot-block">
		<span class="snapshot-title">Param snapshot</span>
		<div class="snapshot-grid">
			{#each paramSnapshot() as item}
				<span class="snap-key">{item.label}</span>
				<span class="snap-val">{item.value}</span>
			{/each}
		</div>
	</div>

	<div class="drawer-actions">
		{#if saved}
			<span class="success-msg">✓ Experiment saved!</span>
		{:else}
			<button
				class="btn btn-primary"
				disabled={saving || !hypothesis.trim()}
				onclick={saveExperiment}
			>
				{saving ? 'Saving…' : 'Save experiment'}
			</button>
			<button class="btn btn-ghost" onclick={() => drawerOpen = false}>Cancel</button>
		{/if}
	</div>
</ShellDrawer>

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
	}
	.panel-toggle {
		font-size: 0.625rem;
		color: var(--text-muted);
	}
	.panel-body {
		padding: var(--space-3) var(--space-4);
		border-top: 1px solid var(--border);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	/* Compact param snapshot in sidebar */
	.snapshot-row {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}
	.snapshot-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-size: 0.75rem;
	}
	.snap-label { color: var(--text-muted); }
	.snap-value {
		font-family: var(--font-mono);
		color: var(--accent);
		font-size: 0.7rem;
	}

	.trigger-btn {
		width: 100%;
		padding: var(--space-2) var(--space-3);
		background: rgba(245, 158, 11, 0.06);
		border: 1px dashed rgba(245, 158, 11, 0.3);
		border-radius: var(--radius-md);
		color: var(--accent);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 120ms ease;
		text-align: center;
	}
	.trigger-btn:hover {
		background: rgba(245, 158, 11, 0.12);
		border-color: var(--accent);
	}

	/* Drawer form styles */
	.field {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}
	.field-label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-secondary);
	}
	.optional {
		color: var(--text-muted);
		font-weight: 400;
	}
	.field-input {
		background: var(--bg-subtle);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		color: var(--text-primary);
		font-family: var(--font-sans);
		font-size: 0.9375rem;
		padding: 0.625rem 0.875rem;
		outline: none;
		resize: vertical;
		line-height: 1.5;
		transition: border-color 120ms ease, box-shadow 120ms ease;
		width: 100%;
	}
	.field-input:focus {
		border-color: var(--accent-dim);
		box-shadow: 0 0 0 3px var(--accent-glow);
	}
	.field-input::placeholder { color: var(--text-muted); }

	.param-snapshot-block {
		background: rgba(255,255,255,0.03);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		padding: 0.875rem 1rem;
	}
	.snapshot-title {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-muted);
		text-transform: uppercase;
		letter-spacing: 0.06em;
		display: block;
		margin-bottom: 0.625rem;
	}
	.snapshot-grid {
		display: grid;
		grid-template-columns: 1fr auto;
		gap: 0.25rem 1rem;
	}
	.snap-key {
		font-size: 0.8125rem;
		color: var(--text-secondary);
	}
	.snap-val {
		font-family: var(--font-mono);
		font-size: 0.8125rem;
		color: var(--accent);
		text-align: right;
	}

	.drawer-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding-top: 0.5rem;
	}
	.success-msg {
		font-size: 0.9rem;
		color: #22c55e;
		font-weight: 600;
	}
</style>
