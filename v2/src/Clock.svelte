<script>
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let currentTime = "";

  async function time() {
    currentTime = await invoke("time", {});
  }

  const updateClock = () => {
    time();
  };

  const interval = setInterval(updateClock, 1000);

  onMount(() => {
    updateClock();
  });

  onDestroy(() => {
    clearInterval(interval);
  });

  function handleClick() {
    document.getElementById("clock").classList.toggle("clock_expanded");
    document.getElementById("calendar").classList.toggle("calendar_reduced");
    document.getElementById("lista").classList.toggle("lista_reduced");
  }
</script>

<div id="clock" class="clock" on:click={handleClick}>
  {currentTime}
</div>
