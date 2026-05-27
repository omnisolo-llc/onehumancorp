issue_title: "Implement Edge AI Media Optimization & Tagging Pipeline"
issue_description: |
  # Architecture Brief: Multi-Tenant Edge AI Media Optimization & Tagging Pipeline

  ## Title
  OHC Edge AI Media Optimization & Tagging Pipeline

  ## Problem Statement
  Small business owners like Maya (baker) and Fatima (food cart) rely heavily on visual catalogs to sell their products. They upload raw, high-resolution photos directly from their smartphones (often 5MB-15MB each), which drastically slows down their storefront's mobile load times, leading to cart abandonment. Furthermore, taking professional-looking photos is difficult and time-consuming; messy backgrounds detract from the perceived value of their products. Finally, non-technical owners do not know how to write SEO-optimized alt-text, limiting their organic discovery. They need an automated pipeline that instantly makes their photos web-ready, professional, and searchable without any manual editing.

  ## Research Report
  - **Performance Impact**: Large image payloads are the #1 contributor to high LCP (Largest Contentful Paint) times, especially on poor 3G/4G networks. A 1-second delay in page load can lead to a 7% reduction in conversions.
  - **Competitor Landscape**: Shopify provides basic resizing, but premium background removal and auto-tagging typically require paid third-party apps. Wix offers basic image enhancement but lacks deep generative AI capabilities natively integrated into the catalog workflow.
  - **AI Media Processing**: Edge-based media processing (e.g., Cloudflare Image Resizing) combined with asynchronous AI workflows (using models like Segment Anything for background removal and BLIP/LLaVA for captioning) can transform user-generated content into professional assets invisibly.
  - **Multi-Tenancy Security**: Media must be strictly isolated per tenant to prevent cross-contamination or unauthorized access to unreleased product photos.

  ## Design Doc

  ### Architecture and Entity-Relationship Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ PRODUCT : "owns"
      PRODUCT ||--o{ MEDIA_ASSET : "contains"
      MEDIA_ASSET ||--|{ MEDIA_VARIANT : "has"

      MEDIA_ASSET {
          uuid id
          uuid tenant_id
          string original_url
          string status "e.g., processing, ready, failed"
          string ai_generated_alt_text
          json ai_tags
      }

      MEDIA_VARIANT {
          uuid id
          string variant_type "e.g., thumbnail, webp_high, bg_removed"
          string cdn_url
          int width
          int height
      }

      AI_DEPARTMENT {
          string role "Marketing, Operations"
      }

      AI_DEPARTMENT ||--o{ MEDIA_ASSET : "analyzes & tags"
  ```

  ### Key Architectural Invariants
  1. **Asynchronous Processing Pipeline**: When a user uploads an image from the mobile app, the original file is securely stored in a tenant-isolated bucket. An event is published to the background job queue to process the image without blocking the user's workflow.
  2. **Edge Delivery & Optimization**: The frontend always requests images via an Edge CDN layer that automatically serves next-gen formats (WebP/AVIF) and dynamically resizes based on the client's `Accept` headers and viewport size.
  3. **AI Background Removal & Tagging**: An AI worker picks up the processing event, performs background removal (creating a transparent/clean variant), and generates SEO-friendly alt-text and categorization tags based on visual analysis.
  4. **Zero Trust Isolation**: All media assets and variants must be strictly tagged with the `tenant_id`. Access to original, pre-processed assets is restricted via signed URLs.

  ### UI Wireframes & Screen Flow (375px First)
  - **Upload Flow (Merchant App)**: Maya snaps a photo of a new cake on her iPhone. The app shows an optimistic UI: the photo appears instantly in the catalog with a subtle "Enhancing with AI..." shimmer effect.
  - **Magic Polish Mode**: Once processed (usually < 10 seconds), Maya receives a silent notification. When she taps the photo, she sees a toggle: "Original" vs "Magic Polish" (background removed, color corrected). "Magic Polish" is selected by default.
  - **Auto-SEO Fields**: The "Alt Text" and "Search Tags" fields are pre-filled by the AI (e.g., "Three-tier vegan chocolate wedding cake with floral decoration"). Maya can edit them but doesn't have to start from scratch.

  ### Mobile UX Flow
  - Merchant uploads an image -> Optimistic render in UI -> Background processing -> Push notification (silent) -> AI variants and metadata available.
  - For the consumer viewing the storefront, images load instantly (via Edge CDN) and look professionally shot, regardless of the merchant's original photography skills.

  ### AI Agent Integration Points
  - **The Marketing Agent**: Consumes the auto-generated tags and alt-text to seamlessly draft social media posts and improve the storefront's organic search ranking.
  - **The Operations Agent**: If the image analysis detects a specific type of product (e.g., "Perishable Food"), it can automatically prompt the merchant to add necessary shipping or handling tags.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the foundational Edge AI Media Optimization pipeline.
  1. Create a secure, tenant-isolated upload endpoint that accepts raw image files and returns an optimistic `MediaAsset` record.
  2. Implement an asynchronous worker (integrating with the background job queue) that listens for new media uploads.
  3. The worker should simulate (or implement using available APIs) the generation of optimized variants (WebP), an AI-enhanced variant (simulating background removal), and AI-generated alt-text/tags.
  4. Update the `MediaAsset` and `MediaVariant` database schemas to support this pipeline, ensuring strict multi-tenant isolation.
  5. Create a simple API endpoint to retrieve the optimized assets for a given product.
  Do NOT prescribe specific background removal libraries or cloud storage providers; focus on the robust asynchronous pipeline and data model.
  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
