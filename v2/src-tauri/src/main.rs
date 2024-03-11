use chrono::{DateTime, Local};
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn time() -> String {
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%H:%M");
    format!("{}", custom_format)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, time])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
