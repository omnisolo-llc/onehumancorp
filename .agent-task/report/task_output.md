issue_title: "Architect & Implement Offline-Tolerant Tap-to-Pay POS System for OHC"
issue_description: |
  # Research Report: Universal Tap-to-Pay & Offline-Tolerant POS Architecture

  ## 1. Problem Statement
  Retail and food cart owners (e.g., Priya the Boutique Operator, Fatima the Food Cart Operator) rely on physical in-person sales just as much as online orders. They face a massive friction point: existing solutions either force them to use a separate POS system (which fragments inventory and customer data) or rely on unstable cloud-only POS systems that break when mobile data networks are slow. They need a unified system where online sales and physical "Tap-to-Pay" sales deplete the same inventory and feed the same AI assistants, with the resilience to process transactions offline or on poor network connections and sync later.

  ## 2. Research Report
  - **Market Context**: Square dominates this space primarily because of their reliable offline-mode and seamless hardware integration. Shopify POS has made strides but often requires significant add-on fees for offline functionality and advanced inventory routing.
  - **The OHC Opportunity**: By integrating Stripe Terminal SDK directly into the OHC Flutter mobile application, we can transform any owner's smartphone into a Tap-to-Pay POS device without requiring external hardware. By combining this with an offline-first SQLite synchronization layer, OHC provides a highly resilient unified commerce experience out-of-the-box.
  - **Competitor Gaps**:
    - *Square*: Excellent POS, but limited agentic/AI automation for operations and customer follow-up.
    - *Shopify POS*: Powerful but expensive and complex for a solopreneur.
    - *Stripe*: Provides the API, but owners need an integrated UI and product system to use it.

  ## 3. Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD;
      MobileClient[OHC Flutter App 375px] --> LocalDB[(SQLite Local State)];
      MobileClient --> StripeTerminal[Stripe Terminal SDK Tap-to-Pay];
      LocalDB --> SyncEngine[Sync Engine Background Queue];
      SyncEngine --> APIGateway[OHC API / gRPC Gateway];
      APIGateway --> CoreDB[(PostgreSQL Tenant DB)];
      APIGateway --> FinanceAgent[Finance & Ops AI Agent];
      StripeTerminal --> Stripe[Stripe Processing API];
  ```

  ### Data Model (PostgreSQL & SQLite Sync)
  - `POSSession`: Represents an active register/shift, tracking start/end cash and offline state.
  - `Transaction`: The core payment record, with a `sync_status` (pending, synced, failed) and an `idempotency_key`.
  - `OfflineOrder`: Captures the cart contents when network is down.
  - `TerminalReader`: Maps a Stripe Terminal physical/virtual reader to a specific tenant/location.

  ### Mobile UX Flow (375px)
  1. **Cart / Catalog View**: Fast, touch-optimized (44x44px targets) grid of products with variants. High-contrast price tags.
  2. **Checkout Action**: Large "Charge \$XX.XX" sticky button at the bottom.
  3. **Tap-to-Pay Screen**: The native OS Tap-to-Pay interface takes over.
  4. **Offline Mode Banner**: If the network drops, a subtle top banner indicates "Offline Mode - Payments Queued." The checkout flow continues seamlessly.
  5. **Sync Recovery**: Upon network restoration, a background process flushes pending `OfflineOrder` and `Transaction` records to the API with idempotent retry logic.

  ### AI Agent Integration Points
  - **Finance & Decision Assistant**: Analyzes POS daily batch data to generate a plain-language summary for the owner ("You had a 15% increase in foot traffic today, mostly buying the new summer dresses").
  - **Operations Assistant**: Triggers automatic low-inventory alerts when physical sales deplete stock below thresholds, drafting supplier reorder emails.

  ## 4. Implementation Prompt
  **Feature Name**: Offline-Tolerant Mobile POS & Tap-to-Pay
  **Target Persona**: Priya (Boutique Operator) and Fatima (Food Cart Operator)
  **Outcome**: Priya can ring up in-store customers using just her iPhone, accept contactless payments, and have the inventory automatically deduct from the same pool as her online store. If her WiFi drops, she can continue to queue sales.

  **Acceptance Criteria**:
  1. **Data Model**: Implement `Transaction`, `POSSession`, and `OfflineOrder` schemas with tenant isolation in Postgres.
  2. **Flutter Integration**: Integrate the Stripe Terminal SDK for iOS/Android in the Flutter frontend to enable native Tap-to-Pay.
  3. **Offline Sync Engine**: Implement a robust SQLite-backed local queue in the Flutter app that persists pending orders and idempotently synchronizes them with the backend when the network connection is restored.
  4. **Agent Feed**: Expose POS closing summaries to the KAIROS AI event feed so the Finance agent can generate daily summaries.
  5. **UX Constraints**: The checkout flow must remain on one screen, fully functional at 375px, without horizontal scrolling, and touch targets > 44px.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
