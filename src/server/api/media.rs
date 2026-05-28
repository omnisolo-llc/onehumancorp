use axum::{
    extract::Multipart,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tokio::time::{sleep, Duration};

pub async fn handle_media_upload(mut multipart: Multipart) -> impl IntoResponse {
    let mut file_name = String::new();

    // Simulate reading the file
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        if let Some(name) = field.file_name() {
            file_name = name.to_string();
        }
        let _data = field.bytes().await.unwrap_or_default();
    }

    if file_name.is_empty() {
        file_name = "uploaded_file.jpg".to_string();
    }

    // Determine target URL for simulating processed file
    let optimized_url = format!("https://cdn.ohc.store/optimized/{}", file_name.replace(".jpg", ".webp").replace(".png", ".webp"));

    // Spawn background task simulating AI processing (cropping, WebP conversion, etc)
    tokio::spawn(async move {
        // Simulating the delay of processing a 10MB photo
        sleep(Duration::from_secs(5)).await;

        // Simulating writing to DB or event queue indicating processing is done
        // In a real application, this would trigger an event or update a row in Postgres
        tracing::info!("Background AI media processing completed for {}", file_name);
    });

    // Instantly return the expected URL so the UI doesn't block
    Json(json!({
        "status": "processing",
        "url": optimized_url,
        "message": "Media upload accepted. Processing in background."
    }))
}
