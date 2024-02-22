// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::read;



#[tauri::command]
fn say_hello(name: &str) -> String {
  format!("Hello {} and welcome to 0n1g1r1", name)
}

#[tauri::command]
fn icon() -> Vec<u8> {
  let local_img = if let Ok(data) = read("/home/niels/onigiri/the_social_network/frontend/src-tauri/src/blanchard.png") {
    data
  } else {
    panic!();
  };
  local_img
}

#[derive(serde::Serialize)]
struct Message {
  message: String,
  author: String,
}

#[tauri::command]
async fn get_message() -> Result<Message, String> {
  let m: Message = Message {
    message: String::from("Bonjour."),
    author: String::from("origaniels")
  };
  Ok(m)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![say_hello, icon, get_message])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
