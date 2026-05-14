
use std::sync::Arc;
use ohc_builtin_agent::mesh::transport::MeshTransport;
use axum::{routing::get, Router, Json};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct VideoMetadata {
    pub id: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub walkthrough_id: Option<String>,
}

pub async fn get_videos() -> Json<Vec<VideoMetadata>> {
    let videos = vec![
        VideoMetadata {
            id: "1".to_string(),
            category: "Getting Started".to_string(),
            title: "Set up your store".to_string(),
            description: "How to add your first product and set your prices.".to_string(),
            url: "https://onehumancorp.com/videos/setup.mp4".to_string(),
            walkthrough_id: Some("store-setup".to_string()),
        },
        VideoMetadata {
            id: "2".to_string(),
            category: "Payments".to_string(),
            title: "Accept your first payment".to_string(),
            description: "Connect your bank account to start receiving money.".to_string(),
            url: "https://onehumancorp.com/videos/payment.mp4".to_string(),
            walkthrough_id: Some("accept-payment".to_string()),
        },
        VideoMetadata {
            id: "3".to_string(),
            category: "AI".to_string(),
            title: "Activate your AI Support Agent".to_string(),
            description: "Turn on your 24/7 AI helper.".to_string(),
            url: "https://onehumancorp.com/videos/agent.mp4".to_string(),
            walkthrough_id: Some("activate-agent".to_string()),
        }
    ];
    Json(videos)
}

pub async fn get_swagger_ui() -> axum::response::Html<&'static str> {
    axum::response::Html(r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <title>Swagger UI</title>
    <link rel="stylesheet" type="text/css" href="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/5.0.0/swagger-ui.css" />
    <style>
      html { box-sizing: border-box; overflow: -moz-scrollbars-vertical; overflow-y: scroll; }
      *, *:before, *:after { box-sizing: inherit; }
      body { margin:0; background: #fafafa; }
    </style>
  </head>
  <body>
    <div id="swagger-ui"></div>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/5.0.0/swagger-ui-bundle.js"> </script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/swagger-ui/5.0.0/swagger-ui-standalone-preset.js"> </script>
    <script>
      window.onload = function() {
        const spec = {
          "openapi": "3.0.0",
          "info": {
            "title": "OneHuman Corp API",
            "version": "1.0.0"
          },
          "paths": {
            "/api/v1/health": {
              "get": {
                "summary": "Check server status",
                "responses": {
                  "200": { "description": "OK" }
                }
              }
            }
          }
        };
        const ui = SwaggerUIBundle({
          spec: spec,
          dom_id: '#swagger-ui',
          deepLinking: true,
          presets: [
            SwaggerUIBundle.presets.apis,
            SwaggerUIStandalonePreset
          ],
          plugins: [
            SwaggerUIBundle.plugins.DownloadUrl
          ],
          layout: "StandaloneLayout"
        });
      };
    </script>
  </body>
</html>
"#)
}

pub fn router() -> Router<Arc<dyn MeshTransport>> {
    Router::new().route("/videos", get(get_videos)).route("/swagger", get(get_swagger_ui))
}
