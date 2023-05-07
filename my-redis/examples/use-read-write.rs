use tokio::fs::File;
use tokio::io::{copy, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let mut file = File::create("foo.txt").await.unwrap();
    let n = file.write(b"some bytes").await.unwrap();
    println!("Wrote the first {} bytes of 'some bytes'.", n);

    let mut file = File::create("foo.txt").await.unwrap();
    file.write_all(b"some all bytes").await.unwrap();

    let mut f = File::open("foo.txt").await.unwrap();
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await.unwrap();

    println!("The bytes: {:?}", &buffer[..n]);

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await.unwrap();
    println!("The bytes: {:?}", buffer);

    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await.unwrap();
    copy(&mut reader, &mut file).await.unwrap();
}
