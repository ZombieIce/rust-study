use tokio::sync::mpsc;

async fn some_computation(interval: u64, task_name: String) -> String {
    // sleep input second
    tokio::time::sleep(std::time::Duration::from_millis(interval)).await;
    // get now timestamp
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("{}: {}", task_name, now)
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    for i in vec![500u64, 1000u64] {
        let now_tx = tx.clone();
        tokio::spawn(async move {
            loop {
                let res = some_computation(i, format!("task{}", i/500)).await;
                now_tx.send(res).await.unwrap();
            }
        });
    }

    loop {
        let res = rx.recv().await.unwrap();
        println!("Got {:?}", res);
    }
}
