use ws::WebSocket;
    
pub struct ChatServer {
    clients: Vec<WebSocket>,
}
