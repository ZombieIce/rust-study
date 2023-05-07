use tokio::sync::{mpsc, oneshot};

enum Command {
    Increment,
}

#[tokio::main]
async fn main() {
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Command, oneshot::Sender<u64>)>(100);
    tokio::spawn(async move {
        let mut counter = 0u64;
        while let Some((cmd, tx)) = cmd_rx.recv().await {
            match cmd {
                Command::Increment => {
                    counter += 1;
                    tx.send(counter).unwrap();
                }
            }
        }
    });


    let mut join_handles = Vec::new();

    for _ in 0..10 {
        let cmd_tx = cmd_tx.clone();
        join_handles.push(tokio::spawn(async move {
            let (resp_tx, resp_rx) = oneshot::channel();
            cmd_tx.send((Command::Increment, resp_tx)).await.ok().unwrap();
            let res = resp_rx.await.unwrap();
            println!("previous counter: {}", res);
        }));
    }
    
    for join_handle in join_handles {
        join_handle.await.unwrap();
    }

}
