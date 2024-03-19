<script>
  import { onDestroy } from "svelte";
  import { get_cal_data, DATA_TIMEOUT } from "./utils";
  let ElementsList = [];

  async function get_list() {
    ElementsList = await get_cal_data();
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
        const millis = Date.parse(ElementsList[i].date) - Date.now();

        const days = Math.floor(millis / (1000 * 60 * 60 * 24)) + 1;
        console.log(days);
        ElementsList[i].days_left = days;
        if (ElementsList[i].completed && days <= 0) {
          ElementsList.splice(i, 1);
        }
      }
    }

    ElementsList.sort((a, b) => a.days_left - b.days_left);
  }

  get_list();
  const updateData = async () => {
    await get_list();
    ElementsList = [...ElementsList];
  };

  const interval = setInterval(updateData, DATA_TIMEOUT);

  onDestroy(() => {
    clearInterval(interval);
  });

  function focus(node) {
    node.addEventListener("click", function () {
      var r = node.querySelector(".info");
      r.classList.toggle("info-show");
    });
  }
</script>

<div id="lista" class="lista">
  <ul id="list-holder">
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
        <div style="display: flex;  justify-content: space-between;">
          {element.date}
          <div style="text-align: right;">{element.days_left} days left</div>
        </div>
        <div class="info info-show">
          {element.info}
        </div>
      </li>
    {/each}
  </ul>
</div>
