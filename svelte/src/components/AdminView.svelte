<script lang="ts">
	import { slide } from 'svelte/transition';
	import Table from './Table.svelte';
</script>

<div transition:slide={{delay: 250, duration: 500}} class="p-5">
	<h1 class="text-secondary">Menu goes here</h1>
</div>
