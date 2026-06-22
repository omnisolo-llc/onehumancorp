issue_title: "Implement Unified Offline-Tolerant Mobile POS & Tap-to-Pay Architecture"
issue_description: |
  ## Title
  Implement Unified Offline-Tolerant Mobile POS & Tap-to-Pay Architecture

  ## Problem Statement
  Small business owners like Priya (boutique operator) and Fatima (food cart operator) often operate in environments with spotty or zero internet connectivity. When the network drops, they cannot process in-person sales, accept tap-to-pay transactions, or access their current inventory. They need a system that continues to function seamlessly offline, securely queues transactions, and automatically synchronizes when connectivity is restored, ensuring they never miss a sale or lose data due to unreliable internet.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Square:** The industry leader in mobile POS. It offers an "Offline Mode" that allows merchants to take swipe and chip payments offline, queueing them for processing when back online. However, tap-to-pay (NFC) often requires real-time authorization due to EMV constraints, though some newer terminal SDKs offer limited offline tap capabilities.
  - **Shopify POS:** Offers offline cash transactions and limited offline credit card processing (depending on the hardware/region). Requires the merchant to proactively sync when back online.
  - **Wix/Squarespace:** Weak offline POS capabilities. They rely heavily on continuous cloud connectivity for inventory and payment processing.
  - **OHC Opportunity:** OHC can differentiate by offering a robust, truly offline-first mobile architecture. By leveraging local databases on the mobile device (e.g., SQLite via Flutter/Tauri) and intelligent background sync via the Operations AI Assistant, OHC can provide uninterrupted service. The AI can manage risk (e.g., warning the owner of offline tap-to-pay limits) and handle the complexity of conflict resolution during sync.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - Flutter/Tauri] -->|Reads/Writes| B(Local SQLite DB)
      A -->|Payment Intent| C[Tap-to-Pay SDK / Terminal]
      C -->|Offline Auth| B
      B -.->|Background Sync Queue| D[Sync Engine]
      D -->|Network Restored| E[API Gateway]
      E --> F[OHC Cloud Backend]
      F -->|Webhook / Process| G[Stripe Payment Gateway]
      F -->|Update| H[Central Database / Inventory]
      H -.->|Push Updates| A
      F --> I[Operations AI Assistant]
      I -->|Sync Alerts / Risk Warnings| A
  ```

  ### UI Wireframes & Mobile UX Flow
  1.  **Home/Feed:** Top banner displays connection status (e.g., "Offline - Changes will sync later").
  2.  **Checkout (375px):**
      - Cart and item selection read instantly from local cache.
      - "Tap to Pay" button remains active.
      - If offline, a modal explains: "Accepting offline payment. Funds will process when online. Limit: $100."
  3.  **Payment Processing:** Simulated success screen with an "Offline Queued" badge.
  4.  **Sync Restoration:** When online, a subtle snackbar indicates "Syncing 3 offline transactions...", followed by "All caught up!"

  ### AI Agent Integration Points
  -   **Operations Assistant:** Monitors the sync queue. If a transaction fails to process after sync (e.g., card declined later), the agent drafts an alert and a follow-up message to the customer (if contact info is known) for the owner to approve.
  -   **Finance Assistant:** Excludes un-synced offline transactions from "Confirmed Revenue" but includes them in "Pending Offline Revenue" to maintain clear accounting.

  ### Key Design Decisions
  -   **Offline-First Data Model:** All catalog, pricing, and critical customer data must be aggressively cached locally using SQLite/Hive on the mobile device.
  -   **Event Sourcing for Sync:** Local mutations (sales, inventory deductions) are stored as events in a local outbox queue, replayed to the server when online to handle conflict resolution.
  -   **Risk Management:** Strict offline limits (e.g., transaction amount, total offline duration) configured per tenant to minimize liability for declined tap-to-pay transactions.

  ## Implementation Prompt
  **Role:** Implementer
  **Task:** Build the offline-tolerant transaction queue and Tap-to-Pay integration for the OHC mobile POS.
  **CUJ:** As Priya (boutique owner), I want to ring up a customer using Tap-to-Pay even when my store's Wi-Fi is down, so I don't lose the sale.
  **Acceptance Criteria:**
  1.  Implement a local SQLite outbox table to store pending transactions.
  2.  Integrate the platform-specific Tap-to-Pay SDK (e.g., Stripe Terminal SDK) with offline support enabled/configured.
  3.  Create a background sync worker that detects network restoration and flushes the outbox queue to the OHC backend.
  4.  Update the mobile Checkout UI to clearly display offline status and transaction queue badges on a 375px screen.
  5.  Ensure the Operations Assistant can read the sync status and generate alerts for failed background transactions.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
