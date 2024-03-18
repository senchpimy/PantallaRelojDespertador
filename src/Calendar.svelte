<script>
  import { onMount, onDestroy } from "svelte";
  import { Calendar } from "fullcalendar";
  import { get_cal_data, DATA_TIMEOUT } from "./utils";

  const data = { events: [] };
  const calendar = { cal: null, element: null };

  function crearCal(calendarEl) {
    return new Calendar(calendarEl, {
      initialView: "dayGridMonth",

      selectable: true,
      select: function (selected) {
        //console.log(selected.start);
      },

      events: data.events,

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
  }

  const updateData = async () => {
    data.events = await get_cal_data();
    data.events.forEach((element) => {
      if ("completed" in element) {
        if (element.completed) {
          element.color = "red";
        }
      }
    });
    calendar.cal = crearCal(calendar.element);
    calendar.cal.render();
  };

  const interval = setInterval(updateData, DATA_TIMEOUT);

  onDestroy(() => {
    clearInterval(interval);
  });

  async function cal() {
    data.events = await get_cal_data();
    data.events.forEach((element) => {
      if ("completed" in element) {
        if (element.completed) {
          element.color = "red";
        }
      }
    });
    calendar.cal = crearCal(calendar.element);
    calendar.cal.render();
  }

  onMount(() => {
    calendar.element = document.getElementById("calendar");
    cal();
  });
</script>

<div class="calendar-container" id="calendar"></div>
