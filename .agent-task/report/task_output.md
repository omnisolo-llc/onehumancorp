issue_title: "Offline-Tolerant Mobile Tap-to-Pay & Edge Ledger Sync"
issue_description: |
  # Offline-Tolerant Mobile Tap-to-Pay & Edge Ledger Sync

  ## Problem Statement
  Small business owners (like Priya the boutique owner or Fatima the food cart operator) rely on their mobile devices for taking payments in-person. However, mobile networks can be flaky, especially at outdoor events, basements, or crowded areas. When a payment gateway fails or times out due to network issues, it causes a bottleneck, frustrated customers, and lost sales. Existing platforms (Shopify POS, Square) handle this with varying degrees of offline mode, but OHC currently lacks a resilient, offline-first tap-to-pay architecture backed by a local edge ledger that securely queues transactions and syncs them when the network returns, preventing double-charging and ensuring inventory consistency.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Square:** The gold standard for offline payments. Allows merchants to take offline card payments, which are stored locally and processed when connectivity is restored (merchants accept the risk of declined cards).
  - **Shopify POS:** Supports offline mode, but requires a complex setup and specific hardware.
  - **Stripe Terminal:** Offers an offline mode feature for their SDKs, allowing collection of card details and processing later.
  - **OHC Opportunity:** Implement an edge ledger in the Flutter mobile client that acts as a secure, local transaction queue. Integrate Stripe Terminal SDK's offline capabilities. When a tap-to-pay transaction occurs offline, the local ledger records it, updates local inventory, and uses a background sync agent to reconcile with the OHC PostgreSQL central ledger once the network is available.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Flutter Mobile Client - 375px] -->|Tap-to-Pay| B(Stripe Terminal SDK)
      B -->|Offline Storage| C[Mobile Edge Ledger - SQLite]
      C -->|Local Inventory Deduct| D[Local Cache]
      A -.->|Network Restored| E{Background Sync Agent}
      E -->|Read| C
      E -->|Process & Reconcile| F[OHC Central API Gateway]
      F --> G[OHC PostgreSQL Central Ledger]
      F --> H[Operations Agent]
      H -->|Global Inventory Update| I[Redis Cache]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Checkout Screen:** Standard tap-to-pay interface. If network is down, display a non-intrusive banner: "Offline Mode Active - Payments will sync later".
  - **Payment Processing:** Fast UI response. The "Payment Successful" screen shows a small icon indicating it's saved locally.
  - **Pending Queue Screen:** A dedicated, hidden-by-default screen showing pending offline transactions, allowing the owner to manually force sync or view the status.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors the background sync. Once a batch of offline transactions is synced, it updates the global inventory and notifies the owner if any items fall below threshold or if there are sync anomalies (e.g., declined offline card).
  - **Finance Agent:** Reconciles the batch with daily summaries and highlights the deferred revenue.

  ## Implementation Prompt
  **Goal:** Implement the mobile edge ledger and background sync protocol for offline-tolerant tap-to-pay.
  **CUJ:** Priya takes an in-person payment at a crowded market with no cell service. The app processes the payment using Stripe Terminal offline mode, records it in the local edge ledger, and instantly updates her local UI. Later, when she gets Wi-Fi, the app automatically syncs the transaction to the backend without double-charging.
  **Acceptance Criteria:**
  1. Define the Edge Ledger data models in the backend to receive offline sync batches.
  2. Create the API endpoints for batch transaction sync and reconciliation.
  3. Ensure idempotent processing using transaction IDs generated at the edge.
  4. Write E2E Playwright/browser tests simulating an offline transaction sync flow.
  5. Do NOT implement the actual Flutter UI yet, just the backend synchronization and queueing mechanism.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
