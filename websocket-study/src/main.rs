use futures_util::StreamExt;
// use tungstenite::connect;
use url::Url;
use tokio_tungstenite::connect_async;
use tokio;
static BINANCE_WS_API: &str = "wss://fstream.binance.com/ws/btcusdt@depth";

#[tokio::main]
async fn main() {
    let url = Url::parse(BINANCE_WS_API).unwrap();
    let (mut socket, response) = connect_async(url).await.expect("Can't connect");
    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, value) in response.headers() {
        println!("* {}: {:?}", header, value);
    }
    
    let handler = tokio::spawn(async move {
        while let Some(msg) = socket.next().await {
            let msg = msg.unwrap();
            println!("Received a message from the server: {:?}", msg);
        }
    });
    handler.await.unwrap();
}