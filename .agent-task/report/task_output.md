issue_title: "Omnichannel Tap-to-Pay Terminal SDK & Agentic POS Architecture"
issue_description: |
  # Mission Queue Protocol: Omnichannel Tap-to-Pay Terminal SDK & Agentic POS Architecture

  ## Problem Statement
  Small business owners like Priya (boutique operator) and Fatima (food cart operator) operate heavily in-person but struggle with fragmented point-of-sale (POS) systems that don't talk to their online storefronts. Traditional solutions like Square or Shopify POS require dedicated hardware, separate app downloads, and often lead to inventory desyncs (selling an item online that just sold in-store). Furthermore, these platforms lack an AI assistant that can instantly process in-person context—like recognizing a returning VIP customer in-store and automatically applying their online loyalty perks.

  ## Research Report
  - **Market Context**: Shopify POS and Square dominate the SMB physical retail space, but they operate as distinct applications from the core e-commerce engine, often requiring complex catalog syncing. Stripe Terminal offers robust SDKs, including Tap-to-Pay on iPhone/Android, allowing any modern smartphone to become a POS without extra hardware.
  - **The OHC Opportunity**: By integrating Tap-to-Pay directly into the unified OHC mobile application and backing it with the same real-time PostgreSQL inventory ledger used for online sales, we eliminate inventory desyncs entirely.
  - **Competitor Gaps**:
    - *Shopify POS / Square*: Requires separate apps or dongles. Lacks an embedded AI assistant to draft follow-up thank-you notes or loyalty upsells immediately after an in-store tap.
    - *Stripe Terminal*: Provides the plumbing but requires heavy developer integration to connect to a unified commerce backend.

  ## Design Doc
  ### System Architecture & Data Model (PostgreSQL + Bazel)
  - **Data Models**:
    - `TerminalSession`: Represents an active POS session on a specific device (linked via SPIFFE identity for zero-trust).
    - `InPersonTransaction`: Extension of the standard Order entity, flagged with `channel=pos` and linked to a `TerminalSession`.
    - `HardwareReader`: Maps to Stripe Terminal Reader objects (for optional physical readers, though Tap-to-Pay via NFC is primary).
  - **Multi-Tenant Invariants**: Every `TerminalSession` and `InPersonTransaction` strictly enforces `tenant_id` via PostgreSQL RLS.
  - **Edge-Caching & Offline Resilience**: The POS must support local caching of the active catalog (via Redis/SQLite in the mobile client) to allow basket building while offline, with transaction queuing (storing intents locally) if the network drops, syncing via the Job Queue when reconnected.

  ### Mobile UX Flow (375px First)
  1. **Quick Mode (The "Register")**: A specialized, high-contrast, large-touch-target (44x44px min) UI mode in the OHC Flutter app.
  2. **Basket Building**: Priya scans a barcode or taps large product variant tiles (size/color).
  3. **Tap-to-Pay Flow**: Priya taps "Charge $45.00". The screen transitions to the native iOS/Android NFC Tap-to-Pay interface.
  4. **Agentic Handoff**: Upon successful payment, the `Operations Agent` instantly deducts inventory across all channels. The `Customer Assistant` drafts an email receipt with a personalized "Thanks for stopping by the shop!" message based on the customer's profile if their email is attached (via digital receipt prompt).

  ### AI Agent Integration
  - **Operations Agent**: Monitors the `InPersonTransaction` stream. If an in-store purchase drops a product's inventory below a critical threshold, it instantly invalidates the edge cache for the online storefront to prevent double-selling.
  - **Finance Agent**: Reconciles Tap-to-Pay payouts with standard online Stripe payouts, presenting a unified Daily Summary to the owner.

  ## Implementation Prompt
  **Feature Name**: Native Tap-to-Pay POS & Unified Inventory Engine
  **Target Persona**: Priya (Boutique Operator), Fatima (Food Cart Operator)
  **User-Facing Outcome**: Priya can use her existing iPhone to ring up in-store customers via Tap-to-Pay. The inventory is instantly deducted from her online store, and her AI assistant automatically sends a personalized digital receipt to returning customers.

  **Acceptance Criteria (Implementer Instructions)**:
  1. **Proto & DB**: Define `TerminalSession` and `InPersonTransaction` in gRPC protos and PostgreSQL migrations with strict RLS (`tenant_id`).
  2. **Service Layer**: Implement the POS checkout API endpoint, interacting with Stripe Terminal SDK logic (mocking hardware interactions for testing). Ensure idempotent processing.
  3. **AI Hand-off**: Configure the `msgbus.rs` to emit an `InStorePurchaseCompleted` event that triggers the Operations and Finance agents.
  4. **Frontend (Flutter/Web)**: Build a 375px-optimized POS "Quick Mode" screen with large product tiles and a prominent "Charge" button that integrates with the backend Terminal session.
  5. **Testing**: Write 100% unit tests for the backend logic and at least 5 Playwright E2E tests simulating the POS checkout flow and verifying inventory deduction.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
