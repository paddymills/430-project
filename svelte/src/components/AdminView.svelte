<script lang="ts">
	import { slide } from 'svelte/transition';
	import Table from './Table.svelte';

	let type = "Admin";

</script>

<div transition:slide={{delay: 250, duration: 500}}>
	<Table />
</div>
