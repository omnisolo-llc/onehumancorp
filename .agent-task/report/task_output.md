issue_title: "Implement Multi-Tenant Offline-Tolerant Mobile POS Sync & Tap-to-Pay Architecture"
issue_description: |
  # Architecture Brief: Offline-Tolerant Mobile POS Sync & Tap-to-Pay

  ## 1. Problem Statement
  Service-based and on-the-go small business owners (e.g., Carlos the Handyman, Fatima the Food Cart Operator) frequently operate in areas with poor or zero cellular connectivity (e.g., basements, crowded events). They need to process sales, reserve inventory, and accept payments seamlessly. Currently, traditional cloud-first POS systems block checkout if the network drops, leading to lost sales and frustrated customers. OHC needs an architecture that allows for offline-tolerant tap-to-pay and inventory management that effortlessly syncs back to the centralized multi-tenant PostgreSQL ledger once connectivity is restored.

  ## 2. Research Report
  - **Market Context:** Square dominates the offline payment space with local store-and-forward mechanisms. Shopify POS requires constant connectivity for real-time inventory checks, failing gracefully only for basic cash sales. Stripe Terminal provides SDKs for offline transaction caching, but requires robust client-side state management.
  - **The OHC Opportunity:** By leveraging the local-first mobile client (Flutter) and SQLite on the device, OHC can create an invisible synchronization layer. AI agents will handle reconciliation anomalies (e.g., stock-outs during offline periods) without requiring the business owner to manually resolve database conflicts.
  - **Competitor Gaps:** Most competitors force the user to "switch to offline mode." OHC should make this transition entirely automatic and invisible to the user.

  ## 3. Design Doc
  ### Data Model & Sync Protocol
  - **Local State (Flutter/SQLite):** Mobile devices will run a local SQLite database that stores a subset of the master catalog (cached via edge nodes).
  - **Transaction Ledger:** Transactions created offline are signed locally and queued in a robust background syncer.
  - **Conflict Resolution (PostgreSQL/Go):** When the connection is restored, the Go API server ingests the queued transactions. If an inventory conflict occurs (e.g., item sold out online while sold offline), the Operations Agent is triggered to alert the owner and propose a refund or backorder.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Flutter Mobile POS] -->|Offline Read/Write| B[(Local SQLite Cache)]
      A -->|Background Sync when Online| C[Go Sync API Layer]
      C -->|Distributed Lock| D[Redis Redlock]
      C -->|Commit Ledger| E[(Multi-Tenant PostgreSQL)]
      E -->|Triggers| F[AI Agent Orchestration]
      F -->|Conflict Detection| G[Operations Agent]
  ```

  ### Mobile UX Flow
  1. The user adds items to the cart and taps "Charge".
  2. If the network is unavailable, the UI shows a subtle "Saved Offline" indicator but completes the transaction immediately.
  3. The Stripe Terminal SDK securely caches the payment intent if configured for offline processing, or caches the cash transaction.
  4. Upon network restoration, a background process syncs the transactions.

  ## 4. Implementation Prompt
  **Role:** Implementer Agent
  **Task:** Build the offline-tolerant transaction queue and synchronization API endpoints.
  **Acceptance Criteria:**
  - Create the necessary API endpoints in the Go backend to sync offline transactions.
  - The synchronization process must ingest an array of offline transactions, validate the payloads, and acquire Redis locks to prevent race conditions.
  - Apply multi-tenant isolation using `tenant_id`.
  - Trigger the Operations Agent if inventory drops below zero.
  - Ensure 100% unit test coverage for the sync logic and create at least one Playwright E2E test verifying the offline-to-online sync flow using the browser.
  - Ensure zero mock data in the UI; empty states must reflect true database state.

  ## 5. Priority & Scope
  - **Priority:** P1
  - **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
