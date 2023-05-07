#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        "return value"
    });


    let opt = handle.await.unwrap();
    println!("GOT: {}", opt);
}
