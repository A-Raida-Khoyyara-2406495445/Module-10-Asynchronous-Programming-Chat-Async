use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri("ws://127.0.0.1:8080".parse().unwrap())
            .connect()
            .await?;

    let stdin = io::stdin();
    let mut stdin_lines = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("From server: {}", text);
                        }
                    }
                    _ => break,
                }
            }
            line = stdin_lines.next_line() => {
                match line {
                    Ok(Some(text)) => {
                        ws_stream.send(Message::text(text)).await?;
                    }
                    _ => break,
                }
            }
        }
    }
    Ok(())
}