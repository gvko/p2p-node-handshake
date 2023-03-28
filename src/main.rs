use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use futures_util::{SinkExt, stream::{SplitSink, SplitStream}, StreamExt};

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the Goerli node over Alchemy WebSocket API
    let (mut ws_stream, _)
        = connect_async("wss://eth-goerli.g.alchemy.com/v2/<KEY>")
        .await
        .expect("Failed to connect to websocket stream");

    let (write, read) = ws_stream.split();
    let mut write_stream = write;

    // Step 1: Send a version message to initiate the handshake
    let res = write_stream.send(Message::text("VERSION 1.0")).await;
    if res.is_err() {
        eprintln!("Failed to send version message");
    }
    println!("{:#?}", res);

    // Receive a response from the server with server version, protocol version, and public keys

    // Verify the server's public key and send a message to agree on the protocol version

    // Receive a message from the server containing its public key for encryption

    // Send a message containing the client's public key

    // Send a message containing the user agent and authentication credentials

    // Receive a message indicating that the handshake is complete

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
    }
}
