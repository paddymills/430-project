<script lang="ts">
	import { Button, FormGroup, Label, Input } from 'sveltestrap';

	let username: String;
	let password: String;
</script>

<Input
	id="username"
	placeholder="Username"
	bind:value={username}
/>
<Input
	id="password"
	type="password"
	placeholder="password"
	bind:value={password}
/>
<Button>Log In</Button>
