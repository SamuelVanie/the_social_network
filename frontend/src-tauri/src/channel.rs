use eventsource::reqwest::Client;
use std::collections::HashMap;
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

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
pub struct OperationResult {
    content: String,
    code: ResultCode,
}

// Struct that will store the Cancellationtokens
// corresponding to the different channels that the client
// subscribed to
pub struct ChannelState {
    pub state: Mutex<HashMap<i32, CancellationToken>>,
}

fn verify_user_isnt_listening(
    channel_id: i32,
    tokens: &HashMap<i32, CancellationToken>,
) -> Result<(), String> {
    let token: Option<&CancellationToken> = tokens.get(&channel_id);

    if token.is_some() {
        return Err(format!(
            "Client is already listening for this channel's {} message",
            channel_id
        ));
    }

    Ok(())
}

fn listen_and_emit_messages(channel_id: i32, app: AppHandle, token: CancellationToken) {
    let url =
        reqwest::Url::parse(format!("http://127.0.0.1:8000/subscribe/{}", channel_id).as_str())
            .unwrap();

    let client = Client::new(url);

    for event in client {
        if token.is_cancelled() {
            break;
        }
        let message = event.unwrap().data;
        app.emit_all("new_message", message).unwrap();
    }
}

#[tauri::command]
pub async fn susbcribe_to_channel(
    channel_id: i32,
    app: AppHandle,
    state: State<'_, ChannelState>,
) -> Result<OperationResult, OperationResult> {
    log::trace!("Subscribe function called");
    let mut tokens = state.state.lock().await;

    match verify_user_isnt_listening(channel_id, &tokens) {
        Ok(_) => {}
        Err(e) => {
            return Err(OperationResult {
                content: e,
                code: ResultCode::ERROR,
            });
        }
    }

    tokens.insert(channel_id, CancellationToken::new());

    let token = tokens.get(&channel_id);
    listen_and_emit_messages(
        channel_id,
        app,
        token
            .expect(format!("Token registered for channel {} is invalid", channel_id).as_str())
            .clone(),
    );

    return Ok(OperationResult {
        content: format!(
            "Stopped listening for messages from channel: {}",
            channel_id
        ),
        code: ResultCode::SUCCESS,
    });
}

#[tauri::command]
pub async fn post_message(author: String, content: String) -> () {
    println!("{}: {}", author, content);
}
