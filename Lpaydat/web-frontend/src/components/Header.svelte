<script lang="ts">
	import { onMount } from "svelte";

	export let value: number;

	let bestScore: number = 0;

	onMount(() => {
		bestScore = Number(localStorage.getItem("best"));
	});

	$: if (bestScore < value) {
		localStorage.setItem("best", String(value));
	}
</script>

<style>
	.header {
		width: 555px;
		justify-content: space-between;
	}
	.row {
		display: flex;
		flex-direction: row;
		align-items: center;
	}
	.box {
		display: flex;
		flex-direction: column;
		background-color: #bbada0;
		padding: 15px 25px;
		font-weight: bold;
		border-radius: 3px;
		color: white;
		text-align: center;
		margin: 10px;
	}

	.text {
		font-size: 12px;
		color: #eee4da;
	}

	.value {
		font-size: 25px;
	}
	.new-game-button {
		background: #8f7a66;
    border-radius: 3px;
    padding: 0 20px;
    text-decoration: none;
    color: #f9f6f2;
    height: 40px;
    text-align: center;
		border: none;
		font-weight: bold;
	}
</style>

<div class="header row">
	<button on:click class="new-game-button">New Game</button>
	<div class="row">
		<div class="box">
			<div class="text">Score</div>
			<div class="value">{value}</div>
		</div>
		<div class="box">
			<div class="text">Best</div>
			<div class="value">{bestScore < value ? value : bestScore}</div>
		</div>
	</div>
</div>
