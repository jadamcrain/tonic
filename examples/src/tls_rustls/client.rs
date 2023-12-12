//! This examples shows how you can combine `hyper-rustls` and `tonic` to
//! provide a custom `ClientConfig` for the tls configuration.

pub mod pb {
    tonic::include_proto!("/grpc.examples.unaryecho");
}

use pb::{echo_client::EchoClient, EchoRequest};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let stream = tokio::net::TcpStream::connect("[::1]:50051").await?;

    let (sender, conn) = hyper::client::conn::handshake(stream).await?;

    tokio::spawn(async {
        if let Err(err) = conn.await {
            eprintln!("error servicing connection: {err}");
        }
    });


    // I've tried instantiating with both the raw sender and the buffered service below
    // let sender = tower::buffer::Buffer::new(sender, 64);


    let mut client = EchoClient::new(sender);

    let request = tonic::Request::new(EchoRequest {
        message: "hello".into(),
    });

    let response = client.unary_echo(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
