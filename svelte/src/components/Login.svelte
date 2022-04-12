<script lang="ts">
	import { Button, Card, Input } from 'sveltestrap';

	import { invoke } from '@tauri-apps/api/tauri';

	let show_valid = false;

	let username = {
		value: "",
		min_len: 4,
		valid: true,
		feedback: null
	};
	let password = {
		value: "",
		min_len: 4,
		valid: true,
		feedback: "Invalid password"
	};

	async function submit() {
		show_valid = true;

		await invoke('validate_login', { username: username, password: password })
			.then((result: { username: boolean, password: boolean }) => {
				username.valid = result.username;
				password.valid = result.password;
			})
			.catch((error) => console.log(error));
	}
</script>

<Card class="m-5 p-3 gap-3">
	<Input
		id="username"
		placeholder="username"
		bind:value={username.value}
		valid={show_valid && username.valid}
		invalid={show_valid && !username.valid}
		feedback={username.feedback}
	/>
	<Input
		id="password"
		type="password"
		placeholder="password"
		bind:value={password.value}
		valid={show_valid && password.valid}
		invalid={show_valid && !password.valid}
		feedback={password.feedback}
	/>
	<Button
		color="primary"
		disabled={ !(username.valid && password.valid) }
		on:click={submit}
	>Log In</Button>
</Card>
