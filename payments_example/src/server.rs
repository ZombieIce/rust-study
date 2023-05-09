use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentRequest, BtcPaymentResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Default)]
pub struct BitCoinService;

#[tonic::async_trait]
impl Bitcoin for BitCoinService {
    async fn send_payment(
        &self,
        req: Request<BtcPaymentRequest>,
    ) -> Result<Response<BtcPaymentResponse>, Status> {
        println!("Got a request: {:?}", req);
        let req = req.into_inner();
        let reply = BtcPaymentResponse {
            success: true,
            message: format!(
                "Payment of {} to {} was successful",
                req.amount, req.to_addr
            )
            .into(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();
    let btc_service = BitCoinService::default();
    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .serve(addr)
        .await
        .unwrap();
}
