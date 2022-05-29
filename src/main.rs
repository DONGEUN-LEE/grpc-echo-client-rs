use echo::{echo_client::EchoClient, EchoRequest};
use tonic::transport::Endpoint;
use tonic::Request;

pub mod echo {
    tonic::include_proto!("echo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::from_static("http://localhost:5000")
        .connect()
        .await?;

    let mut client = EchoClient::new(channel);

    let request = tonic::Request::new(EchoRequest {
        message: "test".into(),
    });

    let response = client.unary_echo(request).await?;

    println!("UnaryEcho: {:?}", response.into_inner().message);

    let outbound = async_stream::stream! {
        let mut vec = Vec::new();
        vec.push("hello");
        vec.push("world");
        for x in &vec {
            let request = EchoRequest {
                message: String::from(*x),
            };

            yield request;
        }
    };

    let response = client
        .bidirectional_streaming_echo(Request::new(outbound))
        .await?;
    let mut inbound = response.into_inner();

    while let Some(echo) = inbound.message().await? {
        println!("received message: {:?}", echo.message);
    }

    Ok(())
}
