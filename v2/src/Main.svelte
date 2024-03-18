<script>
  import { onMount, onDestroy } from "svelte";
  import Clock from "./Clock.svelte";
  import Calendar from "./Calendar.svelte";
  import Lista from "./Lista.svelte";
  import { get_cal_data } from "./utils";

  const updateData = () => {
    get_cal_data(true);
  };

  const interval = setInterval(updateData, 10000);

  onMount(() => {
    get_cal_data(true);
  });

  onDestroy(() => {
    clearInterval(interval);
  });
</script>

<div class="division">
  <Calendar />
  <div class="rigth_column">
    <Clock />
    <Lista />
  </div>
</div>
