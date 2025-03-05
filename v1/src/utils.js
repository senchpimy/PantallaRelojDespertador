import { invoke } from "@tauri-apps/api/tauri";
let DATA = null;

export const DATA_TIMEOUT = 1000 * 60 * 5;

export async function get_cal_data() {
  let str = await invoke("get_cal_data", {});
  DATA = JSON.parse(str);
  return DATA.events
}

