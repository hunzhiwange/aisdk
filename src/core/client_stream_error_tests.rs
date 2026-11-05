//! Preserve the actual cause of interrupted, uncompressed SSE responses.

use super::map_sse_event_result;
use crate::core::DynamicModel;
use crate::error::Error;
use crate::providers::openai_chat_completions::OpenAIChatCompletions;
use futures::StreamExt;
use std::io::{Read, Write};
use std::net::TcpListener;

async fn interrupted_response_error(chunked: bool) -> Error {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        let mut request = Vec::new();
        while !request.windows(4).any(|bytes| bytes == b"\r\n\r\n") {
            let mut buffer = [0; 4096];
            let size = socket.read(&mut buffer).unwrap();
            assert!(size > 0);
            request.extend_from_slice(&buffer[..size]);
        }
        let request = String::from_utf8(request).unwrap();
        assert!(!request.to_ascii_lowercase().contains("accept-encoding:"));
        let body = "data: partial\n\n";
        if chunked {
            write!(socket, "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n{:x}\r\n{body}\r\n", body.len()).unwrap();
        } else {
            write!(socket, "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: 9999\r\nConnection: close\r\n\r\n{body}").unwrap();
        }
    });
    let response = reqwest::Client::builder()
        .no_proxy()
        .no_gzip()
        .no_brotli()
        .no_zstd()
        .no_deflate()
        .build()
        .unwrap()
        .get(format!("http://{address}/?api_key=fixture-secret"))
        .send()
        .await
        .unwrap();
    assert!(response.headers().get("content-encoding").is_none());
    let mut stream = response.bytes_stream();
    let mut received = 0;
    let transport = loop {
        match stream.next().await.expect("truncated body must fail") {
            Ok(bytes) => received += bytes.len(),
            Err(error) => break error,
        }
    };
    assert!(received > 0);
    assert_eq!(transport.to_string(), "error decoding response body");
    server.join().unwrap();
    // Also verify URL removal when a transport failure carries a request URL.
    let transport = transport.with_url(
        format!("http://{address}/?api_key=fixture-secret")
            .parse()
            .unwrap(),
    );
    map_sse_event_result::<OpenAIChatCompletions<DynamicModel>>(Err(
        reqwest_eventsource::Error::Transport(transport),
    ))
    .await
    .unwrap_err()
}

#[tokio::test]
async fn sse_content_length_interruption_preserves_transport_cause_without_url() {
    let error = interrupted_response_error(false).await;
    let Error::ApiError {
        details,
        status_code,
    } = error
    else {
        panic!("expected API error");
    };
    assert!(details.starts_with("error decoding response body | caused_by:"));
    assert!(
        details.contains("end of file before message length reached"),
        "{details}"
    );
    assert!(!details.contains("fixture-secret"));
    assert!(!details.contains("api_key"));
    assert_eq!(status_code, None);
}

#[tokio::test]
async fn sse_chunked_interruption_preserves_transport_cause_without_url() {
    let error = interrupted_response_error(true).await;
    let Error::ApiError {
        details,
        status_code,
    } = error
    else {
        panic!("expected API error");
    };
    assert!(details.starts_with("error decoding response body | caused_by:"));
    assert!(
        details.contains("unexpected EOF during chunk size line"),
        "{details}"
    );
    assert!(!details.contains("fixture-secret"));
    assert!(!details.contains("api_key"));
    assert_eq!(status_code, None);
}
