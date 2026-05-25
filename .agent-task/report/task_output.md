issue_title: "Implement Invisible AI Media and Asset Intelligence Engine"
issue_description: |
  # Title: Invisible AI Media and Asset Intelligence Engine

  ## Problem Statement
  Small business owners like Maya (baker) and Fatima (food cart) struggle to present high-quality, professional imagery of their products. They often upload raw, unedited photos straight from their phone cameras (bad lighting, cluttered backgrounds, massive file sizes). This leads to slow storefront load times, poor SEO, and a non-premium feel that hurts conversion. Existing platforms require them to learn complex image editing tools or manage a separate CDN. They need an invisible media pipeline that automatically transforms their quick snapshots into stunning, performant, and accessible assets the moment they are uploaded from their phone, without ever touching an "edit" button.

  ## Research Report
  *   **Current Architecture Limits:** OHC currently handles image uploads but doesn't natively transform them into optimized, SEO-ready assets using AI.
  *   **Competitor Analysis:**
      *   *Shopify:* Offers basic image resizing and compression, but requires third-party apps for automated background removal or AI enhancement.
      *   *Wix:* Has built-in media tools, but they are manual (Studio Editor). Not fully autonomous.
      *   *Squarespace:* Good image optimization, but lacks generative AI upscaling or automatic alt-text generation for SEO.
  *   **Discovery:** We need an automated media pipeline that sits directly behind the mobile upload endpoint. It should use AI to instantly remove backgrounds, enhance lighting, generate semantic alt-text (critical for our AI Discovery Agent/GEO strategy), and cache at the edge for instant delivery.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MOBILE-CLIENT ||--o{ UPLOAD-ENDPOINT : "Uploads Raw Photo"
      UPLOAD-ENDPOINT ||--o{ MEDIA-ORCHESTRATOR : "Triggers"

      MEDIA-ORCHESTRATOR ||--o{ VISION-AI-AGENT : "Requests Enhancement"

      VISION-AI-AGENT {
          boolean background_removal
          boolean lighting_enhancement
          string generated_alt_text
      }

      VISION-AI-AGENT ||--o{ CDN-STORAGE : "Saves Optimized Variants"
      VISION-AI-AGENT ||--o{ CORE-LEDGER : "Updates Product Metadata"
      CDN-STORAGE ||--o{ EDGE-CACHE : "Distributes globally"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  *   **Merchant View (OHC Mobile App - 375px):**
      *   **Upload Interaction:** Maya taps "Add New Cake". She snaps a photo on her iPhone. The UI shows a Translucent Glass loading card with the text "✨ Optimizing your cake...".
      *   **Action Required Feed:** If the AI detects multiple items in the photo and is unsure which to isolate, it generates an approval card: "✨ We enhanced this photo. Keep the background or make it transparent?" using large, touch-friendly 44x44px buttons.
      *   **Grandmother Test:** The user never sees words like "Compression", "WebP", "CDN", or "Alt-Text". They just see their photo get magically better.

  ### Key Design Decisions
  *   **Asynchronous Processing:** The initial upload must feel instant. Heavy AI tasks (background removal) happen asynchronously. A lower-quality preview is shown instantly while the high-res AI version processes in the background.
  *   **Semantic SEO by Default:** The Vision AI Agent automatically generates rich alt-text (e.g., "Three-tier vegan chocolate wedding cake with floral decorations") and saves it to the product's metadata, directly feeding the AI Discovery Agent.
  *   **Multi-Tenant Isolation at Storage:** Asset paths in the CDN must be strictly segmented by `Tenant ID` and governed by signed URLs to prevent scraping of a competitor's pre-release product catalog.

  ### AI Agent Integration Points
  *   **Marketing Agent:** Consumes the generated alt-text to create social media captions and optimize GEO (Generative Engine Optimization).
  *   **Vision Agent:** A specialized background worker (part of Operations) dedicated to image/video analysis, formatting, and manipulation.

  ## Implementation Prompt
  Implement the Invisible AI Media and Asset Intelligence Engine.
  **Customer User Journey (CUJ):**
  1. The merchant uploads a raw 12MB JPEG from their mobile device.
  2. The system intercepts the upload and immediately generates a highly compressed preview thumbnail to unblock the UI.
  3. In the background, the Vision AI Agent analyzes the image, removes the background (if applicable/configured), enhances lighting, generates a WebP/AVIF variant, and writes an SEO-optimized alt-text string.
  4. The optimized assets are distributed to the edge CDN, and the product's metadata is updated.
  **Acceptance Criteria:**
  *   A raw image upload results in an automatically optimized asset available on the CDN within 5 seconds.
  *   The system automatically generates accurate alt-text and stores it in the database.
  *   Strict multi-tenant isolation is enforced at the storage bucket and CDN level.
  *   Do not prescribe specific AI vision APIs or CDN providers; design the interfaces to be swappable.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []