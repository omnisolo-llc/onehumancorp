issue_title: "Instant Localized Offline-First Point of Sale (POS) Architecture"
issue_description: |
  # Research Report: Instant Localized Offline-First POS Architecture

  ## Problem Statement
  Owners and operators like Priya (boutique operator), Carlos (field service owner), and Fatima (food cart operator) frequently operate in environments with poor, flaky, or non-existent internet connectivity. They need to process transactions, accept payments, and update inventory on mobile devices without friction. A loss of connectivity currently means lost sales or manual tracking, which breaks the promise of a reliable, unified work assistant. The system lacks a robust, offline-first Point of Sale (POS) architecture capable of seamlessly caching localized catalogs, queuing transactions, and synchronizing state once connectivity is restored.

  ## Research Findings
  - **Competitor Systems Audit:** Platforms like Square and Shopify POS utilize sophisticated offline modes. They use local device storage to cache the catalog and queue offline payments.
  - **Current OHC Codebase Audit:** OHC currently lacks an edge-cached dynamic storefront and a unified offline transaction queue. While tenant isolation exists, the mobile client needs a structured local caching and queuing mechanism tightly coupled with the backend Go/gRPC APIs.
  - **Market Gap:** Many POS systems feel like separate, complex applications. OHC has the opportunity to integrate offline POS seamlessly into the AI Assistant interface, where the "Operations Assistant" automatically handles reconciliation when the device comes online.

  ## Design Doc
  **Architecture Diagram**
  ```mermaid
  sequenceDiagram
      participant App as Flutter Mobile App (Offline)
      participant LocalDB as Local Device Storage (Queue)
      participant Backend as Go Backend API
      participant Database as PostgreSQL (Tenant Data)

      App->>LocalDB: Fetch Cached Product Catalog
      App->>LocalDB: Record Payment Transaction locally
      LocalDB-->>App: Acknowledge queued state
      Note over App,LocalDB: Device comes back online
      App->>Backend: Push queued sync transactions (Idempotency Key)
      Backend->>Database: Validate & Update Inventory/Ledger
      Database-->>Backend: OK
      Backend-->>App: Sync Complete
  ```

  **Mobile UX Flow**
  1. App detects network loss -> shows translucent pill "Offline - Cash & Saved Cards Only".
  2. Owner taps items -> Cart reads from local storage instantly.
  3. Owner hits "Charge" -> Payment recorded locally.
  4. Network returns -> Background sync -> Pill changes to sync icon then disappears.

  **AI Agent Integration Points**
  - **Operations Assistant:** Automatically drafts an apology/refund if the system detects an inventory oversell during offline sync.
  - **Finance Assistant:** Flags delayed offline payments and updates the daily summary report.

  ## Implementation Prompt
  Implement the Offline-First POS synchronization protocol. On the frontend (Flutter + PWA), introduce a local device storage-backed transaction queue that intercepts checkout events when offline. On the Go backend, implement a sync endpoint to process batched, idempotency-keyed transactions, enforce row-level tenant security, and update inventory. Ensure the backend sync engine triggers an Operations Assistant event if an inventory conflict (oversell) is detected. All logic must achieve 100% test coverage, and the mobile client must maintain sub-second interaction times while offline.

  ## Priority
  P0

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
