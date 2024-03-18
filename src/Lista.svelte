<script>
  import { onMount } from "svelte";
  import { get_cal_data } from "./utils";
  var ElementsList = [];
  async function get_list() {
    ElementsList = await get_cal_data(false);
    for (let i = ElementsList.length - 1; i >= 0; i--) {
      ElementsList[i].color += "26"; //RGBA
      if (ElementsList[i].type === "recurring") {
        ElementsList.splice(i, 1);
      } else {
        if ("completed" in ElementsList[i] && ElementsList[i].completed) {
          ElementsList[i].ready = true;
        }
        if (!("date" in ElementsList[i])) {
          ElementsList[i].date = ElementsList[i].start.split("T")[0];
        }
      }
    }
  }
  get_list();
  function focus(node) {
    node.addEventListener("click", function () {
      var r = node.querySelector(".info");
      r.classList.toggle("info-show");
    });
  }
  function color(node) {
    var r = node.querySelector("#point");
    console.log(r);
  }
</script>

<div id="lista" class="lista">
  <ul use:color>
    {#each ElementsList as element}
      <li style="background-color:{element.color}" use:focus>
        {#if "ready" in element}
          <div class="lista_element cumplido">
            {element.title}
          </div>
        {:else}
          <div class="lista_element">
            {element.title}
          </div>
        {/if}
        {element.date}
        <div class="info">
          {element.info}
        </div>
      </li>
    {/each}
  </ul>
</div>
