use tokio::sync::oneshot;

async fn some_computation() -> String {
    // sleep 1 second
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    // get now timestamp
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("now: {}", now)
}

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
            let res = some_computation().await;
            tx.send(res).unwrap();
    });

    let res = rx.await.unwrap();
    println!("Got {:?}", res);
}
