issue_title: "[Research] Autonomous Multi-Platform Product Syndication Engine"
issue_description: |
  # [Research] Autonomous Multi-Platform Product Syndication Engine

  ## Problem Statement
  Small business owners and creators (like Priya the boutique operator and Leo the creator) need to reach customers wherever they are—Instagram Shops, TikTok Shop, Google Shopping, Amazon, and their own OHC storefront. However, manually creating, formatting, syncing, and updating product listings across multiple disparate platforms is a massive time sink. When Priya sells the last size M dress in-store via tap-to-pay, she shouldn't have to manually unlist it from Instagram and TikTok to avoid angry customers. They need an AI-driven syndication engine that automatically formats listings for each platform, pushes updates, and universally syncs inventory the millisecond a transaction occurs anywhere.

  ## Research Report

  ### Competitive Landscape
  *   **Shopify Multichannel / Feedonomics**: Powerful, but highly technical. Requires configuring complex XML/CSV feeds, mapping attributes manually, and managing API keys. Built for dedicated e-commerce managers, not solopreneurs on a 375px phone screen.
  *   **Ecwid / BigCommerce**: Better native integrations, but still require significant setup friction. They act as passive conduits rather than active, intelligent agents that optimize the listing for the specific platform's algorithm.
  *   **Link-in-bio (Linktree/Stan)**: Excellent for creators (like Leo), but fundamentally limited to redirecting traffic to a single checkout destination, rather than native, in-platform shopping experiences (like TikTok Shop).

  ### The OHC Gap
  Reviewing the OHC architecture, we have foundations for a Unified Capacity & Inventory Mesh and a robust local ledger. We lack the intelligent abstraction layer—the Syndication Engine—that allows an AI agent to take a single, rough product input from an owner ("Here's a photo of the new summer dress, $50, sizes S-L") and autonomously generate the optimized descriptions, format the imagery, and push the live listings to Meta, TikTok, and Google, while maintaining an unbreakable, real-time sync with the central OHC inventory mesh.

  ## Design Doc

  ### Architecture Overview

  ```mermaid
  erDiagram
      TENANT ||--o{ UNIFIED_PRODUCT : owns
      UNIFIED_PRODUCT ||--o{ PLATFORM_LISTING : syndicates_to
      UNIFIED_PRODUCT ||--|{ INVENTORY_LEDGER : tracks_availability
      PLATFORM_LISTING ||--o{ SYNC_EVENT : generates

      UNIFIED_PRODUCT {
          uuid id PK
          uuid tenant_id FK
          string base_name
          string base_description
          decimal base_price
          jsonb media_assets
      }

      PLATFORM_LISTING {
          uuid id PK
          uuid product_id FK
          string platform_id "e.g., meta_ig, tiktok, google"
          string platform_external_id
          string optimized_title
          string optimized_description
          string sync_status "PENDING, ACTIVE, FAILED"
      }

      INVENTORY_LEDGER {
          uuid id PK
          uuid product_id FK
          integer delta
          string source "e.g., pos, tiktok, web"
      }
  ```

  ### Mobile UX Flow (375px)
  1.  **Input:** The owner snaps a photo of a new item or pastes a rough description into the AI Assistant chat.
  2.  **Drafting:** The Marketing Agent analyzes the image/text and presents a clean "Product Card." It proposes optimized titles/descriptions tailored for Instagram, TikTok, and the Web.
  3.  **Approval:** A simple, translucent toggle list appears: "Publish to: [x] Web [x] Instagram [x] TikTok". The owner taps "Approve & Publish."
  4.  **Invisible Execution:** The engine translates the request into platform-specific API payloads, handles image resizing/compression (WebP/JPEG as required), and pushes the listings.
  5.  **Monitoring:** The home feed shows a simple status pill: "Summer Dress live on 3 platforms."

  ### AI Agent Integration
  *   **Marketing Department (Promoter Agent):** Triggers the pipeline. It uses vision models to extract attributes from images and LLMs to write platform-specific, SEO-optimized copy (e.g., hashtag-heavy for Instagram, keyword-rich for Google).
  *   **Operations Department:** Listens for webhook events from external platforms (e.g., an order placed on TikTok) and instantly writes a transactional deduction to the central `INVENTORY_LEDGER`, which in turn triggers a sub-agent to push inventory zero-outs to the other platforms.

  ## Implementation Prompt
  Implement the backend core for the Multi-Platform Product Syndication Engine.

  **User Journey:**
  A non-technical boutique owner (Priya) uploads a product image and basic price to the OHC assistant. The system must autonomously create a `UNIFIED_PRODUCT`, generate platform-specific metadata (title, tags, formatted description) for at least two mocked external platforms (e.g., `META_IG`, `TIKTOK`), and store these as `PLATFORM_LISTING` records. When an inventory deduction occurs (mocked via an internal API call representing a sale), the engine must autonomously update the status of the `PLATFORM_LISTING` records to reflect the new inventory state, handling out-of-stock scenarios gracefully.

  **Acceptance Criteria:**
  1.  Create the necessary Rust/Go data models (or database migrations) for `UnifiedProduct` and `PlatformListing` with strict tenant isolation (RLS).
  2.  Implement an API endpoint (or gRPC service) to ingest a raw product request and trigger the syndication pipeline.
  3.  Implement a background worker (or actor) that simulates pushing the listing to the external platforms and updating the `sync_status`.
  4.  Implement an inventory decrement endpoint that correctly cascades state changes to all associated `PlatformListing` records.
  5.  100% unit test coverage for the syndication logic and inventory cascade. No mock data in user-facing UI, but external platform APIs should be safely abstracted via interfaces.

  ## Priority
  P1 (High) - Crucial for unlocking multi-channel revenue for core personas like Priya and Maya.

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
