<script lang="ts">
	import { slide } from 'svelte/transition';
	import { Button, Icon } from 'sveltestrap';
	import Table from './Table.svelte';

	import { invoke } from '@tauri-apps/api/tauri';

	enum State {
		Menu,
		Customers,
		Loans
	};

	let state = State.Menu;
	let data;

	function showMenu() {
		state = State.Menu;
	}

	function showCustomers() {
		state = State.Customers;
	}

	function showLoans() {
		state = State.Loans;
	}

	async function load_loans() {
		await invoke('get_loans', {})
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

<div transition:slide={{delay: 250, duration: 500}} class="p-5">
	{#if state === State.Menu}
		<div class="d-grid gap-2 col-6 mx-auto">
			<Button size="lg" color="primary" on:click={showCustomers}>Customers</Button>
			<Button size="lg" color="primary" on:click={showLoans}>Loans</Button>
		</div>
	{:else}
		{#if state === State.Customers}
			<h1>Customers</h1>
		{:else if state === State.Loans}
			<h1>Loans</h1>
		{/if}
		<Button color="primary" on:click={showMenu}><Icon name="x" /> Close</Button>
	{/if}
</div>
