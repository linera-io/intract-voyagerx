<script>
  // You can define any additional logic or props here if needed
  const blockchainName = "Linera";
  const gameName = "2048";
</script>

<style>
  /* Add some basic styling */
  .sidebar {
    position: fixed;
    left: 0;
    top: 0;
    width: 400px;
    height: 100vh;
    background-color: transparent;
    padding: 20px;
    box-sizing: border-box;
    overflow-y: auto;
  }

  .header {
    font-size: 1.8em; /* Increased font size for more impact */
    font-weight: bold;
    margin-top: 1em;
    margin-bottom: 1.5em;
    color: #ff2a00; /* Changed color to gold for a more luxurious feel */
    text-shadow: 2px 2px 4px #000; /* Added text shadow for depth */
  }

  .introduction {
    font-size: 1.1em; /* Slightly larger font size */
    line-height: 1.6;
    color: #aaa; /* Darker color for better readability */
    font-style: italic; /* Italicized for emphasis */
  }
</style>

<div class="sidebar">
  <div class="header">{blockchainName} × {gameName}</div>
  <div class="introduction">
    <p>Welcome to <strong>{gameName}</strong>, a famous game developed on the <strong>{blockchainName}</strong> blockchain. This game exemplifies the integration of blockchain technology with gaming, offering a unique and efficient experience.</p>
    <p>The game leverages the speed and efficiency of the <strong>{blockchainName}</strong> blockchain to manage the game board and algorithm, ensuring a seamless and secure experience.</p>
    <p>We invite you to explore this seamless and secure gaming experience, which highlights the potential of blockchain in enhancing digital interactions.</p>
  </div>
</div>
