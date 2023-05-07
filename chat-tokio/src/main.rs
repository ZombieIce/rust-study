use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

fn give_me_default<T>() -> T
where
    T: Default,
{
    Default::default()
}

#[tokio::main]
async fn main() {
    let value = give_me_default::<i32>();
    println!("{}", value);
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _) = broadcast::channel(10);
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx1 = tx.clone();
        let mut rx = tx1.subscribe();

        tokio::spawn(async move {
            let (read, mut writer) = socket.split();
            let mut reader = BufReader::new(read);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx1.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                        writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
