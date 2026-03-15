<script lang="ts">
	let {
		session,
		onParamChange,
	}: {
		session: any;
		onParamChange: (key: string, value: number) => void;
	} = $props();

	let open = $state(true);
</script>

{#if session.paramManifest?.params.length}
	<div class="panel">
		<button class="panel-header" onclick={() => open = !open} aria-expanded={open}>
			<span class="panel-title">Parameter tuner</span>
			<span class="panel-toggle">{open ? '▲' : '▼'}</span>
		</button>
		{#if open}
			<div class="panel-body">
				{#each session.paramManifest.params as param}
					<div class="param-row">
						<label class="param-label" for="param-{param.key}">
							{param.label}
							<span class="param-value">
								{session.paramValues[param.key]?.toFixed(param.kind === 'float' ? 2 : 0) ?? param.default}
							</span>
						</label>
						{#if param.kind === 'toggle'}
							<input
								type="checkbox"
								id="param-{param.key}"
								checked={Boolean(session.paramValues[param.key])}
								onchange={(e) => onParamChange(param.key, e.currentTarget.checked ? 1 : 0)}
								class="param-toggle"
							/>
						{:else}
							<input
								type="range"
								id="param-{param.key}"
								min={param.min}
								max={param.max}
								step={param.step}
								value={session.paramValues[param.key] ?? param.default}
								oninput={(e) => onParamChange(param.key, Number(e.currentTarget.value))}
								class="param-slider"
							/>
						{/if}
					</div>
				{/each}
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
	.param-row {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}
	.param-label {
		display: flex;
		justify-content: space-between;
		font-size: 0.8125rem;
		color: var(--text-secondary);
	}
	.param-value {
		font-family: var(--font-mono);
		font-size: 0.75rem;
		color: var(--accent);
	}
	.param-slider {
		width: 100%;
		accent-color: var(--accent);
		cursor: pointer;
	}
	.param-toggle {
		width: 16px;
		height: 16px;
		accent-color: var(--accent);
		cursor: pointer;
	}
</style>
