#[macro_use] extern crate rocket;

use rocket::tokio::sync::broadcast::channel;

pub mod routes;
pub mod server;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<server::AppState>(1024).0)
        .mount("/", routes![routes::subscribe])

}
