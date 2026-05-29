use axum::{
    body::Body,
    extract::Request,
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;
use http_body_util::BodyExt;

/// GzipCompress compresses data using gzip.
pub fn gzip_compress(data: Vec<u8>) -> std::io::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    encoder.finish()
}

/// should_compress_request checks request headers to decide if we should even consider compression.
pub fn should_compress_request(headers: &HeaderMap) -> bool {
    let accept_encoding = headers
        .get(header::ACCEPT_ENCODING)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !accept_encoding.contains("gzip") {
        return false;
    }

    // Skip compression for WebSocket upgrades
    if let Some(upgrade) = headers.get(header::UPGRADE) {
        if !upgrade.is_empty() {
            return false;
        }
    }

    true
}

/// should_compress_response checks response status and headers to decide if body should be compressed.
pub fn should_compress_response(res: &Response) -> bool {
    // Only compress successful or common error responses
    if !res.status().is_success() && res.status() != StatusCode::BAD_REQUEST && res.status() != StatusCode::NOT_FOUND {
        return false;
    }

    // Skip if already compressed
    if let Some(content_encoding) = res.headers().get(header::CONTENT_ENCODING) {
        if !content_encoding.is_empty() {
            return false;
        }
    }

    let content_type = res
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // Never compress SSE
    if content_type == "text/event-stream" {
        return false;
    }

    // Only compress text-based formats
    let is_compressible = content_type.starts_with("application/json")
        || content_type.starts_with("text/")
        || content_type.starts_with("application/javascript")
        || content_type.starts_with("application/xml");

    is_compressible
}

pub async fn gzip_middleware(req: Request, next: Next) -> Response {
    let can_compress = should_compress_request(req.headers());

    let res = next.run(req).await;

    if !can_compress || !should_compress_response(&res) {
        return res;
    }

    let (mut parts, body) = res.into_parts();

    // Collect the body into bytes. Note: this buffers the entire response in memory.
    // In a high-performance production system, streaming compression should be used.
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes().to_vec(),
        Err(e) => {
            tracing::error!("Failed to collect response body for gzip: {}", e);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap();
        }
    };

    // Skip small payloads as compression might actually increase size or provide negligible benefit
    if bytes.len() < 1024 {
        return Response::from_parts(parts, Body::from(bytes));
    }

    // Perform compression in a blocking task to avoid blocking the async executor
    let compressed_result = tokio::task::spawn_blocking(move || {
        gzip_compress(bytes)
    }).await;

    match compressed_result {
        Ok(Ok(compressed_bytes)) => {
            parts.headers.insert(header::CONTENT_ENCODING, header::HeaderValue::from_static("gzip"));
            parts.headers.insert(header::CONTENT_LENGTH, compressed_bytes.len().into());
            parts.headers.append(header::VARY, header::HeaderValue::from_static("Accept-Encoding"));
            Response::from_parts(parts, Body::from(compressed_bytes))
        }
        Ok(Err(e)) => {
            tracing::error!("Gzip compression error: {}", e);
            // Fallback to uncompressed - original bytes are lost due to move, so we'd need to clone if we wanted robust fallback
            // For now, if compression fails, we return error to be safe, but a better implementation would preserve original bytes.
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        }
        Err(e) => {
            tracing::error!("Tokio blocking task join error during gzip: {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::read::GzDecoder;
    use std::io::Read;

    #[test]
    fn test_gzip_compress() {
        let data = b"hello world".to_vec();
        let compressed = gzip_compress(data.clone()).unwrap();
        
        assert!(compressed.len() > 0);
        
        let mut decoder = GzDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).unwrap();
        
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_should_compress_request() {
        let mut headers = HeaderMap::new();
        headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static("gzip"));
        assert!(should_compress_request(&headers));

        headers.clear();
        headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static("deflate"));
        assert!(!should_compress_request(&headers));

        headers.clear();
        headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static("gzip"));
        headers.insert(header::UPGRADE, header::HeaderValue::from_static("websocket"));
        assert!(!should_compress_request(&headers));
    }
}
