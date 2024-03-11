<script>
  import { onMount } from "svelte";
  import { Calendar } from "fullcalendar";

  function cal() {
    const calendarEl = document.getElementById("calendar");
    const calendar = new Calendar(calendarEl, {
      initialView: "dayGridMonth",

      selectable: true,
      select: function (selected) {
        console.log(selected.start);
      },

      events: [
        {
          title: "My Event",
          start: "2024-03-01",
        },
      ],

      eventMouseEnter: function (info) {
        info.el.style.borderColor = "red"; //TODO cambiar Colores
      },

      eventMouseLeave: function (info) {
        info.el.style.borderColor = "blue";
      },

      eventClick: function (info) {
        info.el.style.borderColor = "red";
        console.log("AAAAAA");
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
