<script lang="ts">
	import { Styles } from 'sveltestrap';

	import LoginForm from './components/Login.svelte';
	import CustomerView from './components/CustomerView.svelte';
	import AdminView from './components/AdminView.svelte';
	// import Table from './components/Table.svelte';

	let loggedIn = false;
	let isAdmin = false;
</script>

<main class="d-flex flex-column align-items-center p-5">
	{#if !loggedIn}
		<LoginForm on:loginSuccess="{ () => loggedIn = true }" />
	{:else if isAdmin}
		<AdminView />
	{:else}
		<CustomerView />
	{/if}
</main>

<Styles />