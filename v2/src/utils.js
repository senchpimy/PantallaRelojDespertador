import { invoke } from "@tauri-apps/api/tauri";
let DATA = null;

export async function get_cal_data(update) {
  if (DATA && update === false) {
    console.log("NO NUEVAS SOLICITUDES")
    return DATA.events;
  }
  console.log("NUEVAS SOLICITUDES")
  let str = await invoke("get_cal_data", {});
  DATA = JSON.parse(str);
  return DATA.events
}

