issue_title: "Architectural Gap: Zero-Setup Multimodal AI Media-to-Catalog Ingestion Engine"
issue_description: |
  ## Problem Statement
  For physical product owners (like Maya the Baker, Priya the Boutique Owner, and Fatima the Food Cart Operator), populating a digital storefront catalog is the highest point of friction. They operate entirely from their phones, capturing raw photos and videos of their products. Existing platforms (Shopify, Wix) require a multi-step desktop flow: transfer photos, edit out backgrounds, invent SEO descriptions, categorize variants, set prices, and upload them one by one. This 10-minute-per-item friction prevents SMBs from digitizing their inventory. OHC must enable a "Zero-Setup" flow where the owner simply points their phone camera or bulk-selects unedited camera roll photos, and AI autonomously provisions the catalog.

  ## Research Report
  - **Shopify & Square AI:** Provide photo background removal and description generation, but they still require navigating a complex, desktop-first web form and entering structured metadata manually for each product.
  - **Durable & 10Web:** Fast initial site generation, but lacking deep inventory management or batch processing for ongoing product additions via mobile devices.
  - **The OHC Differentiator:** A multimodal edge-ingestion engine that processes bulk mobile media, infers attributes (size, color, category, price), edits the images natively into premium glassmorphism layouts, and automatically builds variant schemas without the user ever touching a web form.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1.  **"Magic Ingest" Button:** A prominent action in the unified iOS/Android/Web PWA UI.
  2.  **Bulk Media Selection:** The owner selects 5-10 raw photos from their camera roll.
  3.  **Loading State (Agentic Processing):** A clean, translucent glass loading overlay (`backdrop-filter: blur(30px) saturate(210%)`) indicating the 'Catalog Department' agent is at work.
  4.  **Actionable Review List:** Instead of forms, a feed-like view appears showing beautifully isolated product images with AI-inferred titles, descriptions, and pricing (based on tenant history).
  5.  **One-Tap Publish:** The owner can tap "Approve All" or adjust a price directly inline via a native keyboard.

  ### AI Agent Integration Points
  -   **Vision AI Department:** Uses Gemini Pro Vision to extract metadata (object type, colors, inferred category).
  -   **Marketing Agent:** Generates SEO-optimized titles and descriptions.
  -   **Catalog Agent:** Normalizes the data against the `tenant_id` existing catalog, automatically grouping variants (e.g., recognizing 3 photos of the same shirt in different colors as a single product with variants).

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ MagicIngestJob : initiates
      MagicIngestJob ||--o{ MediaAsset : contains
      MediaAsset ||--|| VisionMetadata : extracts
      VisionMetadata ||--|| CatalogAgent : processes
      CatalogAgent ||--o{ Product : creates
      Product ||--o{ ProductVariant : infers
  ```
  -   **Multi-tenant Isolation:** `MagicIngestJob` and all subsequent writes use strict `tenant_id` RLS and SPIFFE identity.
  -   **Background Queue:** The upload triggers a Postgres `SKIP LOCKED` job queue. Agents process images asynchronously using distributed locks (Redis `ohc:lock:{tenant_id}:ingest:{job_id}`) to prevent race conditions on catalog updates.

  ## Implementation Prompt
  **Role:** Backend & Frontend Implementer
  **Task:** Implement the end-to-end "Magic Ingest" Multimodal Catalog Engine.
  **CUJ (Critical User Journey):** As a non-technical owner (e.g., Maya), I want to bulk-upload 5 raw cake photos from my phone and have the system automatically create 5 distinct, formatted catalog products with inferred descriptions and pricing, so I can start selling immediately without filling out forms.
  **Acceptance Criteria:**
  - Build the edge-capable media upload component in the mobile-first UI (`375px` optimized, using `.glassmorphism` tokens).
  - Implement the background Postgres job queue processing for media ingestion.
  - Coordinate the Vision and Marketing AI agents to extract metadata and write to the database.
  - Provide an inline, feed-based review screen for the owner to approve or reject the AI-generated catalog items.
  - Ensure zero mock data is used; all data must flow end-to-end via the API.
  - Ensure 100% unit and Playwright E2E coverage.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
