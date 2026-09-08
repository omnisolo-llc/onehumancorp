//! Opt-in gzip JSON framing. Legacy clients always receive text frames.
use axum::extract::ws::{Message, WebSocketUpgrade};
use flate2::{Compression, write::GzEncoder};
use std::io::Write;

pub const GZIP_PROTOCOL: &str = "omnisolo.gzip.v1";

pub fn negotiate(ws: WebSocketUpgrade) -> (WebSocketUpgrade, bool) {
    let ws = ws.protocols([GZIP_PROTOCOL]);
    let gzip = ws.selected_protocol().is_some();
    (ws, gzip)
}

pub fn encode_json(json: String, gzip: bool) -> Message {
    if gzip && json.len() > 1024 {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        if encoder.write_all(json.as_bytes()).is_ok()
            && let Ok(bytes) = encoder.finish()
        {
            return Message::Binary(bytes.into());
        }
    }
    Message::Text(json.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[tokio::test]
    async fn negotiated_and_legacy_connections_receive_compatible_frames() {
        use axum::{Router, response::IntoResponse, routing::get};
        use futures::StreamExt;
        use tokio_tungstenite::tungstenite::client::IntoClientRequest;
        async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
            let (ws, gzip) = negotiate(ws);
            ws.on_upgrade(move |mut socket| async move {
                socket
                    .send(encode_json("x".repeat(2048), gzip))
                    .await
                    .unwrap();
            })
        }
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, Router::new().route("/ws", get(handler)))
                .await
                .unwrap();
        });
        for protocol in [None, Some(GZIP_PROTOCOL)] {
            let mut request = format!("ws://{addr}/ws").into_client_request().unwrap();
            if let Some(protocol) = protocol {
                request
                    .headers_mut()
                    .insert("sec-websocket-protocol", protocol.parse().unwrap());
            }
            let (mut socket, response) = tokio_tungstenite::connect_async(request).await.unwrap();
            let compressed = protocol == Some(GZIP_PROTOCOL);
            assert_eq!(
                response.headers().get("sec-websocket-protocol").is_some(),
                compressed
            );
            let frame = socket.next().await.unwrap().unwrap();
            if compressed {
                assert!(frame.is_binary());
                let bytes = frame.into_data();
                let mut text = String::new();
                flate2::read::GzDecoder::new(bytes.as_ref())
                    .read_to_string(&mut text)
                    .unwrap();
                assert_eq!(text, "x".repeat(2048));
            } else {
                assert_eq!(frame.to_text().unwrap(), "x".repeat(2048));
            }
        }
        server.abort();
    }

    #[test]
    fn compresses_large_negotiated_messages_and_round_trips_unicode() {
        let json = format!("{{\"text\":\"{}\"}}", "hello 世界".repeat(200));
        let Message::Binary(bytes) = encode_json(json.clone(), true) else {
            panic!("negotiated messages over 1KiB must use gzip binary frames");
        };
        assert!(bytes.len() < 1024);
        let mut decoded = String::new();
        flate2::read::GzDecoder::new(bytes.as_ref())
            .read_to_string(&mut decoded)
            .unwrap();
        assert_eq!(decoded, json);
    }

    #[test]
    fn legacy_clients_and_small_messages_keep_text_frames() {
        for (json, gzip) in [
            ("x".repeat(2048), false),
            ("x".repeat(1024), true),
            (String::new(), true),
        ] {
            assert!(
                matches!(encode_json(json.clone(), gzip), Message::Text(text) if text.as_str() == json)
            );
        }
    }
}
