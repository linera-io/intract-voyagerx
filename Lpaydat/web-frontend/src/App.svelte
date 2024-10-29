<script lang="ts">
  const chainId = 'e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65';
  const applicationId = 'c10aebc7f0e362a1cf7872096810467ea10abef0345ed6146a9cea58a46c999fa5f66bf0ead0fcb0558c6849a6e53731b9a14578cb3c2134fd593514bae63891e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65df0000000000000000000000';
  const port = '8080';

  import { setContextClient } from '@urql/svelte';
  import { getClient } from './lib/client';
  import Game from './components/Game.svelte';

  const client = getClient(chainId, applicationId, port);
  setContextClient(client);
</script>

<style>
  main {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    flex-direction: column;
    background-color: transparent;
  }

  h1 {
    font-size: 80px;
    margin: 0;
    padding-bottom: 20px;
    color: #776e65; /* Darker text color */
  }

  .game-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    max-width: 556px;
    margin: 0 auto;
    background-color: #bbada0; /* Game board background */
    padding: 20px;
    border-radius: 10px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
  }

  .credits {
    font-size: 16px;
    font-weight: bold;
    color: #776e65;
  }

  .credits .heart {
    color: red; /* Make the heart emoji red */
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    max-width: 556px;
    margin: 0 auto;
  }

  .title {
    font-size: 80px;
    margin: 0;
    padding-bottom: 20px;
    color: #776e65; /* Darker text color */
  }
</style>

<main>
  <div class="header">
    <h1 class="title">2048</h1>
    <span class="credits">made with <span class="heart">❤️</span> lpaydat</span>
  </div>
  <div class="game-container">
    <Game />
  </div>
</main>
