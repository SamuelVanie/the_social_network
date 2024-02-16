use ws::WebSocket;

#[get("/ws")]
fn chat_server(state: State<server::ChatServer>, websocket: WebSocket) {
    state.clients.push(websocket);
}
