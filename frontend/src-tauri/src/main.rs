// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eventsource::reqwest::Client;
use std::collections::HashMap;
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use env_logger::Env;

#[tauri::command]
fn say_hello(name: &str) -> String {
    format!("Hello {} and welcome to 0n1g1r1", name)
}

#[derive(serde::Serialize, Clone, Debug)]
enum MessageType {
    CONNECT,
    SEND,
    QUIT,
    NONE,
}

#[derive(serde::Serialize, Clone, Debug)]
struct Message {
    room_id: u32,
    message_type: MessageType,
    message_content: String,
}

#[derive(serde::Serialize)]
enum ResultCode {
    ERROR,
    SUCCESS,
}

#[derive(serde::Serialize)]
struct OperationResult {
    content: String,
    code: ResultCode,
}

// Struct that will store the Cancellationtokens
// corresponding to the different channels that the client
// subscribed to
struct ChannelState {
    state: Mutex<HashMap<i32, CancellationToken>>,
}

#[tauri::command]
async fn susbcribe_to_channel(
    channel_id: i32,
    app: AppHandle,
    state: State<'_, ChannelState>,
) -> Result<OperationResult, String> {

    log::trace!("Subscribe function called");
    let mut tokens = state.state.lock().await;

    let token: Option<&CancellationToken> = tokens.get(&channel_id);

    if token.is_some() {
        log::warn!(
            "{}",
            format!(
                "Client is already listening for this channel's {} message",
                channel_id
            )
        );
        return Ok(OperationResult {
            content: format!("Already subscribed to channel: {}", channel_id),
            code: ResultCode::ERROR,
        });
    }

    tokens.insert(channel_id, CancellationToken::new());
    let token = tokens.get(&channel_id);

    let url =
        reqwest::Url::parse(format!("http://127.0.0.1:8000/subscribe/{}", channel_id).as_str())
            .unwrap();

    let client = Client::new(url);

    for event in client {
        if token.expect("Token is invalid").is_cancelled() {
            break;
        }
        let message = event.unwrap().data;
        app.emit_all("new_message", message).unwrap();
    }

    return Ok(OperationResult {
        content: format!(
            "Stopped listening for messages from channel: {}",
            channel_id
        ),
        code: ResultCode::SUCCESS,
    });
}

#[tauri::command]
async fn post_message(author: String, content: String) -> () {
    println!("{}: {}", author, content);
}

fn main() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "trace")
        .write_style_or("LOG_STYLE", "always");

    env_logger::init_from_env(env);

    tauri::Builder::default()
        .manage(ChannelState { state: Mutex::new(HashMap::new()) })
        .invoke_handler(tauri::generate_handler![
            say_hello,
            susbcribe_to_channel,
            post_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
