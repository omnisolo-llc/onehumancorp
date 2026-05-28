use axum::{
    body::Bytes,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tokio::time::{sleep, Duration};

// A naive handler taking raw body since multipart is disabled
pub async fn handle_media_upload(body: Bytes) -> impl IntoResponse {
    let file_name = "uploaded_file.webp".to_string();

    // Determine target URL for simulating processed file
    let optimized_url = format!("https://cdn.ohc.store/optimized/{}", file_name);

    // Spawn background task simulating AI processing (cropping, WebP conversion, etc)
    let body_len = body.len();
    tokio::spawn(async move {
        // Simulating the delay of processing a 10MB photo
        sleep(Duration::from_secs(5)).await;

        // Simulating writing to DB or event queue indicating processing is done
        // In a real application, this would trigger an event or update a row in Postgres
        tracing::info!("Background AI media processing completed for {} of size {}", file_name, body_len);
    });

    // Instantly return the expected URL so the UI doesn't block
    Json(json!({
        "status": "processing",
        "url": optimized_url,
        "message": "Media upload accepted. Processing in background."
    }))
}
