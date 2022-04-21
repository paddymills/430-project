<script lang="ts">
	import { slide } from 'svelte/transition';
	import { Alert, Button, Icon, Spinner } from 'sveltestrap';

	import { invoke } from '@tauri-apps/api/tauri';
	import CustomerTable from './CustomerTable.svelte';
	import LoanTable from './LoanTable.svelte';

	enum State {
		Menu,
		Customers,
		Loans
	};
	
	let state = State.Menu;
	// let data;

	const showMenu = () => state = State.Menu;
	const showCustomers = () => state = State.Customers;
	const showLoans = () => state = State.Loans;

	async function load_customers() {
		return await invoke('get_customers', {})
			.then((result: [any]) => {
				console.log(result);

				return result;
			})
			.catch((error) => {
				console.log(error);

				return error
			});
	}
</script>

<div transition:slide={{delay: 250, duration: 500}} class="p-5">
	<div class="d-flex flex-column align-items-center gap-2">
		{#if state === State.Menu}
			<Button size="lg" color="primary" on:click={showCustomers}>Customers</Button>
			<Button size="lg" color="primary" on:click={showLoans}>Loans</Button>
		{:else}
			{#if state === State.Customers}
				{#await load_customers()}
					<Spinner color="primary" />
				{:then data}
					<CustomerTable {data} />
				{:catch error}
					<Alert color="danger">
						<h1>Data failure</h1>
						Failed to load customer data. { error }
					</Alert>
				{/await}
			{:else if state === State.Loans}
				<h1>Loans</h1>
			{/if}
			<Button color="primary" on:click={showMenu}><Icon name="x" /> Close</Button>
		{/if}
	</div>
</div>