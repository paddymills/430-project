<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { slide } from 'svelte/transition';
	import { Button, Card, Input, Spinner } from 'sveltestrap';

	import { invoke } from '@tauri-apps/api/tauri';

	const dispatch = createEventDispatcher();
	let show_valid = false;
	let submitting = false;

	let username = {
		value: "",
		min_len: 4,
		valid: false
	};
	let password = {
		value: "",
		min_len: 4,
		valid: false
	};

	function handleKeyPress(event) {
		if ( event.key == 'Enter' )
			submit();
	}

	async function submit() {
		submitting = true;
		
		await invoke('validate_login', { user: username.value, pwd: password.value })
			.then((result: {
				username: boolean,
				password: boolean
				is_admin: boolean
			}) => {
				// console.log(result);
				show_valid = true;

				username.valid = result.username;
				password.valid = result.password;

				if (username.valid && password.valid) {
					dispatch('loginSuccess', {
						username: username.value,
						admin: result.is_admin
					});
				}
			})
			.catch((error) => console.log(error))
			.finally(() => submitting = false);
	}
</script>

<div transition:slide={{delay: 250, duration: 500}}>
	<Card class="m-5 p-3 gap-3">
		<h3>loan system</h3>
		<Input
			id="username"
			placeholder="username"
			bind:value={username.value}
			on:keypress={handleKeyPress}
			valid={show_valid && username.valid}
			invalid={show_valid && !username.valid}
			feedback={username.valid ? null : "Invalid username"}
		/>
		<Input
			id="password"
			type="password"
			placeholder="password"
			bind:value={password.value}
			on:keypress={handleKeyPress}
			valid={show_valid && password.valid}
			invalid={show_valid && !password.valid}
			feedback={password.valid ? null : "Invalid password"}
		/>
		<Button color="primary" on:click={submit}>
			{#if submitting }
				<Spinner color="light" size="sm" />
			{:else}
				Log In
			{/if}
		</Button>
	</Card>
</div>
