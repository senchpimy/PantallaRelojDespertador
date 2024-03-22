<script>
  import { onMount } from "svelte";
  import Main from "./Main.svelte";
  import Config from "./Config.svelte";
  let container;
  let currentSceneIndex = 0;
  let scenes = [Main, Config];
  onMount(() => {
    container = document.getElementById("scenes");
    let len = 99 * scenes.length;
    container.style.width = `${len}vw`;
    console.log(container);

    container.addEventListener("mousedown", (e) => {
      isMouseDown = true;
      startX = e.clientX;
      currentX = startX;
    });

    container.addEventListener("touchstart", (e) => {
      startX = e.touches[0].clientX;
      isMouseDown = true;
      currentX = startX;
    });

    container.addEventListener("mousemove", (e) => {
      if (!isMouseDown) return; // If mouse is not down, do nothing

      const deltaX = e.clientX - currentX;
      currentX = e.clientX;

      container.style.transform = `translateX(calc(-100vw * ${currentSceneIndex} + ${deltaX}px))`;
    });

    container.addEventListener("touchmove", (e) => {
      if (!isMouseDown) return;

      const deltaX = e.touches[0].clientX - currentX;
      currentX = e.touches[0].clientX;

      container.style.transform = `translateX(calc(-100vw * ${currentSceneIndex} + ${deltaX}px))`;
    });

    container.addEventListener("touchend", () => {
      handleUp();
    });

    container.addEventListener("mouseup", () => {
      handleUp();
    });
  });

  function handleUp() {
    if (!isMouseDown) return; // If mouse was not down, do nothing
    isMouseDown = false;

    const deltaX = currentX - startX;

    if (deltaX > 100 && currentSceneIndex > 0) {
      // Swipe left
      currentSceneIndex--;
      container.style.transform = `translateX(calc(-100vw * ${currentSceneIndex}))`;
    } else if (deltaX < -100 && currentSceneIndex < 2) {
      // Swipe right
      currentSceneIndex++;
      container.style.transform = `translateX(calc(-100vw * ${currentSceneIndex}))`;
    } else {
      // Return to original position
      container.style.transform = `translateX(calc(-100vw * ${currentSceneIndex}))`;
    }
  }

  let startX = 0;
  let currentX = 0;

  let isMouseDown = false;
</script>

<main class="container">
  <div class="scenes" id="scenes">
    {#each scenes as ElementListComponent}
      <div class="scene">
        <ElementListComponent />
      </div>
    {/each}
  </div>
</main>
