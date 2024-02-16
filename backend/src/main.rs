#[macro_use] extern crate rocket;

mod server;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::chat_server])
        .attach(AdHoc::on_ignite("Chat Server", |rocket| async {
            let chat_server = server::ChatServer { clients: Vec::new() };
            rocket.manage(chat_server)
        }))

}
