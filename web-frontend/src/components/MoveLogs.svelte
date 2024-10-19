<script lang="ts">
  export let hashes: { hash: string, timestamp: string }[] = [];

  // Function to format the timestamp
  function formatTimestamp(isoString: string): string {
    const date = new Date(isoString);
    return date.toLocaleString(); // Adjust this to your preferred format
  }

  // Function to format the hash
  function formatHash(hash: string): string {
    return `${hash.slice(0, 6)}...${hash.slice(-6)}`;
  }
</script>

<style>
  .sidebar {
    width: 280px;
    background-color: #2d2d2d;
    padding: 20px;
    box-shadow: 2px 0 5px rgba(0, 0, 0, 0.1);
    position: fixed;
    top: 0;
    right: 0;
    height: 100%;
    overflow-y: auto;
  }

  .log-item {
    margin-bottom: 15px;
    padding: 10px;
    background-color: #343434;
    border-radius: 5px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .log-title {
    font-weight: bold;
    margin-bottom: 5px;
  }
</style>

<div class="sidebar">
  <div class="log-title">Blockhashes</div>
  {#each hashes as { hash, timestamp }}
    <div class="log-item">
      <div><strong>Hash:</strong> {formatHash(hash)}</div>
      <div>{formatTimestamp(timestamp)}</div>
    </div>
  {/each}
</div>
