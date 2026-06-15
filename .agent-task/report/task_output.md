issue_title: "[Architecture] Decentralized Edge-Computed Image Optimization"
issue_description: |
  # Decentralized Edge-Computed Image Optimization

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) and Maya (Home Baker) upload high-resolution photos directly from their modern smartphones (often >5MB per image). When these images are served unmodified to customers on slow mobile networks (like Fatima the Food Cart Operator's customers), page load times soar, leading to abandoned carts and lost revenue. OHC currently lacks an edge-computed image optimization pipeline, forcing the platform to choose between expensive centralized processing or slow customer experiences.

  ## Research Report
  Our competitive analysis indicates that platforms like Shopify and Wix utilize centralized CDNs with built-in image optimization (e.g., Cloudflare Image Resizing, Fastly). However, this approach incurs significant recurring compute costs at scale.

  *   **Shopify:** Uses a proprietary CDN that automatically serves WebP/AVIF based on the `Accept` header. Excellent performance, but entirely centralized.
  *   **Squarespace:** Similar approach, heavy reliance on Fastly.
  *   **OHC Opportunity:** By leveraging the capabilities of modern browsers during the upload phase (client-side resizing/compression before upload) combined with a lightweight edge-caching layer (Cloudflare Workers/Fastly Compute@Edge) for legacy clients or unsupported formats, OHC can drastically reduce both storage costs (GCS/MinIO) and CDN bandwidth while delivering near-instant image loads.

  ### Target Personas Affected:
  *   **Maya (Baker):** Uploads large 4K photos of custom cakes. Customers browsing her Instagram-linked portfolio on 3G need these optimized instantly.
  *   **Priya (Boutique):** Needs to upload dozens of product variants quickly without hitting an arbitrary "file too large" error.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Owner
      participant PWA as OHC Mobile App (PWA/Flutter)
      participant Storage as GCS / MinIO
      participant Edge as Edge CDN (Cloudflare/Fastly)
      actor Customer

      Owner->>PWA: Select High-Res Image (10MB)
      Note over PWA: Client-Side Optimization<br/>(Resize to max 2048px, compress to WebP, ~200KB)
      PWA->>Storage: Upload Optimized WebP
      Storage-->>PWA: Return Image URL
      Customer->>Edge: Request Image (Mobile Viewport)
      Note over Edge: Edge Cache & Content Negotiation<br/>(Check Accept header for AVIF/WebP)
      Edge->>Storage: Fetch Origin (Cache Miss)
      Storage-->>Edge: Return WebP
      Edge-->>Customer: Serve Optimized Image (Fast Load)
  ```

  ### Mobile UX Flow (375px First)
  1.  **Image Selection:** The owner taps "Add Photo" in the OHC app or PWA. They select a massive photo from their camera roll.
  2.  **Instant Feedback:** Instead of a long loading spinner, the image instantly appears in the UI (using a local `Blob` URL).
  3.  **Background Processing:** A subtle progress bar indicates background uploading. The app transparently resizes the image down to a maximum dimension (e.g., 2048px on the longest edge) and compresses it to WebP format *before* the network request begins.
  4.  **Completion:** The progress bar completes quickly because the payload size was reduced by 90%.

  ### AI Agent Integration Points
  *   **Marketing Assistant:** When generating social media posts or email newsletters, the agent can explicitly request specific optimized variants (e.g., `?width=600&format=jpeg` for legacy email clients) directly from the Edge CDN.
  *   **Operations Assistant:** Can flag products that have missing or low-quality images and prompt the owner to upload better ones.

  ### Key Design Decisions
  1.  **Shift Left Compression:** Perform the heaviest lifting (resizing/compression) on the owner's powerful smartphone CPU rather than on OHC servers. This saves bandwidth, storage, and compute costs.
  2.  **WebP by Default:** Standardize on WebP for origin storage to provide an optimal baseline of quality vs. size.
  3.  **Graceful Degradation:** The Edge CDN layer ensures compatibility by serving JPEGs if a legacy browser requests the image and lacks WebP support.

  ## Implementation Prompt
  **Objective:** Implement the client-side image optimization pipeline for the OHC frontend (Flutter/PWA) and document the required Edge CDN configuration.

  **User Journey:**
  - Maya takes a 12MB photo of a new cake on her iPhone.
  - She opens the OHC app and adds the photo to a new product listing.
  - The app instantly displays the photo and quickly uploads a highly compressed, high-quality WebP version (~300KB) in the background.
  - A customer taps Maya's link on Instagram; the image loads instantly on their mobile connection.

  **Acceptance Criteria:**
  1.  Implement a client-side utility in the frontend codebase (Flutter or TS/PWA) that intercepts image uploads.
  2.  The utility must resize images exceeding 2048px (on the longest edge) while maintaining aspect ratio.
  3.  The utility must convert the image to WebP format with a quality setting that balances visual fidelity and file size (e.g., 80%).
  4.  The uploaded payload sent to the backend API must be the optimized WebP, not the original high-res image.
  5.  Ensure the UI remains responsive during the compression process (e.g., using Web Workers in PWA or Isolates in Flutter).
  6.  Add E2E tests verifying that a large mock image upload results in a smaller WebP payload being sent over the network.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
