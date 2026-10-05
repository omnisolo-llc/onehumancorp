//! Transport policy shared by the local HTTP harness adapters.
//!
//! Callers retain their numeric loopback address checks and operation deadlines.
//! JSON limits apply to decoded HTTP bodies, separately from response headers.

use reqwest::header::{CONTENT_LENGTH, TRANSFER_ENCODING};

const MAX_HEADER_BYTES: usize = 64 * 1024;

pub(super) fn local_client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .http1_only()
        // Also prevent Hyper's retry of an unstarted request on a reused socket.
        .pool_max_idle_per_host(0)
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .no_zstd()
        .build()
}

pub(super) enum ResponseError {
    Transport(reqwest::Error),
    Invalid(&'static str),
}

pub(super) fn validate_headers(response: &reqwest::Response) -> Result<(), ResponseError> {
    let headers = response.headers();
    // Reqwest 0.12 does not expose Hyper's HTTP/1 header-buffer setting. Hyper
    // bounds incomplete headers itself (417,792 bytes in the locked version);
    // this smaller limit applies to the parsed fields before consuming a body.
    let bytes: usize = headers
        .iter()
        .map(|(name, value)| name.as_str().len() + 2 + value.as_bytes().len() + 2)
        .sum();
    if bytes > MAX_HEADER_BYTES {
        return Err(ResponseError::Invalid("HTTP headers exceeded the limit"));
    }
    if headers.contains_key(TRANSFER_ENCODING) {
        if headers.contains_key(CONTENT_LENGTH) {
            return Err(ResponseError::Invalid(
                "HTTP response contained both Transfer-Encoding and Content-Length",
            ));
        }
        // Hyper removes chunk framing. Other transfer codings are not part of
        // either adapter's JSON/SSE protocol and must not pass through as data.
        let mut values = headers.get_all(TRANSFER_ENCODING).iter();
        let chunked = values
            .next()
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.trim().eq_ignore_ascii_case("chunked"));
        if !chunked || values.next().is_some() {
            return Err(ResponseError::Invalid(
                "HTTP response used an unsupported transfer coding",
            ));
        }
    }
    Ok(())
}

pub(super) async fn bounded_body(
    mut response: reqwest::Response,
    limit: usize,
) -> Result<Vec<u8>, ResponseError> {
    validate_headers(&response)?;
    if response
        .content_length()
        .is_some_and(|length| length > limit as u64)
    {
        return Err(ResponseError::Invalid(
            "HTTP response exceeded the body limit",
        ));
    }
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(ResponseError::Transport)? {
        if chunk.len() > limit - body.len() {
            return Err(ResponseError::Invalid(
                "HTTP response exceeded the body limit",
            ));
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}
