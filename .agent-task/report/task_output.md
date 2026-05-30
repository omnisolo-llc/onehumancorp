issue_title: "[Architecture] Offline-First Mobile POS & Tap-to-Pay Engine"
issue_description: |
  ## Problem Statement
  Small business owners like Fatima (Food Cart operator) and Priya (Boutique owner) operate in environments with intermittent cellular service or high-density areas where networks get congested. Fatima frequently takes orders and payments at street festivals where her low-end Android phone loses connection. When this happens, she can't process transactions, resulting in lost sales and frustrated customers. Priya needs to quickly accept payments in her shop without investing in expensive, dedicated POS hardware. They both need an invisible, highly resilient mobile POS that works seamlessly on their smartphones, capable of processing "Tap-to-Pay" transactions completely offline, queuing them locally, and syncing securely once connectivity is restored.

  ## Research Report
  *   **Current Architecture Limits:** OHC's current checkout flow assumes a constant internet connection. If the connection drops during payment authorization, the UI hangs or fails, forcing the user to retry, which ruins the customer experience.
  *   **Competitor Analysis:**
      *   *Square:* Requires proprietary hardware (dongles, terminals). Their offline mode is decent but locks the merchant into their expensive hardware ecosystem.
      *   *Shopify POS:* Highly reliant on continuous connectivity for inventory checks and full payment processing. It is also an expensive add-on to their core platform.
      *   *Stripe Terminal:* Excellent APIs, but requires the developer to build the offline queuing and synchronization logic.
  *   **Discovery:** OHC needs a native, offline-first mobile POS that leverages the smartphone's built-in NFC for Tap-to-Pay (Apple Tap to Pay / Android Tap to Pay). It must implement a robust local queuing system (CRDTs or local write-ahead log) to safely store transactions offline and sync them immediately upon reconnection, without any manual intervention from the user.

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

  ### AI Agent Integration Points
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
