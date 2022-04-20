<script lang="ts">
	import { Dropdown, DropdownToggle, DropdownMenu, DropdownItem } from 'sveltestrap';
	import { Styles, Icon } from 'sveltestrap';

	import LoginForm from './components/Login.svelte';
	import CustomerView from './components/CustomerView.svelte';
	import AdminView from './components/AdminView.svelte';
	// import Table from './components/Table.svelte';

	let loggedIn = false;
	let isAdmin = false;
	let username = null;

	function login(event) {
		loggedIn = true;

		// console.log(event);

		username = event.detail.username;
		isAdmin = event.detail.admin;
	}

	//testing
	loggedIn = true;
	username = "cust2";
</script>

<main class="d-flex flex-column align-items-center p-5">
	{#if !loggedIn}
		<LoginForm on:loginSuccess="{ login }" />
	{:else}
		<div class="w-100 m-3 p-3 border-bottom d-flex justify-content-between">
			<h4>{ isAdmin ? "Admin View" : "Customer View" }</h4>
			<Dropdown>
				<DropdownToggle caret outline color="primary" size="sm"><Icon name="person-circle" /></DropdownToggle>
				<DropdownMenu end>
					<DropdownItem header>{ username }</DropdownItem>
					<DropdownItem on:click={ () => loggedIn = false }>Logout</DropdownItem>
				</DropdownMenu>
			</Dropdown>
		</div>

		{#if isAdmin}
			<AdminView />
		{:else}
			<CustomerView {username} />
		{/if}
	{/if}
</main>

<Styles />