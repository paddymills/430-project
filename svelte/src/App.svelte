<script lang="ts">
	import { Card, Styles } from 'sveltestrap';

	export let name: string;
</script>

<main class="d-flex flex-column align-items-center p-5">
	<Card class="p-3">
		<p class="m-1">{name}</p>
	</Card>
</main>

<Styles />