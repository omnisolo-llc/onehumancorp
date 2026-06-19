issue_title: "Research: Offline-First Point-of-Sale & Tap-to-Pay Terminal System"
issue_description: |
  # Offline-First Point-of-Sale & Tap-to-Pay Terminal System

  ## Problem Statement
  Small business owners need reliable payment processing regardless of internet connectivity. Maya (baker) often sells at farmer's markets with spotty cell reception. Carlos (handyman) takes payments in clients' basements where Wi-Fi doesn't reach. Fatima (food cart) operates in crowded festival environments where cellular networks are overloaded. Currently, OneHumanCorp (OHC) relies entirely on cloud connectivity for payment processing and inventory syncing. If the network drops, businesses halt. OHC lacks an edge-caching, local-first synchronization architecture to enable uninterrupted Tap-to-Pay and Point-of-Sale (POS) operations.

  ## Research Report
  We investigated the underlying mobile POS architectures from major competitors to understand how they achieve high availability at the edge. The findings show a clear industry shift towards local-first databases with background synchronization queues.

  ### Competitive Analysis
  | Platform | Offline Mode Capable? | Underlying Tech | Key Constraint |
  |---|---|---|---|
  | Square | Yes (Up to 24h) | Local encrypted SQLite + Sync Queue | Assumes risk for declined cards later |
  | Shopify POS | Yes (Partial) | React Native + local state caching | Needs internet to apply some discounts/sync inventory |
  | Stripe Terminal | Yes (Forwarding) | Stripe Terminal SDK | Specific hardware required (BBPOS/Stripe Reader) |
  | Wix POS | Limited | Web-view wrapper | Highly dependent on constant connectivity |
  | **OHC (Target)** | **Yes (Continuous)** | **Local-First Edge DB + Conflict-Free Sync** | **Must remain zero-config for user** |

  **Product Use Evidence:** When operating the OHC UI with Network Throttling set to Offline, attempting to process a transaction fails entirely, presenting an endless loading spinner or network error. A real owner would lose the sale in this state. The required fix is to locally cache the transaction and intelligently sync when the connection is restored.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph "Mobile Device (e.g., Carlos's Phone)"
          UI[OHC Flutter App UI - 375px]
          POS[Offline POS Engine]
          EdgeDB[Encrypted Local DB / IndexedDB]
          TapSDK[Tap-to-Pay / Terminal SDK]
          SyncQueue[Background Sync Worker]
          UI --> POS
          POS --> TapSDK
          POS --> EdgeDB
          EdgeDB --> SyncQueue
      end

      subgraph "OHC Cloud Backend"
          Gateway[API Gateway]
          Ledger[OHC Universal Ledger]
          Inventory[Global Inventory Service]
          Gateway --> Ledger
          Gateway --> Inventory
      end

      SyncQueue -- "Auto-syncs when online" --> Gateway
  ```

  ### Mobile UX Flow
  - Normal Flow: The UI looks the same.
  - Offline Flow: When connectivity drops, the app seamlessly switches to a local mode. A small translucent pill indicator shows "Offline Mode: Transactions will sync when connected."
  - The merchant completes the transaction normally. The UI immediately reflects a "Payment Accepted" state to the customer to keep the line moving.
  - Sync Flow: Once connectivity is restored, the `SyncQueue` uploads stored transactions in the background. If a card is declined post-sync, an actionable task is added to the Work Triage feed.

  ### AI Agent Integration
  - **Work Triage / Ops Agent:** If an offline transaction is later declined by the processor, the agent creates a high-priority task for the owner, drafting a follow-up message to the customer with an alternative payment link.

  ## Implementation Prompt
  **User Facing Outcome:** The POS interface must allow merchants to ring up items and process (or queue) card payments even when the device is completely offline. The UI must smoothly transition to offline mode and sync data when the network returns without any manual intervention.
  **CUJ (Critical User Journey):**
  1. Owner logs into OHC app.
  2. Device loses internet connection (e.g., enters airplane mode).
  3. Owner adds an item to the cart and proceeds to checkout.
  4. Owner taps/swipes card.
  5. App immediately confirms "Payment Accepted" and queues the transaction.
  6. Device regains internet connection.
  7. App background-syncs the queued transaction to the OHC backend, updating the ledger and inventory.
  **Acceptance Criteria:**
  - Implement a secure local database (Edge DB) to store product cache and offline transactions.
  - Implement a background synchronization worker that automatically pushes pending transactions when online.
  - Full E2E Playwright test simulating offline transaction processing, network restoration, and correct ledger/inventory updates in the backend.

  ## Priority
  P0 (Critical)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
