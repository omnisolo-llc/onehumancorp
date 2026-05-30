```yaml
issue_title: "[Architecture] Offline-First Tap-to-Pay POS Implementation"
issue_description: |
  ## Problem Statement

  Small business owners need reliable payment processing regardless of internet connectivity. Maya (baker) often sells at farmer's markets with spotty cell reception. Carlos (handyman) takes payments in clients' basements where Wi-Fi doesn't reach. Fatima (food cart) operates in crowded festival environments where cellular networks are overloaded. Currently, OneHumanCorp (OHC) relies entirely on cloud connectivity for payment processing and inventory syncing. If the network drops, businesses halt. Competitors like Square and Shopify have robust "Offline Mode" capabilities, allowing merchants to swipe or tap cards, queue the transactions locally, and sync them automatically when connectivity is restored. OHC lacks an edge-caching, local-first synchronization architecture to enable uninterrupted Tap-to-Pay and Point-of-Sale (POS) operations.

  ## Research Report

  We investigated the underlying mobile POS architectures from major competitors to understand how they achieve high availability at the edge. The findings show a clear industry shift towards local-first databases with background synchronization queues.

  ### Competitive Analysis

  | Platform | Offline Mode Capable? | Underlying Tech | Key Constraint |
  |---|---|---|---|
  | Square | Yes (Up to 24h) | Local encrypted SQLite + Sync Queue | Assumes risk for declined cards later |
  | Shopify POS | Yes (Partial) | React Native + local state caching | Needs internet to apply some discounts/sync inventory |
  | Stripe Terminal | Yes (Forwarding) | Stripe Terminal SDK | Specific hardware required (BBPOS/Stripe Reader) |
  | Wix POS | Limited | Web-view wrapper | Highly dependent on constant connectivity |
  | **OHC (Target)** | **Yes (Continuous)** | **Local-First Edge DB (e.g., IndexedDB/SQLite) + Conflict-Free Sync** | **Must remain zero-config for user** |

  ### Persona Pain Points

  *   **Maya:** "I lost a $150 custom cake order at the Sunday market because my phone couldn't connect to 5G to process the tap payment."

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  sequenceDiagram
      participant User as OHC Mobile App (Edge)
      participant SDK as Payment SDK / Local DB
      participant Sync as Background Sync Engine
      participant Cloud as OHC Cloud Platform
      participant AI as Finance Agent

      User->>SDK: Process Tap-to-Pay (Offline)
      SDK-->>User: Success (Payment Queued)
      Note over SDK: Encrypted transaction stored locally

      loop Background Process
          Sync->>SDK: Check for connectivity
          alt Internet Restored
              SDK->>Cloud: Batch upload queued transactions
              Cloud-->>SDK: Ack sync
              Cloud->>AI: Trigger reconciliation
              AI-->>Cloud: Process potential declines
              Cloud->>User: Push notification (Sync Complete)
          end
      end
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)

  1.  **Checkout Screen (Online):** Standard cart interface. "Tap to Pay" button prominent. Green Wi-Fi icon indicates active connection.
  2.  **Checkout Screen (Offline):** The interface seamlessly adapts. The Wi-Fi icon turns gray/amber with a subtle "Offline Mode" badge. The "Tap to Pay" button remains active.
  3.  **Payment Processing:** A modal overlays with the OHC Glassmorphism design (blur backdrop). "Hold card near phone".
  4.  **Success State:** Checkmark animation. "Payment Saved! Will process when reconnected."
  5.  **Queue Dashboard:** A new card appears on the main dashboard (hidden behind Advanced Settings if empty, prominent if items exist) showing "3 Payments Pending Sync".

  ### AI Agent Integration

  *   **Finance Agent:** Monitors the background sync queue upon reconnection. If a stored offline payment is declined by the gateway upon sync, the Finance Agent automatically drafts a polite SMS/Email to the customer (using contact info stored at checkout or via loyalty profile) requesting alternative payment, removing the manual reconciliation burden from the business owner.
  *   **Operations Agent:** Temporarily locks high-value inventory items locally to prevent double-booking, releasing them or permanently reserving them upon cloud sync.

  ### Data Model & Invariants

  *   **OfflineTransaction:** A secure, local entity containing the encrypted payload, amount, timestamp, and a unique `idempotency_key`.
  *   **Tenant Isolation:** All local databases MUST be scoped to the authenticated tenant. On logout, the local cache is strictly wiped or encrypted using a key derived from the user's session.

  ## Implementation Prompt

  **Task for Implementer:** Build the foundational Offline-First Background Sync Queue for the OHC POS module.

  **User Journey (CUJ):**
  1. The user (business owner) is logged into the OHC mobile app.
  2. The user loses internet connectivity (simulated offline mode).
  3. The user initiates a Tap-to-Pay transaction for a $10 item.
  4. The application saves the transaction locally with a clear "Saved for later" UI indicator.
  5. The user regains connectivity.
  6. The background sync engine detects the network and flushes the queue to the server.
  7. The server processes the payment via the payment provider.
  8. The UI updates to show the transaction as fully complete.

  **Acceptance Criteria:**
  - Implement a robust local storage mechanism (e.g., IndexedDB on web/PWA or SQLite on native) to queue transaction intents.
  - Implement an event-driven background sync manager that listens for network status changes.
  - Ensure all synced transactions utilize idempotency keys to prevent double-charging.
  - Build the UI fallback states (amber offline indicators, pending sync badges) adhering to the OHC Glassmorphism standards (375px responsive).
  - Implement a simulated backend endpoint to receive batch offline transaction syncs.
  - DO NOT prescribe exact database schemas or library choices; optimize for resilience.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
```
