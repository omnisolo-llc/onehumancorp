<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.08); padding: 24px; border-radius: 12px; font-family: 'Outfit', sans-serif; color: #E0E0E0;">

# Frontend Proxy Server

The **Frontend Proxy Server** is responsible for serving the Flutter web static assets and reverse-proxying API traffic to the backend dashboard server. This eliminates CORS issues and simplifies deployment.

## Request Flow

```mermaid
sequenceDiagram
    participant User as Web Client
    participant Proxy as Frontend Proxy Server
    participant Backend as OHC Dashboard Core
    User->>Proxy: Request Static File (index.html)
    Proxy-->>User: Serve Asset (FRONTEND_STATIC_DIR)
    User->>Proxy: /api/v1/resource
    Proxy->>Backend: Reverse Proxy Request
    Backend-->>Proxy: JSON Response
    Proxy-->>User: JSON Response
```

## Developer Usage

*Requires `FRONTEND_STATIC_DIR` and `BACKEND_URL` environment variables.*

</div>
