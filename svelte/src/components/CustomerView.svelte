<script lang="ts">
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api/tauri';

	import Table from './Table.svelte';
	
	export let username;
	let data = [
        { id: 123, amt: "$200.00", date: "2017-05-22" },
        { id: 456, amt: "$100.00", date: "2020-12-01" },
    ]

	onMount(load_loans);

	async function load_loans() {
		await invoke('get_cust_loans', { user: username })
			.then((result: [any]) => {
				// console.log(result);

				data = result.map(x => {
					x.editing = false;

					return x;
				});

				console.log(data);
			})
			.catch((error) => console.log(error));
	}
</script>

<div transition:slide={{delay: 250, duration: 500}}>
	<Table {data} />
</div>
