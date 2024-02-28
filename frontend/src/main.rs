use eventsource::reqwest::Client;
use tokio_util::sync::CancellationToken;

async fn connect(channel_id: i32, token: CancellationToken) {
    let url = reqwest::Url::parse(format!("http://127.0.0.1:8000/subscribe/{}", channel_id).as_str()).unwrap();
    let client = Client::new(url);

    for event in client {
        if token.is_cancelled() {
            println!("I quit the channel!");
            break;
        }
        println!("{}", event.unwrap());
    }
}

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let token_clone = token.clone();

    let _task = tokio::spawn(async move {
        tokio::select! {
            _ = token_clone.cancelled() => {
                println!("cancelled");
            }
            _ = connect(1, token_clone.clone()) => {
            }
        }
    });

    while !token.is_cancelled() {
        println!("Do you want to cancel the subscription? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
            token.cancel();
        }
    }
}
