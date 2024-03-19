use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use reqwest;
use std::sync::RwLock;

#[derive(Debug)]
struct AppCal {
    port: String,
    sidd: String,
    server: String,
}

impl AppCal {
    fn new() -> Self {
        Self {
            port: String::from("8080"),
            sidd: String::new(),
            server: String::from("localhost"),
        }
    }
}

lazy_static! {
    static ref GLOBAL_VAR: RwLock<AppCal> = RwLock::new(AppCal::new());
}

#[tauri::command]
fn time() -> String {
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%H:%M");
    format!("{}", custom_format)
}

#[tauri::command]
fn save_data(server: String, port: String) {
    let mut writer = GLOBAL_VAR.write().unwrap();
    writer.server = server;
    writer.port = port;
}

#[tauri::command]
fn get_cal_data() -> String {
    let reader = GLOBAL_VAR.read().unwrap();
    let json_response = r#"{"error": {"code": 500,"message": "Internal Server Error"}}"#;
    let resp = reqwest::blocking::get(format!("http://{}:{}", reader.server, reader.port));
    match resp {
        Ok(val) => val.text().unwrap(),
        Err(_) => String::from(json_response),
    }
}

fn main() {
    let app = tauri::Builder::default();
    app.invoke_handler(tauri::generate_handler![time, get_cal_data, save_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
