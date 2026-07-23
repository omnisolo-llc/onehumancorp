issue_title: "Implement Universal Offline-First Tap-to-Pay POS"
issue_description: |
  ## Title
  Universal Offline-First Tap-to-Pay POS

  ## Problem Statement
  Operators like Priya (Boutique) and Fatima (Food Cart) operate in environments where Wi-Fi is spotty, yet they must seamlessly process in-person payments and keep inventory synchronized. Relying solely on third-party bulky card readers adds friction and upfront hardware costs. They need the ability to turn their existing Android or iPhone into a tap-to-pay terminal that works flawlessly offline and invisibly syncs inventory and sales when connectivity returns.

  ## Research Report
  - **Market Landscape:** Current SMB POS systems (Square, Toast) require dedicated hardware and are often tied strictly to online states. Stripe Terminal offers Tap to Pay on iPhone and Android, reducing hardware dependency, but offline resilience in typical web POS implementations is weak.
  - **Persona Evidence:** Fatima works at a food cart where cellular data frequently drops. She cannot afford to lose a customer or slow down the queue while waiting for a timeout.
  - **Gap Analysis:** Competitors like Shopify POS have offline modes but require a complex reconciliation process. A true offline-first Tap-to-Pay POS built with Flutter can leverage local SQLite/Isar for immediate state transitions (Inventory -1, Order = Pending Sync) and use a CRDT (Conflict-free Replicated Data Type) or robust background sync queue to finalize Stripe intents once the network is restored.

  ## Design Doc
  ### High-Level Architecture
  ```mermaid
  erDiagram
      FlutterPOS {
          string device_id
          string network_status
      }
      LocalSQLite {
          string transaction_id
          float amount
          string status
      }
      BackgroundSyncIsolate {
          string queue_status
      }
      GoBackend {
          string idempotency_key
          string payment_intent
      }
      StripeTerminalSDK {
          string reader_id
          string encrypted_payload
      }

      FlutterPOS ||--o{ LocalSQLite : "saves offline transactions to"
      FlutterPOS ||--|| StripeTerminalSDK : "captures card via"
      LocalSQLite ||--o{ BackgroundSyncIsolate : "consumed by"
      BackgroundSyncIsolate }o--|| GoBackend : "syncs when online to"
  ```
  - **Frontend:** Flutter mobile app (iOS/Android) leveraging Stripe Terminal SDK for Tap to Pay.
  - **Local Storage:** SQLite storing the active catalog, pricing, and an `OfflineTransactionQueue`.
  - **Sync Mechanism:** Background isolate running a retry-with-exponential-backoff loop. When online, it consumes the `OfflineTransactionQueue`, communicates with the Go backend to finalize transactions, and resolves CRDT inventory states.
  - **Backend:** Go API with idempotency keys for all payment captures. `OrderService` handles deduplication.

  ### Mobile UX Flow (375px First)
  - **Screen 1 (POS Cart):** Grid of large, high-contrast product image tiles. Tap adds to cart. Sticky footer shows Total and a large "Charge $X" button.
  - **Screen 2 (Tap to Pay):** Full-screen prompt leveraging native Apple/Google Tap to Pay UI.
  - **Screen 3 (Success/Offline Status):** Big green checkmark. If offline, a subtle "Saved offline, will sync later" indicator appears in the app header, allowing immediate transition to the next customer.

  ### AI Agent Integration
  - **Finance Assistant:** Observes the sync queue and pushes a summary when offline transactions are successfully reconciled (e.g., "3 offline orders synced. $45 added to today's revenue.").

  ## Implementation Prompt
  Implement the offline-first Tap-to-Pay POS interface in the Flutter app.
  1. Build a robust `OfflineSyncManager` that queues transactions when the device is disconnected.
  2. Implement the POS Cart screen (375px optimized) with a grid of products and a sticky "Charge" button.
  3. Integrate the Stripe Terminal SDK for Tap to Pay, ensuring it gracefully falls back to queuing the transaction intent if the network drops.
  4. Write E2E Playwright tests simulating the offline-to-online transition using mocked network conditions, proving the order is eventually consistent on the backend.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
