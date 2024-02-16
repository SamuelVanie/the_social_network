use rocket::{response::stream::{Event, EventStream}, tokio::{select, sync::broadcast::{error::RecvError, Sender}}, Shutdown, State};
use crate::server;

#[get("/subscribe/<channel_id>")]
pub async fn subscribe(channel_id: u32, sessions: &State<server::AppState>, mut end: Shutdown) -> EventStream![] {
    let channel: &Sender<server::Message> = match sessions.clients.get(&channel_id) {
        None => return EventStream!{ yield Event::empty(); },
        Some(c) => c,
    };

    let mut client = channel.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg  = client.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}
