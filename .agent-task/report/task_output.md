issue_title: "Research: Mobile Payload Optimization & Edge Caching"
issue_description: |
  # Research Report: OHC Mobile-First Network Payload Optimization & Edge Caching

  ## Problem Statement
  For our key personas like Fatima (food cart, low-end Android, slow network) and Carlos (field service, weak 4G), the initial load time and network footprint of the OHC assistant are critical. Currently, the unified application (PWA/Flutter) may deliver large payloads or unoptimized assets over slow connections, causing latency and degrading the "instant access" promise. We need to implement a mobile-first, edge-cached, dynamic optimization strategy.

  ## Research Findings
  - **Competitor Insights**: Shopify (Hydrogen/Oxygen) and Wix optimize heavily by serving static assets from edge CDNs and chunking API payloads.
  - **Codebase Gaps**: Currently, there is an `edge-cache` (OpenResty) service in `docker-compose.yml`, but there is no specific design for compressing dynamic responses (e.g., WebP image transcoding) or splitting large multi-tenant JSON payloads for the mobile PWA shell.
  - **Mobile Constraints**: 375px screens need smaller images and paginated or lazy-loaded data. A slow connection means `initial-load` must be < 2 seconds.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  flowchart TD
      A[Mobile Client PWA/Flutter] --> B[Edge Cache OpenResty]
      B -- Cached Assets --> A
      B -- API Requests --> C[Rust Server API]
      C --> D[Image Transcoder MinIO/GCS]
      D -- WebP Assets --> B
      C -- Paginated Data --> B
  ```

  ### UI Wireframes / Screen Flow
  - **Initial Load**: The user opens the app and immediately sees a lightweight shell (navigation + skeleton loading) cached at the edge.
  - **Work Triage Feed**: Displays the first 10 items. As the user scrolls, lazy loads the next chunk.
  - **Image Loading**: Images use progressive loading or blurred placeholders before the full WebP image loads.

  ### Mobile UX Flow
  1. **Launch**: Instant display of cached shell (PWA).
  2. **Data Fetch**: API returns chunked, compressed JSON payloads.
  3. **Asset Rendering**: Images are served in WebP format, appropriately sized for 375px screens.
  4. **Offline Resilience**: Stale-while-revalidate headers ensure previously loaded data remains accessible during network drops.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Optimizes the generation of the priority feed to return a smaller, high-signal chunk first.
  - **Content/Image Agent**: Automatically requests WebP generation for user-uploaded assets via the transcoder.

  ## Implementation Prompt
  Implement the mobile payload optimization and edge caching strategy as follows:
  1. Enhance the OpenResty configuration in `docker/nginx/nginx.conf` to implement aggressive caching for static assets with stale-while-revalidate headers.
  2. Add an image transcoding pipeline in the Rust backend to automatically convert user-uploaded images to WebP before storing them. Ensure image URLs served to the client default to the WebP versions.
  3. Update the Work Triage API endpoint to support pagination (e.g., cursor-based) and ensure the client requests data in small chunks.

  The user-facing outcome is a significantly faster initial load time and reduced data usage on mobile devices. Acceptance criteria include:
  - Static assets are served from the edge cache with cache-control headers.
  - Uploaded images are available as WebP.
  - The Work Triage feed API paginates data correctly.

  ## Estimated Scope
  Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
