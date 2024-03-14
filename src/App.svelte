<script>
  import { onMount } from "svelte";
  import Main from "./Main.svelte";
  let container;
  let currentSceneIndex = 1;
  let scenes = [Main];
  onMount(() => {
    container = document.getElementById("scenes");
    let len = scenes.length;
    container.style.width = `100 * ${len}vw`;
    //container.style.translate = `translateX(calc(-100vw * ${currentSceneIndex}))`;

    //container.addEventListener("mousedown", (e) => {
    //  isMouseDown = true;
    //  startX = e.clientX;
    //  currentX = startX;
    //});

    //container.addEventListener("mousemove", (e) => {
    //  if (!isMouseDown) return; // If mouse is not down, do nothing

    //  const deltaX = e.clientX - currentX;
    //  currentX = e.clientX;

    //  container.style.transform = `translateX(calc(-100vw * ${currentSceneIndex} + ${deltaX}px))`;
    //});

    //container.addEventListener("mouseup", () => {
    //  if (!isMouseDown) return; // If mouse was not down, do nothing
    //  isMouseDown = false;

    //  const deltaX = currentX - startX;

    //  if (deltaX > 100 && currentSceneIndex > 0) {
    //    // Swipe left
    //    currentSceneIndex--;
    //    container.style.transform = `translateX(calc(-100vw * ${currentSceneIndex}))`;
    //  } else if (deltaX < -100 && currentSceneIndex < 2) {
    //    // Swipe right
    //    currentSceneIndex++;
    //    container.style.transform = `translateX(calc(-100vw * ${currentSceneIndex}))`;
    //  } else {
    //    // Return to original position
    //    container.style.transform = `translateX(calc(-100vw * ${currentSceneIndex}))`;
    //  }
    //});
  });

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
