<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		open = $bindable(false),
		title = '',
		onclose,
		children,
	}: {
		open: boolean;
		title: string;
		onclose?: () => void;
		children?: Snippet;
	} = $props();

	function close() {
		open = false;
		onclose?.();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') close();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
	<!-- Backdrop dims the canvas, leaves sidebar untouched -->
	<div class="drawer-backdrop" onclick={close} role="presentation"></div>

	<!-- Drawer slides in over the canvas area from the left edge of canvas-col -->
	<div class="drawer" role="dialog" aria-modal="true" aria-label={title}>
		<div class="drawer-header">
			<span class="drawer-title">{title}</span>
			<button class="drawer-close" onclick={close} aria-label="Close">✕</button>
		</div>
		<div class="drawer-body">
			{@render children?.()}
		</div>
	</div>
{/if}

<style>
	/* Backdrop: covers only the canvas col area — positioned inside .play-area's canvas-col */
	.drawer-backdrop {
		position: fixed;
		inset: 0;
		/* Semitransparent so canvas is still dimly visible behind the drawer */
		background: rgba(10, 10, 15, 0.55);
		backdrop-filter: blur(2px);
		z-index: 100;
		cursor: pointer;
		animation: fadeIn 180ms ease both;
	}

	.drawer {
		position: fixed;
		/* Anchored to the left of the viewport — sits over the canvas, away from sidebar */
		top: 0;
		left: 0;
		bottom: 0;
		width: min(460px, calc(100vw - 360px));
		background: var(--bg-overlay);
		border-right: 1px solid var(--border-hover);
		box-shadow: 4px 0 32px rgba(0, 0, 0, 0.5);
		z-index: 101;
		display: flex;
		flex-direction: column;
		animation: slideIn 220ms cubic-bezier(0.16, 1, 0.3, 1) both;
		overflow: hidden;
	}

	.drawer-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid var(--border);
		flex-shrink: 0;
		background: var(--bg-raised);
	}

	.drawer-title {
		font-size: 0.9375rem;
		font-weight: 600;
		color: var(--text-primary);
		letter-spacing: -0.01em;
	}

	.drawer-close {
		background: none;
		border: none;
		cursor: pointer;
		color: var(--text-muted);
		font-size: 1rem;
		line-height: 1;
		padding: 4px 6px;
		border-radius: var(--radius-sm);
		transition: color 120ms ease, background 120ms ease;
	}
	.drawer-close:hover {
		color: var(--text-primary);
		background: rgba(255, 255, 255, 0.06);
	}

	.drawer-body {
		flex: 1;
		overflow-y: auto;
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
		scrollbar-width: thin;
		scrollbar-color: rgba(255,255,255,0.12) transparent;
	}
	.drawer-body::-webkit-scrollbar { width: 4px; }
	.drawer-body::-webkit-scrollbar-track { background: transparent; }
	.drawer-body::-webkit-scrollbar-thumb { background: rgba(255,255,255,0.12); border-radius: 2px; }

	@keyframes fadeIn {
		from { opacity: 0; }
		to   { opacity: 1; }
	}

	@keyframes slideIn {
		from { transform: translateX(-100%); opacity: 0; }
		to   { transform: translateX(0);    opacity: 1; }
	}

	/* Mobile: drawer goes full-width below sidebar */
	@media (max-width: 900px) {
		.drawer {
			left: 0;
			right: 0;
			top: auto;
			bottom: 0;
			width: 100%;
			height: 70vh;
			border-right: none;
			border-top: 1px solid var(--border-hover);
			animation: slideUp 220ms cubic-bezier(0.16, 1, 0.3, 1) both;
		}
		@keyframes slideUp {
			from { transform: translateY(100%); opacity: 0; }
			to   { transform: translateY(0);    opacity: 1; }
		}
	}
</style>
