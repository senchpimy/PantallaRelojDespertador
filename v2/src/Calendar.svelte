<script>
  import { onMount } from "svelte";
  import { Calendar } from "fullcalendar";
  import { invoke } from "@tauri-apps/api/tauri";

  async function get_cal_data() {
    let str = await invoke("get_cal_data", {});
    let cal_data = JSON.parse(str);
    return cal_data.events;
  }

  async function cal() {
    let events = await get_cal_data();
    console.log(events);
    const calendarEl = document.getElementById("calendar");
    const calendar = new Calendar(calendarEl, {
      initialView: "dayGridMonth",

      selectable: true,
      select: function (selected) {
        console.log(selected.start);
      },

      events: events,

      eventMouseEnter: function (info) {
        //info.el.style.borderColor = "red"; //TODO cambiar Colores
      },

      eventMouseLeave: function (info) {
        //info.el.style.borderColor = "blue";
      },

      eventClick: function (info) {
        //info.el.style.borderColor = "red";
      },

      customButtons: {
        myCustomButton: {
          text: "custom!",
          click: function () {
            alert("clicked the custom button!");
          },
        },
      },

      headerToolbar: {
        left: "prev,next today myCustomButton",
        center: "title",
        right: "dayGridMonth,timeGridWeek,timeGridDay",
      },
    });
    calendar.render();
  }

  onMount(() => {
    cal();
  });
</script>

<div class="calendar-container" id="calendar"></div>
