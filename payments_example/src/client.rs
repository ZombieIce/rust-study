use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() {
    let mut client = BitcoinClient::connect("http://[::1]:50051")
        .await
        .unwrap();
    let request = tonic::Request::new(BtcPaymentRequest {
        amount: 100,
        to_addr: "1NcXPMRaanz43b1kokpPuYDdk6GGDvxT2T".into(),
        from_addr: "1F1tAaz5x1HUXrCNLbtMDqcw6o5GNn4xqX".into(),
    });

    let response = client.send_payment(request).await.unwrap();
    println!("RESPONSE={:?}", response);
}