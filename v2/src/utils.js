import { invoke } from "@tauri-apps/api/tauri";
let DATA = null;

export async function get_cal_data(update) {
  if (DATA && !update) {
    return DATA;
  }
  let str = await invoke("get_cal_data", {});
  DATA = JSON.parse(str);
  return DATA.events
}

