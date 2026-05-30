issue_title: "[Architecture] Hardware-Free Offline-First Mobile POS and Tap-to-Pay Integration"
issue_description: |
  # Problem Statement
  Small business owners like Fatima (Food Cart operator) and Priya (Boutique owner) operate in person and often in environments with intermittent cellular service or high-density areas where networks get congested. Currently, OHC lacks a native, seamless in-person POS capability that connects directly to the unified OHC mobile app. The "Setup Complexity" of legacy hardware POS solutions creates significant friction and fails the "grandmother test." We need an integrated, zero-hardware solution utilizing native mobile NFC (Tap-to-Pay on iPhone/Android) to unify their online and offline sales, inventory, and ledger in real-time, that works seamlessly even in offline environments.

  # Research Report
  Traditional solutions require external bluetooth or physical plug-in card readers.
  - **Shopify:** Requires external POS hardware or specific Tap-to-Pay apps that are often separate from the primary management app, causing fragmentation.
  - **Square:** Known for their hardware, but shifts to Tap-to-Pay require their specific ecosystem which may lock users out of a unified platform.
  - **Wix/Squarespace:** Rely heavily on third-party hardware integrations causing setup complexity.
  - **Stripe Terminal:** Offers Tap-to-Pay SDKs that allow merchants to accept payments directly on their mobile devices using NFC without extra hardware. This is the ideal technology enabler for OHC.

  By directly embedding Tap-to-Pay via Stripe Terminal SDKs into the primary OHC app, we can completely bypass external hardware. This positions OHC in the "Leapfrog Zone" (High Autonomy, Radical Simplicity), allowing a merchant to open the app, enter an amount, and instantly have a customer tap their card on the merchant's phone.

  # Design Doc

  ## Key Design Decisions
  - **Zero-Hardware Approach:** Fully leverage Apple Tap to Pay on iPhone and Android native NFC Tap-to-Pay. No bluetooth readers.
  - **Unified Ledger & Inventory:** In-person transactions must directly mutate the same core Ledger and Inventory entities as online sales to prevent double-selling.
  - **Offline-First CRDTs / Queueing:** We must use a local-first database (like SQLite with CRDTs) so the app never blocks on network requests. If the network is unavailable, transactions are queued locally securely and synced when connectivity is restored, ensuring no lost sales in spotty environments.
  - **Zero Trust Security:** Enforce strict multi-tenant isolation at the terminal session level using SPIFFE/SPIRE-backed identities to guarantee one merchant cannot access another's transactions.

  ## Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      MERCHANT ||--o{ TERMINAL_SESSION : initiates
      TERMINAL_SESSION ||--|| TRANSACTION : processes
      TRANSACTION }|--|| LEDGER : records
      TRANSACTION }|--o{ INVENTORY : decrements

      MERCHANT {
          string id PK
          string tenant_id
      }
      TERMINAL_SESSION {
          string session_id PK
          string device_id
          string status
      }
      TRANSACTION {
          string tx_id PK
          float amount
          string status
          boolean is_offline_queued
      }
      LEDGER {
          string entry_id PK
          float balance
      }
      INVENTORY {
          string product_id PK
          int stock_level
      }
  ```

  ```mermaid
  sequenceDiagram
      actor Merchant (Priya)
      participant App as OHC Mobile App
      participant Terminal as NFC/Tap-to-Pay SDK
      participant Queue as Local Offline DB (CRDT)
      participant Sync as Background Sync Engine
      participant OpsAgent as Operations Agent
      participant CoreAPI as OHC Core API
      participant PaymentGW as Stripe/Payment Gateway

      Merchant->>App: Adds items to cart, taps "Charge"
      App->>Terminal: Initialize Tap-to-Pay Session
      Terminal-->>Merchant: Display "Present Card" UI
      actor Customer
      Customer->>Terminal: Taps physical card / Apple Pay
      Terminal->>App: Encrypted Payment Token
      App->>Queue: Save to Local DB (Offline First)
      Queue-->>App: Saved successfully
      App-->>Merchant: Display Success & Play Chime (Instant)

      loop Background Process
        Sync->>Queue: Check for pending transactions
        alt Internet Connected
            Sync->>CoreAPI: Batch upload transactions
            CoreAPI->>PaymentGW: Authorize & Capture
            PaymentGW-->>CoreAPI: Success Confirmation
            CoreAPI->>OpsAgent: Trigger Event: Sale Completed
            OpsAgent->>CoreAPI: Decrement Inventory & Update Ledger
            CoreAPI-->>Sync: Ack sync complete
            Sync->>Queue: Mark transaction as synced
        end
      end
  ```

  ## Mobile UX Flow (375px First)
  Every screen follows the macOS-style Translucent Glass materials combined with clean Ubiquiti UniFi modular dashboard card layouts.
  1. **Checkout Screen (Online/Offline):** Seamlessly adapts with offline indicators. The "Tap to Pay" button prominent. The Wi-Fi icon turns gray/amber with a subtle "Offline Mode" badge if offline.
  2. **Payment Processing:** A modal overlays with the OHC Glassmorphism design (blur backdrop). "Hold card near phone".
  3. **Success State:** Checkmark animation. "Payment Saved! Will process when reconnected." (If offline) or "Payment Complete" (If online). Immediate option to send a digital receipt (SMS/Email).
  4. **Queue Dashboard:** A new card appears on the main dashboard showing pending offline syncs (e.g., "3 Payments Pending Sync").

  ## AI Agent Integration Points
  - **Finance Agent:** Monitors the background sync queue upon reconnection. Handles potential declines gracefully. If a stored offline payment is declined by the gateway upon sync, the Finance Agent automatically drafts a polite SMS/Email to the customer requesting alternative payment.
  - **Operations Agent:** Temporarily locks high-value inventory items locally to prevent double-booking, releasing them upon cloud sync. Intercepts the "Sale Completed" event to automatically decrement inventory. If inventory drops below a threshold, silently queues a reorder task.

  # Implementation Prompt
  Implement the hardware-free Offline-First Tap-to-Pay POS module for the OHC mobile application.
  1. Build a local queuing mechanism (e.g., IndexedDB/SQLite using CRDTs) for offline transaction intents with strict multi-tenant isolation.
  2. Implement an event-driven background sync manager to flush the queue when network is restored.
  3. Use idempotency keys for all synced transactions to prevent double charging.
  4. Create UI fallback states (amber offline indicators, pending sync badges) following OHC Glassmorphism standards (375px responsive).
  5. Build a simulated backend endpoint to receive batch offline transaction syncs that will eventually hit Stripe.
  6. The app state (inventory, orders) must be readable and writable when offline, syncing automatically upon network restoration.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
