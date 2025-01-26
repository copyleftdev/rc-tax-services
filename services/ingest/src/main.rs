use anyhow::Result;
use common::PropertyRecord;
use futures::StreamExt;
use reqwest::Client;
use serde_json::from_str;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() -> Result<()> {
    let compute_url = std::env::var("COMPUTE_URL")
        .unwrap_or("http://compute:8080/api/compute".to_string());

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await?;
    println!("Ingest WS listening on {addr}");

    let http_client = Client::new();

    loop {
        let (stream, _sock_addr) = listener.accept().await?;
        println!("New WS client connected...");

        let compute_clone = compute_url.clone();
        let client_clone = http_client.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_ws(stream, compute_clone, client_clone).await {
                eprintln!("WS connection error: {e}");
            }
        });
    }
}

async fn handle_ws<S>(
    raw_stream: S,
    compute_url: String,
    http_client: Client,
) -> Result<()>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
{
    let ws_stream = accept_async(raw_stream).await?;
    let (mut _ws_tx, mut ws_rx) = ws_stream.split();

    println!("WebSocket handshake complete.");

    while let Some(msg_res) = ws_rx.next().await {
        let msg = match msg_res {
            Ok(m) => m,
            Err(e) => {
                eprintln!("WebSocket receive error: {e}");
                break;
            }
        };

        match msg {
            Message::Text(text) => {
                // Attempt to parse a PropertyRecord
                match from_str::<PropertyRecord>(&text) {
                    Ok(record) => {
                        // Forward to compute
                        let _ = post_to_compute(&http_client, &compute_url, &record).await;
                    }
                    Err(_) => {
                        eprintln!("Invalid PropertyRecord JSON: {text}");
                    }
                }
            }
            Message::Binary(bin) => {
                if let Ok(text) = std::str::from_utf8(&bin) {
                    match from_str::<PropertyRecord>(text) {
                        Ok(record) => {
                            let _ = post_to_compute(&http_client, &compute_url, &record).await;
                        }
                        Err(_) => {
                            eprintln!("Invalid binary JSON data");
                        }
                    }
                }
            }
            Message::Ping(_p) => {
                // Renamed 'p' to '_p' to avoid unused variable warning
                // let _ = _ws_tx.send(Message::Pong(_p)).await;
            }
            Message::Close(_c) => {
                println!("Client closed WebSocket");
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

async fn post_to_compute(
    client: &Client,
    compute_url: &str,
    record: &PropertyRecord,
) -> Result<()> {
    // .json(...) requires the "json" feature in reqwest
    let resp = client
        .post(compute_url)
        .json(record)
        .send()
        .await?;

    if !resp.status().is_success() {
        eprintln!(
            "Compute returned error: status={}, body={:?}",
            resp.status(),
            resp.text().await.ok()
        );
    }
    Ok(())
}
