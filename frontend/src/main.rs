use eventsource::reqwest::Client;

fn main() {
    let url = reqwest::Url::parse("http://127.0.0.1:8000/subscribe/1").unwrap();
    let client = Client::new(url);

    for event in client {
        println!("{}", event.unwrap());
    }
}
