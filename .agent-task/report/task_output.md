issue_title: "Implement Invisible AI Media Processing & Edge Delivery Engine"
issue_description: |
  ## Title
  Implement Invisible AI Media Processing & Edge Delivery Engine

  ## Problem Statement
  Small business owners (like Maya the baker and Carlos the handyman) frequently upload high-resolution, unoptimized photos and videos directly from their mobile devices. They do not know how to resize, compress, or convert formats (like WebP/AVIF), nor should they need to. Currently, this leads to slow page loads, poor SEO, and high bandwidth costs, negatively impacting the buyer's experience. We need an invisible, zero-config engine that automatically ingests, processes (compression, format conversion, smart cropping), and serves media globally via an edge network without ever blocking the user's workflow or requiring technical knowledge.

  ## Research Report
  *   **Current State in OHC**: Media uploads are handled synchronously or with minimal processing, leading to large file sizes on storefronts.
  *   **Shopify/Wix Comparison**:
      *   **Shopify**: Automatically uses WebP and serves via a global CDN. They offer dynamic image transformation via URL parameters (e.g., `_100x100.jpg`).
      *   **Wix**: Uses a sophisticated media manager that automatically optimizes images and videos upon upload, serving them via CDN.
  *   **Opportunity**: OHC can differentiate by integrating AI-driven smart cropping (e.g., automatically centering on the cake in Maya's photo) and background processing that never interrupts the mobile-first creation flow, keeping the UX entirely invisible to the user.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD;
      MobileClient[Mobile App / Web UI] -->|Direct Upload| UploadBucket[(Raw S3 Bucket)];
      UploadBucket -->|Event: Object Created| EventBridge[Event Router];
      EventBridge -->|Trigger| MediaProcessor[AI Media Processing Agent];
      MediaProcessor -->|AI Smart Cropping| VisionAI[Vision AI Service];
      MediaProcessor -->|Format Conversion\nCompression| ImageMagick[Media Toolkit];
      VisionAI --> MediaProcessor;
      ImageMagick --> MediaProcessor;
      MediaProcessor -->|Save Processed Assets| EdgeBucket[(Optimized S3 Bucket)];
      EdgeBucket --> CDN[Global Edge CDN];
      CDN --> EndUser[Storefront Visitor];
      MediaProcessor -->|Update DB| Postgres[(Main DB: Asset Registry)];
  ```

  ### Mobile UX Flow
  1.  **User Action**: Maya taps "Add Photo" on her cake product page.
  2.  **Upload**: She selects a 10MB photo from her iPhone camera roll.
  3.  **Instant Feedback**: The UI immediately shows a placeholder/thumbnail and allows her to continue editing the product details. *Crucially, she does not wait for processing.*
  4.  **Background Processing**: The image uploads directly to cloud storage. The AI Media Processing Agent wakes up, identifies the focal point (the cake), generates responsive sizes (thumbnail, mobile, desktop), and converts to WebP.
  5.  **Completion**: The asset registry is updated. When a customer views the storefront, the optimized WebP is served instantly from the nearest edge node.

  ### UI Descriptions (macOS/Ubiquiti Design)
  *   **Media Upload Card**: A clean, translucent glass card with a subtle drop shadow. It features a large, friendly "+" icon.
  *   **Progress Indication**: A thin, glowing indeterminate progress bar at the bottom of the card during upload, transitioning to a soft pulse while the "AI is enhancing" the image.
  *   **Advanced Settings (Hidden)**: Deep within the store settings, an "Advanced Media" toggle (default off) allows tech-savvy users to disable auto-cropping or specific format conversions, maintaining the "grandmother test" for the primary flow.

  ### AI Agent Integration Points
  *   **Vision AI Department**: Analyzes the image to detect the subject (e.g., face, product) and determines the optimal crop coordinates.
  *   **Operations Agent**: Monitors the asynchronous processing queue. If a processing task fails (e.g., corrupted file), it gracefully retries or leaves a plain-language note in the user's Inbox ("We had trouble processing one of your photos. You might need to re-upload it.").

  ## Implementation Prompt
  **User-Facing Outcome**: Storefronts load blazingly fast because all uploaded media is automatically optimized and served via CDN. Users never have to think about file sizes, formats, or cropping.

  **Core User Journeys (CUJs)**:
  *   User uploads a large (10MB+) image file via the mobile UI.
  *   The system instantly accepts the upload and allows the user to proceed.
  *   The system invisibly processes the image into multiple responsive sizes and modern formats (WebP).
  *   The system applies AI smart cropping if needed.
  *   The storefront serves the optimized image via CDN.

  **Acceptance Criteria**:
  1.  Uploads are asynchronous; the UI never blocks while waiting for processing.
  2.  Images are automatically converted to WebP format.
  3.  Images are resized into predefined responsive breakpoints.
  4.  AI smart cropping is applied to ensure the main subject is centered in thumbnails.
  5.  Processed images are served via a CDN.
  6.  The database asset registry is accurately updated with the new URLs.
  7.  The system handles processing failures gracefully without crashing the main application.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []