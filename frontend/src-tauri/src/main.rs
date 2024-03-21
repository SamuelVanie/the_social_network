// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::sync::Mutex;
use env_logger::Env;
use std::collections::HashMap;

mod channel;

#[tauri::command]
fn say_hello(name: &str) -> String {
    format!("Hello {} and welcome to 0n1g1r1", name)
}

fn main() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "trace")
        .write_style_or("LOG_STYLE", "always");

    env_logger::init_from_env(env);

    tauri::Builder::default()
        .manage(channel::ChannelState {
            state: Mutex::new(HashMap::new()),
        })
        .invoke_handler(tauri::generate_handler![
            say_hello,
            channel::susbcribe_to_channel,
            channel::post_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
