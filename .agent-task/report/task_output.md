issue_title: "Implement AI-Powered Zero-Touch Catalog & Inventory Importer (The Insta-Catalog Bridge)"
issue_description: |
  # AI-Powered Zero-Touch Catalog & Inventory Importer (The Insta-Catalog Bridge)

  ## Problem Statement
  Small business owners like Maya (Home Baker) and Priya (Boutique Operator) often start their businesses on social media (Instagram, WhatsApp, TikTok). When they migrate to a professional platform to handle deposits, variants, and booking workflows, the biggest friction point is data entry. Recreating a catalog of 200+ cakes or boutique items manually on a 375px mobile screen is painful and slow. They need an AI-powered bridge that can securely connect to their social feeds, visually analyze their posted products, extract implied pricing/variants from captions, and generate a fully structured OHC catalog with zero manual data entry.

  ## Research Report
  **Market Competitive Analysis:**
  - **Shopify/Wix:** Rely heavily on CSV imports or app ecosystem plugins for migration. CSVs are technical and completely fail the "mobile-first" and "non-technical owner" test. Maya does not know what a CSV is.
  - **Square:** Offers basic scraping but lacks visual AI to understand what an item is (e.g., categorizing a "Red Velvet Tiered Cake" with appropriate variant tags).
  - **Current OHC Capability:** Currently requires manual product creation via standard forms.

  **Our Opportunity:**
  OHC can leverage its Vision LLM capabilities (Gemini Pro Vision / GPT-4o) to turn a mere Instagram handle or a batch of uploaded photos into a fully categorized, variant-mapped, and SEO-optimized storefront in 60 seconds. This capability transforms onboarding from a chore into a "magic moment" that proves the value of an AI-powered assistant immediately.

  ## Design Doc
  ### Core Architectural Concepts
  1. **Social Ingestion Gateway:** A service layer that authenticates and fetches media/captions from platforms (Instagram Basic Display API, WhatsApp Business API) or accepts bulk mobile uploads.
  2. **Vision-Language Processing Pipeline:** An asynchronous background worker queue that feeds images and captions to the Vision LLM to extract: Product Name, Description, Inferred Price, Categories, and Variants (Size, Flavor, Color).
  3. **Optimistic Draft Catalog:** The AI generates a "Draft" catalog. The user reviews this on mobile in a swipeable, Tinder-like UI (Approve, Edit, Discard) to quickly validate the AI's work.
  4. **Multi-Tenant State Transition:** Once approved, the draft items are committed to the main `CATALOG_ITEM` PostgreSQL tables with full tenant isolation.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner (Mobile UI)
      participant Ingestion API (Go)
      participant AI Worker Queue (Postgres/Redis)
      participant Vision LLM (Gemini)
      participant Catalog DB (Postgres)

      Owner->>Ingestion API: Provide Instagram Handle / Upload Photos
      Ingestion API->>AI Worker Queue: Enqueue Ingestion Task
      AI Worker Queue->>Vision LLM: Analyze Image & Caption
      Vision LLM-->>AI Worker Queue: JSON: {Name, Price, Variants}
      AI Worker Queue->>Catalog DB: Save as 'DRAFT' Status
      AI Worker Queue-->>Owner: Notification: "Your catalog is ready to review!"
      Owner->>Catalog DB: Review & Approve Drafts
      Catalog DB->>Catalog DB: Transition to 'ACTIVE'
  ```

  ### Mobile-First UX Flow (375px Viewport)
  1. **Trigger:** In the Catalog tab, tap the magic wand icon: "Auto-Build from Instagram".
  2. **Loading State:** A translucent glass card shows real-time progress: "Analyzing 45 photos... Found 12 cupcakes... Generating variants...".
  3. **Review Swiper:** A stack of cards appears. Each card shows the photo, the AI-generated title, and detected variants. The owner swipes right to approve, left to discard, or taps to edit.
  4. **Completion:** A confetti animation plays, and the approved items instantly populate the live OHC storefront and inventory systems.

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Goal:** Build the AI-Powered Catalog Importer pipeline and mobile review UI.

  **Acceptance Criteria:**
  1. Create the `CatalogImportJob` and `DraftCatalogItem` schemas with strict `tenant_id` isolation.
  2. Implement an asynchronous worker (using PostgreSQL SKIP LOCKED) that processes images using the configured Vision LLM.
  3. Build the Flutter UI for the mobile-first "Draft Review" swiper (must work flawlessly at 375px width).
  4. Ensure end-to-end integration: user provides images -> AI extracts data -> user approves -> active catalog is populated.
  5. Include E2E Playwright tests that mock the social ingestion but verify the full AI parsing and UI approval flow.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
