<script lang="ts">
	import ShellDrawer from './ShellDrawer.svelte';
	import { api } from '$lib/api';
	import { onMount } from 'svelte';

	let { demo }: { demo: any } = $props();

	let open = $state(false);
	let drawerOpen = $state(false);
	let obsClaim = $state('');
	let obsConfidence = $state('tentative');
	let obsWhy = $state('');
	let obsFollowUp = $state('');
	let saving = $state(false);
	let saved = $state(false);
	let observations = $state<any[]>([]);

	onMount(() => loadObservations());

	async function loadObservations() {
		try {
			const res = await fetch(api(`/observations?demo_id=${demo.id}`));
			if (res.ok) observations = await res.json();
		} catch { /* ignore */ }
	}

	function openDrawer() {
		obsClaim = '';
		obsConfidence = 'tentative';
		obsWhy = '';
		obsFollowUp = '';
		saved = false;
		drawerOpen = true;
	}

	async function saveObservation() {
		if (!obsClaim.trim()) return;
		saving = true;
		try {
			const res = await fetch(api('/observations'), {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					claim: obsClaim.trim(),
					linked_entities: [{ id: demo.id, type: 'demo' }],
					confidence: obsConfidence,
					why_it_matters: obsWhy.trim() || null,
					follow_up_question: obsFollowUp.trim() || null,
				}),
			});
			if (res.ok) {
				saved = true;
				await loadObservations();
				setTimeout(() => {
					saved = false;
					drawerOpen = false;
					obsClaim = '';
					obsWhy = '';
					obsFollowUp = '';
					obsConfidence = 'tentative';
				}, 1800);
			}
		} catch { /* ignore */ }
		saving = false;
	}

	const CONFIDENCE_COLORS: Record<string, string> = {
		speculative: '#ef4444',
		tentative: '#f59e0b',
		supported: '#3b82f6',
		established: '#22c55e',
	};
</script>

<div class="panel">
	<button class="panel-header" onclick={() => open = !open} aria-expanded={open}>
		<span class="panel-title">
			📝 Observations
			{#if observations.length}
				<span class="panel-badge">{observations.length}</span>
			{/if}
		</span>
		<span class="panel-toggle">{open ? '▲' : '▼'}</span>
	</button>
	{#if open}
		<div class="panel-body">
			<!-- Add button always at the top -->
			<button class="add-btn" onclick={openDrawer}>
				+ Add observation
			</button>

			<!-- Saved observations list -->
			{#if observations.length}
				<div class="obs-list">
					{#each observations as obs}
						<div class="obs-card">
							<div class="obs-header">
								<span
									class="obs-confidence-badge"
									style="color: {CONFIDENCE_COLORS[obs.confidence] ?? 'var(--text-muted)'}; border-color: {CONFIDENCE_COLORS[obs.confidence] ?? 'var(--border)'}40;"
								>
									{obs.confidence}
								</span>
							</div>
							<p class="obs-claim">{obs.claim}</p>
							{#if obs.why_it_matters}
								<p class="obs-why">{obs.why_it_matters}</p>
							{/if}
							{#if obs.follow_up_question}
								<p class="obs-followup">❓ {obs.follow_up_question}</p>
							{/if}
						</div>
					{/each}
				</div>
			{:else}
				<p class="empty-hint">No observations yet. Add one after playing.</p>
			{/if}
		</div>
	{/if}
</div>

<!-- Drawer renders over canvas, sidebar + observations list stay visible -->
<ShellDrawer bind:open={drawerOpen} title="📝 New observation">
	<div class="field">
		<label class="field-label" for="obs-claim">Observation <span class="required">*</span></label>
		<textarea
			id="obs-claim"
			class="field-input"
			rows="4"
			placeholder="What did you notice during play? Be as specific as possible."
			bind:value={obsClaim}
		></textarea>
	</div>

	<div class="field">
		<label class="field-label" for="obs-confidence">Confidence</label>
		<div class="confidence-select-wrap">
			<select id="obs-confidence" class="confidence-select" bind:value={obsConfidence}>
				<option value="speculative">Speculative — gut feeling, no strong evidence</option>
				<option value="tentative">Tentative — noticed it once or twice</option>
				<option value="supported">Supported — repeatable, consistent</option>
				<option value="established">Established — well-evidenced across sessions</option>
			</select>
		</div>
	</div>

	<div class="field">
		<label class="field-label" for="obs-why">Why does this matter? <span class="optional">(optional)</span></label>
		<textarea
			id="obs-why"
			class="field-input"
			rows="2"
			placeholder="What does this tell us about the mechanic's design?"
			bind:value={obsWhy}
		></textarea>
	</div>

	<div class="field">
		<label class="field-label" for="obs-followup">Follow-up question <span class="optional">(optional)</span></label>
		<textarea
			id="obs-followup"
			class="field-input"
			rows="2"
			placeholder="What would you test next?"
			bind:value={obsFollowUp}
		></textarea>
	</div>

	<div class="drawer-actions">
		{#if saved}
			<span class="success-msg">✓ Observation saved!</span>
		{:else}
			<button
				class="btn btn-primary"
				disabled={saving || !obsClaim.trim()}
				onclick={saveObservation}
			>
				{saving ? 'Saving…' : 'Save observation'}
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
		gap: var(--space-3);
	}

	/* Add button always at top */
	.add-btn {
		width: 100%;
		padding: var(--space-2) var(--space-3);
		background: rgba(20, 184, 166, 0.05);
		border: 1px dashed rgba(20, 184, 166, 0.3);
		border-radius: var(--radius-md);
		color: var(--teal);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 120ms ease;
		text-align: center;
	}
	.add-btn:hover {
		background: rgba(20, 184, 166, 0.1);
		border-color: var(--teal);
	}

	.obs-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}
	.obs-card {
		padding: var(--space-3);
		background: rgba(255,255,255,0.03);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}
	.obs-header { display: flex; align-items: center; }
	.obs-confidence-badge {
		font-size: 0.625rem;
		padding: 2px 6px;
		border-radius: 999px;
		text-transform: uppercase;
		letter-spacing: 0.04em;
		font-weight: 500;
		border: 1px solid;
	}
	.obs-claim {
		font-size: 0.8125rem;
		color: var(--text-primary);
		line-height: 1.5;
	}
	.obs-why {
		font-size: 0.75rem;
		color: var(--text-secondary);
		font-style: italic;
	}
	.obs-followup {
		font-size: 0.75rem;
		color: var(--accent);
	}
	.empty-hint {
		font-size: 0.8rem;
		color: var(--text-muted);
		font-style: italic;
		text-align: center;
		padding: var(--space-2) 0;
	}

	/* Drawer form */
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
	.required { color: var(--accent); }
	.optional { color: var(--text-muted); font-weight: 400; }
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

	.confidence-select-wrap {
		position: relative;
	}
	.confidence-select {
		width: 100%;
		background: var(--bg-subtle);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		color: var(--text-primary);
		font-family: var(--font-sans);
		font-size: 0.875rem;
		padding: 0.5rem 0.875rem;
		cursor: pointer;
		outline: none;
		appearance: auto;
	}
	.confidence-select:focus {
		border-color: var(--accent-dim);
		box-shadow: 0 0 0 3px var(--accent-glow);
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
