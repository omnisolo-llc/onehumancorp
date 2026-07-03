issue_title: "Architectural Gap: Offline-Tolerant Mobile POS & Agentic Inventory Sync"
issue_description: |
  # Research Report: Offline-Tolerant Mobile POS & Agentic Inventory Sync

  ## Problem Statement
  Currently, OHC lacks a robust, offline-tolerant point-of-sale (POS) capability with real-time tap-to-pay integration and agentic inventory synchronization.
  For personas like Priya (Boutique Operator) and Fatima (Food Cart Operator), network connections can be flaky.
  When Priya sells an item in-store using Tap-to-Pay on her iPhone, the online inventory must sync immediately to prevent online overselling. If Fatima's food cart loses 5G connectivity, she still needs to record sales and queue payments for deferred processing.
  The current architecture relies heavily on constant connectivity to the core backend, making mobile-first physical commerce fragile.

  ## Track 1: Architectural Gap & Scaling Discovery
  - **Codebase & Docs Audit:** Current documentation and repo structures emphasize online bookings and digital work triage but lack local-first event queueing for offline POS operations.
  - **Competitor Systems Audit:**
    - *Shopify POS:* Uses a local cache for product catalogs and queues transactions when offline, syncing via mutations once reconnected.
    - *Square:* Best-in-class offline mode. Transactions are encrypted and stored locally. The terminal UI degrades gracefully, allowing cash recording and deferred card processing (within risk limits).
    - *Stripe Terminal:* Provides iOS/Android SDKs that manage card reader connections and local state, but requires the host app to manage the offline-to-online ledger sync.
  - **Identify Gaps:** OHC needs a local-first Mobile POS module within the Flutter app that integrates with Tap-to-Pay on iPhone/Android, backed by a robust outbox or syncing pattern to communicate with the backend agentic workflows.

  ## Track 2: Selected Architecture Deep Dive
  - **Business Journey Mapping (Priya's Boutique):**
    1. Priya opens OHC app on iPhone (375px viewport).
    2. She taps "New Sale", scanning a barcode using the camera or selecting a variant.
    3. Network drops. The app seamlessly transitions to "Offline Mode".
    4. She selects "Cash" (or deferred card).
    5. The transaction is logged to a local Outbox.
    6. Network restores. The outbox flushes to the OHC backend.
    7. The **Operations AI Agent** detects the inventory drop, updates the online storefront cache, and if stock is low, drafts a reorder email to her supplier.

  - **Data Model & Invariants:**
    - *Local Data:* A local transaction queue that persists un-synced orders.
    - *Backend Ledger:* An immutable sales ledger and inventory event log with strict `tenant_id` isolation.
    ```mermaid
    erDiagram
      TENANT ||--o{ INVENTORY_ITEM : owns
      INVENTORY_ITEM ||--o{ INVENTORY_EVENT : tracks
      TENANT ||--o{ POS_SESSION : operates
      POS_SESSION ||--o{ POS_TRANSACTION : generates
      POS_TRANSACTION }|--|| INVENTORY_EVENT : triggers
    ```

  - **AI Department Coordination:**
    - **Finance Agent:** Validates and reconciles the synced POS transactions.
    - **Operations Agent:** Monitors inventory events. If stock < threshold, it proactively drafts restock orders or marks storefront items as "Sold Out".

  ## Track 3: Technical Integrity & Mobile-First Review
  - **Mobile-First UX Flow (375px):**
    - Large 44x44px minimal tap targets for product variants.
    - Translucent glass status bar indicating "Offline Mode (3 pending syncs)".
    - Seamless bottom sheet for "Tap to Pay".
  - **Performance & Offline Targets:**
    - Instant UI response on transaction save.
    - Background task handles exponential backoff sync to the backend.
  - **Zero Trust & Security:**
    - Local transactions are cryptographically tied to the device's session.
    - Backend API rejects any synced transaction where `tenant_id` does not match the validated identity of the syncing device.

  ## Track 4: Strategic Feature Issue Dispatch (Implementation Prompt)
  **Prompt for Implementer:**
  Implement the "Local-First POS Sync & Tap-to-Pay" foundation.
  1. Create the Flutter UI for a 375px Mobile POS checkout screen featuring large touch targets, variant selection, and a unified "Pay" button that triggers a mocked Tap-to-Pay bottom sheet.
  2. Implement a local persistent Outbox in the Flutter client to record transactions when offline.
  3. Create the backend capability to accept a batch of queued transactions, strictly enforcing `tenant_id` isolation.
  4. Ensure backend processing emits an inventory depletion event so the AI Operations Agent can subsequently pick it up.
  5. Ensure E2E Playwright tests can complete a checkout flow and verify the backend ledger receives the transaction.

  **Acceptance Criteria:**
  - A user (Priya) can ring up an order and see it saved locally instantly.
  - A backend sync successfully updates the tenant's inventory ledger.
  - UI strictly adheres to the translucent glass and clean spacing design tokens.

  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []