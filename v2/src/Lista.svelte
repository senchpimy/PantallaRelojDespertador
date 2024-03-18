<script>
  import { get_cal_data } from "./utils";
  var ElementsList = [];
  async function get_list() {
    ElementsList = await get_cal_data(false);
  }
  get_list();
</script>

<div id="lista" class="lista">
  <ul>
    {#each ElementsList as element}
      <li>
        <h4>{element.title}</h4>
        <p>Texto</p>
      </li>
    {/each}
  </ul>
</div>
