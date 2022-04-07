<script lang="ts">
	import { onMount } from 'svelte';
	import { Button, Card, Input } from 'sveltestrap';

	import { invoke } from '@tauri-apps/api/tauri';

	let show_valid = false;
	let users: [String] = invoke('getUsernames');

	let username = {
		value: "",
		min_len: 4,
		valid: false,
		feedback: "Invalid username"
	};
	let password = {
		value: "",
		min_len: 4,
		valid: false,
		feedback: "Invalid password"
	};

	onMount(() => {
		console.log(users);
	});

	function validateInput() {
		username.valid = username.value.length >= username.min_len;
		password.valid = password.value.length >= password.min_len;
	}

	function submit() {
		show_valid = true;
	}
</script>

<Card class="m-5 p-3 gap-3">
	<Input
		id="username"
		placeholder="username"
		bind:value={username.value}
		on:input={validateInput}
		valid={show_valid && username.valid}
		invalid={show_valid && !username.valid}
		feedback={username.feedback}
	/>
	<Input
		id="password"
		type="password"
		placeholder="password"
		bind:value={password.value}
		on:input={validateInput}
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
