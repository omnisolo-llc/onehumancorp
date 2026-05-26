issue_title: "Universal Media CDN & Generative Asset Pipeline"
issue_description: |
  **Problem Statement**
  Small business owners, especially visual businesses like bakers (Maya), boutique owners (Priya), and food cart operators (Fatima), rely entirely on imagery to sell their products. However, they lack the technical skills to resize, crop, format (e.g., WebP), or compress images for the web. They upload massive, raw smartphone photos that break mobile storefront layouts and ruin page load times, directly impacting their sales and search ranking. Furthermore, they need AI to remove backgrounds or generate missing product shots. They need a system that invisibly intercepts any uploaded media, automatically optimizes it for global, low-latency mobile delivery, and provides generative capabilities (background removal, upscaling) without ever showing them a settings menu or asking them about resolution.

  **Research Report**
  *Competitive Landscape*
  - **Shopify**: Features robust CDN delivery but relies on third-party apps for advanced generative AI background removal or dynamic resizing.
  - **Wix/Squarespace**: Standard media libraries with built-in compression. Good enough, but often slow to apply modern formats like AVIF or WebP dynamically, and lacks deep AI integration in the core media flow.
  - **Cloudinary / Imgix**: Powerful developer-first tools. Overkill and completely inaccessible for a non-technical SMB owner.

  *The OHC Gap*
  OneHumanCorp needs an invisible, multi-tenant media processing and edge delivery architecture. When Maya uploads a 12MB HEIC photo from her iPhone, the system must instantly store the raw asset, generate responsive WebP variants, and attach them to her catalog entry, all while maintaining strict Zero-Trust tenant isolation.

  **Design Doc**

  *Architecture Diagram*
  ```mermaid
  sequenceDiagram
      participant User as OHC Mobile App (Maya)
      participant Edge as OHC Edge / Gateway
      participant Ingest as Media Ingest Service
      participant Storage as Raw Asset Vault (S3/Tenant-Isolated)
      participant AI as Generative AI Pipeline
      participant CDN as OHC CDN Edge (Cloudflare/Fastly)

      User->>Edge: Upload raw 12MB HEIC photo
      Edge->>Ingest: Forward payload with SPIFFE identity
      Ingest->>Storage: Store original (Zero-Trust boundaries)
      Ingest->>AI: Async request: crop, WebP, bg-removal
      AI-->>Storage: Store optimized variants
      AI-->>Ingest: Return asset metadata (URLs)
      Ingest-->>Edge: Return CDN ready URLs
      Edge-->>User: Display optimized image in UI instantly
      User->>CDN: Request image via storefront
      CDN-->>User: Deliver cached WebP at edge
  ```

  *Entity-Relationship Diagram*
  ```mermaid
  erDiagram
      TENANT ||--o{ MEDIA_ASSET : uploads
      MEDIA_ASSET ||--o{ ASSET_VARIANT : generates

      TENANT {
          string id PK
          string plan
      }
      MEDIA_ASSET {
          string id PK
          string tenant_id FK
          string original_url
          string blur_hash
          float size_mb
          timestamp created_at
      }
      ASSET_VARIANT {
          string id PK
          string asset_id FK
          string format "webp | avif | jpeg"
          int width
          int height
          string cdn_url
      }
  ```

  *Mobile UX Flow (375px)*
  - **Upload**: Large, thumb-friendly "Add Photo" tap area.
  - **Processing**: A smooth, non-blocking skeleton loader or blur-hash placeholder immediately replaces the tap area while processing happens in the background. No progress bars with technical percentages.
  - **Generative Action**: A single "Magic Retouch" button (using translucent glass styling) that triggers background removal.
  - **Parity**: All functions (upload, retouch, crop) must perform smoothly on low-end Android (Fatima) and high-end iPhone (Maya).

  *Key Decisions*
  - **On-the-fly vs. Pre-processing**: Adopt a hybrid approach. Pre-process standard responsive sizes (thumbnail, mobile full, desktop) asynchronously. Generate specific crops on-the-fly via Edge functions if requested, caching the result.
  - **Zero-Trust Multi-Tenancy**: All media must be logically and physically partitioned by `tenant_id`. Presigned URLs must enforce expiration and scope.
  - **Invisible AI**: AI background removal is a background task, not a separate tool.

  **Implementation Prompt**
  *For Implementer Agent:*
  Implement the `MediaIngestService` and edge delivery rules for the Universal Media Pipeline.
  - **User Journey**: An SMB owner (e.g., Maya) uploads a product photo from her mobile phone. The system must accept the upload, securely store the raw file respecting multi-tenant isolation, automatically generate WebP/AVIF variants, and return a set of CDN-ready URLs for responsive mobile display.
  - **Acceptance Criteria**:
      1. The API endpoint accepts standard image uploads.
      2. Tenant isolation is mathematically guaranteed (e.g., specific bucket paths/keys validated by identity).
      3. The service triggers background jobs for resizing and format conversion.
      4. The response includes a blur-hash and URLs for thumbnail and full-size variants.
      5. Ensure the design supports high-performance edge caching.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
