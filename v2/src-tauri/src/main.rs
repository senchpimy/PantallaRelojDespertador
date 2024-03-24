use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use reqwest;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::{
    ops::{Deref, DerefMut},
    sync::RwLock,
};

static PATH: &str = "/home/plof/.pantallareloj";

#[derive(Debug, Serialize, Deserialize)]
struct AppCal {
    port: String,
    server: String,
}

impl AppCal {
    fn new() -> Self {
        let file = File::open(PATH);
        match file {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                serde_json::from_str(&contents).unwrap()
            }
            Err(a) => {
                println!("ERROR: {}", a);
                Self {
                    port: String::from("8000"),
                    server: String::from("127.0.0.1"),
                }
            }
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
fn save_data(str: String) {
    let app: AppCal = serde_json::from_str(&str).unwrap();
    let mut writer = GLOBAL_VAR.write().unwrap();
    let r = writer.deref_mut();
    *r = app;
    fs::write(PATH, &str).expect("Unable to write file");
}

#[tauri::command]
fn get_data() -> String {
    let reader = GLOBAL_VAR.read().unwrap();
    let tmp = reader.deref();
    serde_json::to_string(&tmp).unwrap()
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
    app.invoke_handler(tauri::generate_handler![
        time,
        get_cal_data,
        save_data,
        get_data
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
