issue_title: "Implement Offline-Tolerant Edge-Cached POS & Native Tap-to-Pay for In-Person Workflows"
issue_description: |
  ## Mission Queue Protocol Brief

  **Title:** Implement Offline-Tolerant Edge-Cached POS & Native Tap-to-Pay for In-Person Workflows

  ### Problem Statement
  Small business operators like Fatima (food cart, slow mobile data), Priya (boutique, in-store sales), and Carlos (field services, mobile-only) rely heavily on in-person physical interactions. When they are at a pop-up market, rural client site, or their own store, they cannot afford a failure of their POS system due to unstable network conditions or high latency. Furthermore, requiring dedicated hardware or complex pairing for card payments increases the barrier to entry. They need a system that caches the catalog locally, supports Tap-to-Pay directly on their mobile device without extra hardware, and syncs reliably in the background when connectivity returns. The current OHC stack lacks robust offline capabilities for its catalog operations and native mobile payment capabilities.

  ### Research Report
  Our analysis of the existing OHC research (e.g., `[research]_universal_edge_cached_dynamic_storefront_seo.md`, `[research]_ohc_centralized_inventory_pos.md`, and `ohc_smb_market_report.md`) highlights the crucial importance of high-performance localized caching and an omni-channel point-of-sale.
  - **Competitor Insights:** Systems like Square and Shopify POS offer in-person sales but suffer heavily when connectivity drops. Stripe's Tap-to-Pay terminal SDK has enabled modern platforms to bypass proprietary hardware, using NFC on standard iOS and Android devices.
  - **The Gap:** OHC needs an invisible, agent-orchestrated architecture where the catalog and pending orders are edge-cached (using technologies like JourneyApps PowerSync or SQLite local cache with Valkey synchronization). The system should automatically fallback to local storage during network interruptions and seamlessly submit the queued orders when the connection is restored. Tap-to-Pay integration using Stripe Terminal must be natively wired into the mobile-first UX.

  ### Design Doc
  **Architecture Overview**
  - **Local State & Sync:** The mobile shell (Flutter/PWA) uses a local SQLite database that mirrors the core catalog, pricing, and variant data from the primary PostgreSQL database. A background sync agent powered by PowerSync or custom WebSockets coordinates delta updates.
  - **Payment Integration:** Stripe Terminal SDK for iOS/Android handles the secure Tap-to-Pay interactions via the device's NFC chip, eliminating extra card readers. Payments requested offline are securely queued or processed using store-and-forward methods depending on provider limits.
  - **Multi-Tenant Boundaries:** All cached records strictly enforce the `tenant_id`. Redis locks (`ohc:lock:{tenant_id}:inventory:{variant_id}`) ensure that the AI Operations Assistant validates final stock counts to prevent overselling across online and offline channels when connection returns.

  **Architecture Diagram (Mermaid.js)**
  ```mermaid
  graph TD
      subgraph Mobile Client "375px Flutter Shell"
          UI[POS Cart UI]
          LocalDB[(Local SQLite/PowerSync)]
          Terminal[Stripe Terminal SDK - Tap-to-Pay]
          SyncManager[Offline Queue & Sync Engine]
      end

      subgraph Cloud Backend "Go + Bazel"
          API[gRPC / REST API]
          OperationsAgent[AI Operations Assistant]
          FinanceAgent[AI Finance Assistant]
          MainDB[(PostgreSQL - Tenant Isolated)]
          Cache[(Valkey/Redis - Locks)]
      end

      UI -->|Query Catalog| LocalDB
      UI -->|Process Payment| Terminal
      Terminal -->|NFC/Card| SyncManager
      SyncManager -->|Sync Deltas| API
      API -->|Validate Inventory| OperationsAgent
      OperationsAgent --> MainDB
      OperationsAgent --> Cache
      API -->|Record Revenue| FinanceAgent
      FinanceAgent --> MainDB
  ```

  **Mobile UX Flow (375px First)**
  1. **Home Shell:** Owner opens OHC app. "Start POS" action is prominently displayed above the AI triage feed.
  2. **Catalog View:** A clean, image-led grid (e.g., Fatima's menu or Priya's variants). Fast scrolling, loaded from the local DB. No loading spinners.
  3. **Cart & Checkout:** Tapping items adds to a bottom-sheet cart. Tapping "Charge $X.XX" brings up a clear, full-screen payment modal.
  4. **Tap-to-Pay Modal:** The screen uses a translucent glass styling. A clear animated NFC icon tells the customer to "Tap card or phone here". No external hardware is shown.
  5. **Offline State:** If network is unavailable, a subtle but truthful "Saved to Sync later" badge appears, and the app allows the next order immediately.

  **AI Agent Integration Points**
  - **Operations Assistant:** When local orders are synchronized back to the server, this agent deducts global inventory, updates the daily summary feed, and alerts the owner if any offline-overselling occurred (recommending a refund or substitute).
  - **Finance Assistant:** Generates automated daily settlement reports tracking "in-person Tap-to-Pay" vs "online web" revenue.

  ### Implementation Prompt
  **To the Implementer:**
  Implement the Offline-Tolerant Edge-Cached POS capability.
  - **CUJ:** A non-technical owner (like Carlos or Fatima) opens the OHC mobile shell, navigates to the POS screen, selects products from their catalog without a network connection, and processes an in-person payment via Stripe Tap-to-Pay. The system caches the transaction locally and securely syncs the order to the backend API when connectivity resumes.
  - **Acceptance Criteria:**
    1. The catalog must load from local storage instantly (<100ms) with zero network dependency on the critical read path.
    2. The UI must cleanly indicate offline status and queue checkout operations resiliently.
    3. The frontend must integrate a mocked or actual tap-to-pay interface representing the Terminal interaction, conforming strictly to a 375px width design using translucent materials and clear touch targets (44x44px minimum).
    4. The backend must provide an endpoint to accept synced offline batches and the AI Operations Agent must process these orders, applying `tenant_id` locks to deduct inventory safely.
  Do not prescribe specific database schemas or API function signatures; focus on achieving the end-to-end resilient user journey.

  ### Priority & Scope
  - **Priority:** P0 (Critical for bridging online and physical operations)
  - **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []