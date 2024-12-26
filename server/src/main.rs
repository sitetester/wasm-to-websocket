use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

/// Sets up a WebSocket server that:
/// - Binds to a local address (127.0.0.1:8081)
/// - Accepts incoming connections & handle each in a separate `tokio::spawn`
#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8081";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("WebSocket server listening on ws://{}", addr);

    // accept().await yields (TcpStream, SocketAddr) for each new connection
    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection from {}", addr);
        // Handle each connection concurrently
        tokio::spawn(handle_connection(stream, addr));
    }
}

/// Handles an individual WebSocket connection from a client. It's responsible for:
/// - Upgrading the TCP connection to a WebSocket connection
/// - Processing incoming messages
/// - Sending responses
///
/// # Arguments
/// * `stream` - The TCP stream representing the client connection
/// * `addr` - The socket address of the connected client
async fn handle_connection(stream: TcpStream, addr: std::net::SocketAddr) {
    // Perform WebSocket handshake by upgrading the TCP connection
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(err) => {
            println!("Error accepting connection from {}: {}", addr, err);
            return;
        }
    };

    // Split WebSocket stream into writer and reader parts
    // This allows us to send and receive messages independently
    let (mut write, mut read) = ws_stream.split();

    // `next().await` yields Option<Result<Message, Error>> for each message
    while let Some(msg) = read.next().await {
        match msg {
            // Handle text messages
            Ok(Message::Text(client_text)) => {
                println!("Received `{client_text}` from {addr}");

                let current_time = chrono::Local::now().format("%H:%M:%S").to_string();
                let ws_msg = Message::Text(format!("Hello from Rust at {current_time}").into());
                if let Err(err) = write.send(ws_msg).await {
                    println!("Error sending message to {addr}: {err}");
                    break; // Break loop on send error
                }
            }
            // When the client's browser window/tab is closed or web page is refreshed
            // Or when the client explicitly calls websocket.close() in their code
            Ok(Message::Close(_)) => {
                println!("Client {} initiated close", addr);
                break;
            }
            Err(e) => {
                println!("Error receiving message from {}: {}", addr, e);
                break;
            }
            // Ignore other cases for now (not relevant to task)
            _ => (),
        }
    }

    println!("Connection closed: {}", addr);
}
