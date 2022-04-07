<script lang="ts">
	import { Card, Styles } from 'sveltestrap';
	import LoginForm from './components/Login.svelte';
</script>

<main class="d-flex flex-column align-items-center p-5">
	<Card class="p-3">
		<LoginForm />
	</Card>
</main>

<Styles />