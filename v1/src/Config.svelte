<script>
  import Keyboard from "simple-keyboard";
  import "simple-keyboard/build/css/index.css";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  let config_obj = null;

  async function get_data() {
    let str = await invoke("get_data", {});
    try {
      config_obj = JSON.parse(str);
      for (const property in config_obj) {
        let r = document.getElementById(property);
        r.value = config_obj[property];
      }
    } catch {}
  }

  async function save_data() {
    for (const property in config_obj) {
      let r = document.getElementById(property);
      config_obj[property] = r.value;
    }
    let str = JSON.stringify(config_obj);
    await invoke("save_data", { str });
  }

  get_data();

  const keyboardPointer = { pointer: null, writer_id: null, obj_pointer: null };
  function hideKb() {
    if (!keyboardPointer.pointer.classList.contains("hide-kb")) {
      keyboardPointer.pointer.classList.toggle("hide-kb");
    }
  }

  function showKb() {
    if (keyboardPointer.pointer.classList.contains("hide-kb")) {
      keyboardPointer.pointer.classList.toggle("hide-kb");
    }
  }

  onMount(() => {
    document.getElementById("save-btn").addEventListener("click", () => {
      save_data();
    });
    keyboardPointer.obj_pointer = new Keyboard({
      onKeyPress: (button) => onKeyPress(button),
      theme: "hg-theme-default myTheme1",
      useTouchEvents: true,
      layout: {
        default: [
          "1 2 3 4 5 6 7 8 9 0 {bksp}",
          "q w e r t y u i o p",
          "a s d f g h j k l ; HIDE",
          "z x c v b n m .",
        ],
      },
    });

    function onKeyPress(button) {
      if (button === "HIDE") {
        hideKb();
        return;
      }
      let val = document.getElementById(keyboardPointer.writer_id);
      if (button === "{bksp}") {
        val.value = val.value.slice(0, -1);
        return;
      }
      val.value += button;
    }

    keyboardPointer.pointer = document.getElementById("keyboard");
    keyboardPointer.pointer.classList.toggle("hide-kb");
  });

  function focusKeyboard(element) {
    element.addEventListener("focus", () => {
      keyboardPointer.writer_id = element.id;
      showKb();
    });
  }
</script>

<h1>Config</h1>
<div id="config-container">
  <div>
    <h1>server</h1>
    <input id="server" use:focusKeyboard />
  </div>

  <div>
    <h1>port</h1>
    <input id="port" use:focusKeyboard />
  </div>

  <button id="save-btn">Guardar</button>
  <button onClick="window.location.reload();">Reload</button>
</div>
<div class="texto">
  <div class="simple-keyboard hide-kb" id="keyboard"></div>
</div>
