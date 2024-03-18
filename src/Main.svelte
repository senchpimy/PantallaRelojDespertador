<script>
  import { onMount } from "svelte";
  import Clock from "./Clock.svelte";
  import Calendar from "./Calendar.svelte";
  import Lista from "./Lista.svelte";
  import { get_cal_data } from "./utils";

  onMount(() => {
    get_cal_data(true);
  });
</script>

<div class="division">
  <Calendar />
  <div class="rigth_column">
    <Clock />
    <Lista />
  </div>
</div>
