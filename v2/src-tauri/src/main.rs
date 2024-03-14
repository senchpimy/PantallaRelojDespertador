use chrono::{DateTime, Local};
use reqwest;
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

#[tauri::command]
fn get_cal_data() -> String {
    let json_response = r#"{"error": {"code": 500,"message": "Internal Server Error"}}"#;
    let resp = reqwest::blocking::get("http://localhost:8080");
    match resp {
        Ok(val) => val.text().unwrap(),
        Err(_) => String::from(json_response),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, time, get_cal_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
