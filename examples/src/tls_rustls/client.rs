//! This examples shows how you can combine `hyper-rustls` and `tonic` to
//! provide a custom `ClientConfig` for the tls configuration.

pub mod pb {
    tonic::include_proto!("/grpc.examples.unaryecho");
}

use tower::ServiceBuilder;
use pb::{echo_client::EchoClient, EchoRequest};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let stream = tokio::net::TcpStream::connect("[::1]:50051").await?;

    let (svc, conn) = hyper::client::conn::Builder::new()
        .http2_only(true)
        .handshake(stream)
        .await?;

    tokio::spawn(async {
        if let Err(err) = conn.await {
            eprintln!("error servicing connection: {err}");
        }
    });


    // If you need clone, you can buffer the service behind an mpsc
    // let svc = ServiceBuilder::new().buffer(256).service(svc);


    let mut client = EchoClient::new(svc);

    let request = tonic::Request::new(EchoRequest {
        message: "hello".into(),
    });

    let response = client.unary_echo(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
