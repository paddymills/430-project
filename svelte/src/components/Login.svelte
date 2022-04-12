<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { slide } from 'svelte/transition';
	import { Button, Card, Input } from 'sveltestrap';

	import { invoke } from '@tauri-apps/api/tauri';

	const dispatch = createEventDispatcher();
	let show_valid = false;

	let username = {
		value: "",
		min_len: 4,
		valid: true
	};
	let password = {
		value: "",
		min_len: 4,
		valid: true
	};

	async function submit() {
		show_valid = true;

		await invoke('validate_login', { user: username.value, pwd: password.value })
			.then((result: { username: boolean, password: boolean }) => {
				username.valid = result.username;
				password.valid = result.password;

				if (username.valid && password.valid) {
					dispatch('loginSuccess', username.value);
				}
			})
			.catch((error) => console.log(error));
	}
</script>

<div transition:slide={{delay: 250, duration: 500}}>
	<Card class="m-5 p-3 gap-3">
		<Input
			id="username"
			placeholder="username"
			bind:value={username.value}
			valid={show_valid && username.valid}
			invalid={show_valid && !username.valid}
			feedback={username.valid ? null : "Invalid username"}
		/>
		<Input
			id="password"
			type="password"
			placeholder="password"
			bind:value={password.value}
			valid={show_valid && password.valid}
			invalid={show_valid && !password.valid}
			feedback={password.valid ? null : "Invalid password"}
		/>
		<Button
			color="primary"
			on:click={submit}
		>Log In</Button>
	</Card>
</div>
