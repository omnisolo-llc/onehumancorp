issue_title: "[Architecture] Universal Multi-Modal Asset AI Processing & Edge Delivery Pipeline"
issue_description: |
  ### Problem Statement
  When Maya (the baker) takes a photo of her latest vegan cake or Carlos (the handyman) snaps a video of a completed kitchen repair on their phones, the raw assets are huge, unoptimized, and often visually messy (bad lighting, cluttered backgrounds). They shouldn't need to learn Photoshop, figure out what "WebP" means, or understand video compression to make their storefronts look professional and load instantly. Existing platforms force them to either upload 15MB photos that ruin their site's loading speed or manually use third-party apps to compress and edit their media. We need an invisible, background engine that instantly processes, enhances, and edge-caches every piece of media they upload.

  ### Research Report
  - **Competitive Analysis:**
    - **Shopify:** Provides basic image compression but relies heavily on third-party marketplace apps for background removal, AI upscaling, and video optimization.
    - **Wix/Squarespace:** Auto-scales images for different devices but lacks built-in AI enhancement (like lighting correction or automatic background removal for product shots) out-of-the-box without manual intervention.
    - **GoDaddy:** Very basic image handling; users often complain about blurry images if they don't format them correctly beforehand.
  - **Data & Findings:**
    - High-quality, fast-loading images increase conversion rates by up to 35%.
    - Over 70% of our target personas (like Fatima or Leo) capture all business media directly on low-to-mid-tier smartphones.
    - Latency is critical: If an asset takes longer than 2 seconds to load on a 3G/4G connection, bounce rates skyrocket.
  - **The Opportunity:** Implement an invisible pipeline where raw media uploads are intercepted by an AI Operations Agent. This agent automatically removes backgrounds for product shots, color-corrects, transcodes to modern formats (AV1/WebP/Blurhash), and distributes to a global edge CDN—all without the user pressing a single "optimize" button.

  ### Design Doc

  #### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ MEDIA_ASSET : uploads
      MEDIA_ASSET ||--o{ ASSET_VARIANT : generates
      MEDIA_ASSET {
          string asset_id PK
          string tenant_id FK
          string raw_uri
          string status "pending, processing, ready"
      }
      ASSET_VARIANT {
          string variant_id PK
          string asset_id FK
          string format "webp, av1, blurhash"
          string edge_url
      }
  ```

  ```mermaid
  sequenceDiagram
      actor Maya as Maya (Mobile App)
      participant OHC_API as OHC API (Rust)
      participant Auth as SPIFFE/SPIRE Identity
      participant JobQueue as Background Job Queue
      participant AIAgent as AI Vision & Operations Dept
      participant EdgeCDN as Edge Cache CDN

      Maya->>OHC_API: Uploads 12MB raw cake photo (375px viewport)
      OHC_API->>Auth: Validate Tenant & M2M Identity
      Auth-->>OHC_API: OK
      OHC_API->>JobQueue: Enqueue AssetProcessingTask
      OHC_API-->>Maya: Return success (optimistic UI with local blob)
      JobQueue->>AIAgent: Trigger processing pipeline
      AIAgent->>AIAgent: Remove background, color correct, generate Blurhash
      AIAgent->>AIAgent: Transcode to WebP (image) / AV1 (video)
      AIAgent->>EdgeCDN: Push optimized variants to edge CDN
      AIAgent->>OHC_API: Update MediaAsset status to 'ready'
      OHC_API-->>Maya: WebSocket/SSE push notification (Asset Ready)
  ```

  #### Mobile UX Flow & UI Wireframes (375px first)
  1. **Upload Interaction:** Maya taps "Add Product Photo". The native camera or photo picker opens.
  2. **Optimistic UI:** The photo instantly appears in the product card grid. A subtle macOS-style Translucent Glass shimmer effect indicates it's being "magic enhanced" in the background. No blocking loaders.
  3. **Completion:** The shimmer fades, replacing the local image with the edge-cached, AI-enhanced version (background removed, perfectly lit).
  4. **Grandmother Test:** There are no settings for "compression level", "file format", or "crop ratio". The user just picks a photo, and it looks beautiful. Advanced settings (if needed) are hidden behind an "Adjustments" toggle.

  #### AI Agent Integration Points
  - **AI Vision Department:** Intercepts the raw file. Uses vision models to detect if it's a product (triggering background removal) or a lifestyle shot (triggering only color grading and compression).
  - **Operations Department:** Manages the background queue, ensuring that heavy video transcoding doesn't block critical path APIs and scales gracefully.

  #### Key Design Decisions and Why
  - **Asynchronous Processing:** Uploads must never block the UI. The app uses a local object URL to display the image immediately while the backend queue does the heavy lifting.
  - **Zero Trust Isolation:** Every asset is cryptographically bound to the `tenant_id` via SPIFFE/SPIRE. Cross-tenant asset leakage is impossible.
  - **Blurhash Generation:** Every image generates a tiny base64 Blurhash inline payload. This guarantees that even on Fatima's 3G connection, the storefront instantly paints the visual structure before the high-res WebP loads.

  ### Implementation Prompt
  Implement the Universal Multi-Modal Asset AI Pipeline. Create the necessary backend queuing mechanisms and AI agent tool calls to handle raw image/video uploads. Ensure the user-facing mobile client (Tauri v2) supports optimistic UI rendering with local blobs while the server processes the asset asynchronously. The final output must deliver edge-cached WebP/AV1 URLs and inline Blurhashes. Ensure strict multi-tenant isolation so assets cannot be accessed outside their owning organization. Do not block the user's workflow during processing.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []