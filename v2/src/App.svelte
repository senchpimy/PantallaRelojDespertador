<script>
  import Clock from "./Clock.svelte";
  import Calendar from "./Calendar.svelte";
  import { onMount } from "svelte";
  let container;
  let currentSceneIndex = 1;
  onMount(() => {
    container = document.getElementById("scenes");
    container.style.translate = `translateX(calc(-100vw * ${currentSceneIndex}))`;

    container.addEventListener("mousedown", (e) => {
      isMouseDown = true;
      startX = e.clientX;
      currentX = startX;
    });

    container.addEventListener("mousemove", (e) => {
      if (!isMouseDown) return; // If mouse is not down, do nothing

      const deltaX = e.clientX - currentX;
      currentX = e.clientX;

      container.style.transform = `translateX(calc(-100vw * ${currentSceneIndex} + ${deltaX}px))`;
    });

    container.addEventListener("mouseup", () => {
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
    });
  });

  let startX = 0;
  let currentX = 0;

  let isMouseDown = false;
</script>

<main class="container">
  <div class="scenes" id="scenes">
    <div class="scene">
      <h1>Scenea 1</h1>
    </div>
    <div class="scene">
      <div class="division">
        <Calendar />
        <Clock />
      </div>
    </div>
    <div class="scene">
      <h1>Scenea 2</h1>
    </div>
  </div>
</main>
