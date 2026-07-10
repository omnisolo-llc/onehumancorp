issue_title: "[Research] Dynamic Centralized Inventory and OHC POS Multi-Channel Engine Architecture"
issue_description: |
  # Research Report: Dynamic Centralized Inventory and OHC POS Multi-Channel Engine

  ## Track 1: Architectural Gap & Scaling Discovery

  ### Codebase & Docs Audit
  Our existing research (`docs/business/market_research/ohc_smb_market_report.md` and `docs/reports/ohc_smb_platform_research_report.md`) strongly identifies **Inventory Sync Across Channels** as a top-10 pain point for SMBs (Pain Point #4). Users like Priya (Boutique Operator) complain about items selling out online but still showing in-store, or vice-versa.
  Presently, OHC lacks a robust, multi-tenant, centralized inventory tracking system capable of real-time multi-channel synchronization, which is a critical piece of the core platform structure. There is an opportunity to design an architecture that spans online and in-person point-of-sale (POS) systems.

  ### Competitor Systems Audit
  Leading platforms like **Shopify** and **Square** excel at having a unified source of truth for inventory. Square is particularly successful in bridging the gap between physical retail POS and online e-commerce seamlessly, acting as the centralized inventory ledger. In contrast, smaller builders (Wix, Squarespace) often struggle with true centralized real-time synchronization, leading to double-selling.

  ### Identifying Gaps
  OHC needs a **Unified Ledger for Inventory**. The current platform needs an architecture that treats online sales, social DM sales, and physical in-person sales (via Tap-to-Pay or POS) as equal consumers and mutators of a single truth source for stock counts.

  ## Track 2: Selected Architecture Deep Dive

  ### Business Journey Mapping
  **Persona:** Priya (Boutique Operator)
  - **Journey:** Priya adds a new summer dress variant. She sells one via the OHC online storefront, and another in-person via the OHC Mobile App Tap-to-Pay.
  - **Friction to eliminate:** Priya shouldn't need to manually update stock counts. If an item sells in-store, the online storefront needs to immediately mark it "Sold Out" if stock reaches zero. If a customer tries to buy it on IG DMs, the AI agent needs to know it's unavailable immediately.

  ### Data Model & Invariants
  The system will introduce a Centralized Inventory Ledger approach.

  ```mermaid
  erDiagram
      Tenant ||--o{ Product : owns
      Product ||--o{ Variant : has
      Variant ||--o{ InventoryLevel : has
      Location ||--o{ InventoryLevel : maintains
      InventoryLevel {
          string variant_id
          string location_id
          int available_count
          int committed_count
      }
      Transaction ||--o{ InventoryTransaction : triggers
      InventoryTransaction {
          string type (sale, restock, return)
          int quantity_change
      }
  ```
  **Multi-tenant rules:** All operations MUST be scoped via `tenant_id` and rely on strict row-level security.
  **Invariants:** `available_count` must never drop below 0 unless explicit backordering is enabled.

  ### AI Department Coordination
  - **Operations Agent:** Monitors `InventoryLevel`. When stock dips below a threshold, it drafts a supplier reorder email.
  - **Sales/Customer Agent:** When classifying intents for DMs (e.g., "Do you have the dress in Medium?"), it performs a real-time check against the Centralized Inventory before drafting a positive reply.

  ## Track 3: Technical Integrity & Mobile-First Review

  ### Mobile-First UX Flow
  - **Viewport (375px):** Priya's OHC app has an "Inventory" tab. She sees a clean list of variants with big +/- buttons for manual adjustments.
  - **Tap-to-Pay POS:** A dedicated mobile view allowing her to select products, add to cart, and process a physical payment. This immediately deducts from the shared `InventoryLevel`.

  ### Performance & Offline Targets
  - **Real-time Sync:** Uses Redis pub/sub or WebSockets to push inventory updates to active connected clients (e.g., stopping an online checkout if the last item is bought in-store).
  - **Offline Capability:** The POS view must queue transactions locally (SQLite/IndexedDB) if the network drops, syncing and reconciling `InventoryTransaction` ledgers when back online.

  ## Track 4: Strategic Feature Issue Dispatch (Implementation Prompt)

  **Prompt for Implementer Agent:**
  Design and implement the core `Inventory Service` and its corresponding mobile-first UI for OneHumanCorp.

  **Outcome:**
  A unified inventory system where stock changes (via manual update, online sale, or simulated POS transaction) instantly reflect across the platform. An AI agent should be able to query this stock level to answer customer DMs accurately.

  **Critical User Journey (CUJ):**
  1. The business owner (Priya) opens the mobile UI (375px).
  2. Priya navigates to the Inventory section and sees a list of products.
  3. Priya manually adjusts the stock of "Summer Dress - Medium" from 5 to 4.
  4. Priya opens a "POS" simulator view, processes an in-person sale for the dress, and the stock goes from 4 to 3.
  5. The UI updates instantly.

  **Acceptance Criteria:**
  - Build the backend inventory ledger logic respecting `tenant_id` isolation.
  - Build a 375px-optimized frontend using OHC Premium Token principles (translucent glass, clean typography).
  - Include Playwright E2E tests verifying the stock adjustment and POS transaction flow end-to-end.
  - NO mock data in the UI; state must be driven by the real backend.
  - Ensure 100% unit test coverage for new backend logic.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
