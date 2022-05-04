<script lang="ts">
	import { slide } from 'svelte/transition';
	import { Alert, Button, Icon, Spinner } from 'sveltestrap';

	import { invoke } from '@tauri-apps/api/tauri';
	import CustomerTable from './tables/CustomerTable.svelte';
	import LoanTable from './tables/LoanTable.svelte';

	const State = {
		Menu: 'menu',
		Customers: 'customer',
		Loans: 'loan'
	};
	
	let state = State.Menu;
	let alertText;
	let alertColor;
	let showAlert = false;
	let changeKey = 1;

	const showMenu = () => state = State.Menu;
	const showCustomers = () => state = State.Customers;
	const showLoans = () => state = State.Loans;

	async function loadData() {
		return await invoke(`get_${state}s`, {})
			.then((result: [any]) => {
				// console.log(result);

				return result;
			})
			.catch((error) => {
				console.log(error);

				return error
			});
	}

	function handleAlert(event) {
		alertText = event.detail.message;
		
		if (event.detail.isError ){
			alertColor = 'danger';
		} else {
			alertColor = 'success';
		}

		setTimeout(() => (showAlert = false), 5000);

		showAlert = true;
	}
</script>

<div transition:slide={{delay: 250, duration: 500}} class="p-5">
	<div class="d-flex flex-column align-items-center gap-2">
		{#if state === State.Menu}
			<Button size="lg" color="primary" on:click={showCustomers}>Customers</Button>
			<Button size="lg" color="primary" on:click={showLoans}>Loans</Button>
		{:else}
			{#key changeKey}
			{#await loadData()}
				<h4><Spinner color="primary" />Loading Loans</h4>
			{:then data}
				{#if state === State.Customers}
					<CustomerTable {data} on:alert={handleAlert} on:refresh={() => changeKey += 1} />
				{:else if state === State.Loans}
					<LoanTable {data} on:alert={handleAlert} on:refresh={() => changeKey += 1} />
				{/if}
			{:catch error}
				<Alert color="danger">
					<h1>Data failure</h1>
					Failed to load {state} data. { error }
				</Alert>
			{/await}
			{/key}
			<Button color="primary" on:click={showMenu}><Icon name="x" /> Close</Button>
		{/if}
	</div>
    <Alert class="m-5" color={alertColor} isOpen={showAlert} toggle={() => (showAlert = false)}>{ alertText }</Alert>
</div>