use tonic::{transport::Server, Request, Response, Status};

use payment::payments_server::{Payments, PaymentsServer};
use payment::{PaymentRequest, PaymentResponse};

pub mod payment {
    tonic::include_proto!("payment");
}

#[derive(Debug, Default)]
pub struct PaymentsService {}

#[tonic::async_trait]
impl Payments for PaymentsService {
    async fn initiate_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        // let req = request.into_inner();

        let reply = PaymentResponse {
            amount: String::from("2000"),
            from: String::from("Parth"),
            to: String::from("Milan"),
            reference: String::from("2332u3enjedn3e3e2"),
            id: 223211,
            status: 1,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse()?;
    let btc_service = PaymentsService::default();

    Server::builder()
        .add_service(PaymentsServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}
