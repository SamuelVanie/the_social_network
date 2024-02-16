use std::collections::HashMap;

use rocket::{serde::{Deserialize, Serialize}, tokio::sync::broadcast::Sender};

// the different types of messages that a client could
// send down the channel
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
enum MessageType {
    CONNECT,
    SEND,
    QUIT
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub room_id: u32,
    pub message_type: MessageType,
    pub message_content: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    // will contain the room ids with the Sender end of the
    // broadcast sockets
    // each connections between a client and the server is in here
    pub clients: HashMap<u32, Sender<Message>>,
}
