<script lang="ts">
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/tauri';

	import Table from './Table.svelte';
	
	export let username;
	let data = [
        [123, "Name1", "Open"],
        [456, "Name2", "Closed"],
    ]

	onMount(load_loans);

	async function load_loans() {
		await invoke('get_customer_loans', { user: username })
			.then((result: [any]) => {
				console.log(result);

				data = result.map(x => [x.loan_id, x.loan_amount, x.start_date])	
			})
			.catch((error) => console.log(error));
	}

	

</script>

<div transition:slide={{delay: 250, duration: 500}}>
	<Table {data} />
</div>
