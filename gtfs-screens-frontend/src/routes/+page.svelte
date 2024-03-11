<script>

	import {onMount} from 'svelte';
	import * as lib from 'gtfs-screens-backend';

	onMount(async () => {
		await lib.default();
		lib.greet();
	});

</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<button on:click={() => lib.greet()}> Greet!!! </button>
