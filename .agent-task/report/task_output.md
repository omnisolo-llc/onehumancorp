issue_title: "Hardware-Free Tap-to-Pay POS: Embedded Mobile POS & Offline-First Replay Ledger Architecture"
issue_description: |
  # Platform Architecture: Hardware-Free Tap-to-Pay POS & Offline-First Replay Ledger

  ## 1. Problem Statement & Business Opportunity
  Priya (boutique owner) and Fatima (food cart operator) need to accept in-person card payments directly on their mobile phones without purchasing, pairing, or maintaining physical card readers.

  Legacy point-of-sale platforms (e.g., Square, Shopify POS) mandate proprietary bluetooth hardware (like Square Readers or Shopify Tap & Chip readers) or segregate Tap-to-Pay into detached companion apps. When connectivity drops in basement shops or pop-up street markets, these applications either fail entirely or expose merchants to high chargeback liabilities by blindly accepting card details offline.

  Furthermore, small business owners are trapped in "subscription hell," spending upwards of $30-$100/month on separate subscriptions for payment gateways, inventory sync tools, and accounting software.

  **The OHC Advantage:** OHC will leapfrog this entire hardware layer. By embedding native mobile NFC capabilities (Apple Tap to Pay on iPhone and Google Tap to Pay on Android) directly into the primary OHC companion shell, merchants can accept credit cards instantly. All transactions flow through an **Offline-First Replay Ledger** that preserves absolute double-entry financial integrity under severe cell network congestion, automatically resolving inventory and ledger conflicts via localized CRDT models.

  ---

  ## 2. Research Report & Competitive Landscape
  A comprehensive audit of general and AI-native commerce systems reveals a major market gap:

  *   **Shopify POS:** Offers extensive multi-location inventory, but requires external hardware readers for standard merchant transaction rates. If merchants use mobile Tap-to-Pay, they are forced to configure separate store profiles, causing fragmented sales histories.
  *   **Square POS:** Heavily centered around proprietary hardware dongles. Their native offline mode relies on store-and-forwarding unencrypted card numbers, shifting 100% of chargeback and fraud liability to the merchant if the card is subsequently declined.
  *   **Durable AI / Wix:** Light-weight website generation, but highly fragile backend operations. In-person sales are treated as manual entry notes with zero real-time inventory decrementing or ledger integrations.
  *   **OHC Innovation (Leapfrog Model):** Completely hardware-free. We integrate the Stripe Terminal SDK for Software-based Tap-to-Pay (NFC) directly into the OHC Flutter shell. To resolve the offline reliability issue, OHC uses a local SQLite (SIPDB) append-only transaction event log. When offline, OHC validates local transaction rules (e.g., matching available inventory lock tokens), records cash or deferred card captures, and automatically replays them with cryptographic tenant isolation upon reconnection.

  ---

  ## 3. High-Level Architectural Design (Zero-Trust & Multi-Tenancy)
  The core design centers around an **NFC-enabled POS Session Manager** communicating with a decentralized SQLite DB on the client and a multi-tenant Rust API service on the backend.

  ### 3.1. Systems Diagram (Mermaid.js)
  ```mermaid
  graph TD;
      subgraph Mobile Companion Shell (375px)
          UI[Interactive Keypad POS] -->|NFC Read| AppleGoogleTap[Apple/Google Tap to Pay SDK]
          AppleGoogleTap -->|Transaction Token| LocalQueue[Offline Transaction Queue - SQLite]
          LocalQueue -->|CRDT Deductions| LocalLedger[(Local SIPDB SQLite)]
      end

      subgraph OHC Multi-Tenant Cloud Native API (Rust)
          LocalQueue -->|Reconnection Replay Sync| Gateway[Zero-Trust Gateway / SPIFFE SPIRE]
          Gateway -->|Tenant-Isolated Org Claims| TerminalService[Terminal Session service]
          TerminalService -->|Idempotent Event Log| PG[(PostgreSQL Unified Ledger)]
          PG -->|Trigger Sync| InvService[Inventory Ledger]
      end

      subgraph AI Department Coordination
          PG -->|Sync Event| OperationsAgent[Operations AI Agent]
          OperationsAgent -->|Low Stock Alert| MarketingAgent[Marketing AI Agent]
          TerminalService -->|Reconcile Anomalies| FinanceAgent[The Accountant Agent]
      end
  ```

  ### 3.2. Data Model & Integrity Invariants (Mermaid.js ER)
  ```mermaid
  erDiagram
      ORGANIZATION ||--o{ TERMINAL_SESSION : registers
      TERMINAL_SESSION ||--o{ TRANSACTION_QUEUE : buffers
      TRANSACTION_QUEUE ||--o{ LEDGER_ENTRY : produces
      LEDGER_ENTRY ||--|| INVENTORY_LEDGER : decrements

      ORGANIZATION {
          uuid tenant_id PK
          string company_name
          string base_currency
      }
      TERMINAL_SESSION {
          uuid session_id PK
          uuid tenant_id FK
          string device_signature "cryptographically signed by SPIRE"
          string status "active/offline/closed"
          timestamp created_at
      }
      TRANSACTION_QUEUE {
          uuid tx_id PK
          uuid session_id FK
          decimal total_amount
          string currency
          string payment_method "tap_to_pay/cash/qr_code"
          string status "queued/pending_sync/completed"
          string payload_hash "cryptographic hash for replay validation"
          timestamp created_at
      }
      LEDGER_ENTRY {
          uuid entry_id PK
          uuid tx_id FK
          uuid tenant_id FK
          string account_type "debit/credit"
          decimal amount
          timestamp posted_at
      }
      INVENTORY_LEDGER {
          string sku PK
          uuid tenant_id FK
          int stock_count
          timestamp last_updated
      }
  ```

  ---

  ## 4. Mobile UX Flow (375px First & "Grandmother Test" Approved)
  Designed for maximum touch clarity under noisy, high-pressure field conditions (e.g., Fatima cooking or Priya in a crowded boutique).

  *   **Viewport Constraints:** Optimized for 375px width. Touch targets are scaled to a minimum of **60x60px** with active tactile haptic feedback.
  *   **Visual Standard:** Apple/Ubiquiti translucent glass dashboard cards with high-contrast indicator badges.
  *   **Status Pill Indicator:** Transparent top-bar capsule: `[🟢 Connected]` or `[🟠 Offline - 8 Syncs Queued]`.

  ### 4.1. Screen Flow (375px Layout)
  1.  **Home Command:** Single-tap primary action button: `[ Accept In-Person Payment ]` (72px height, vivid blue translucent acrylic background).
  2.  **Keypad Entry:** A full-screen numerical keypad with clean typography. Displays real-time calculations (e.g., `$15.00 + $1.25 Tax = $16.25`).
  3.  **Payment Mode selection:** Massive buttons for `[ Tap to Pay (NFC) ]` (default active state) and `[ Cash / QR ]`.
  4.  **Native Tap Overlay:** Trigger native NFC card-capture animation. Merchant simply extends their phone to the customer.
  5.  **Tactile Success State:** Large green confirmation screen with high-contrast text: `[ Receipt Sent! ]` alongside immediate SMS/Email input box.

  ---

  ## 5. Performance, Offline, & Zero-Trust Invariants
  *   **First Input Delay (FID):** < 50ms for local keypad interactions using optimistic UI states.
  *   **Network Resilience:** Transaction payloads are batched, SHA-256 hashed, and queued in local SQLite using unique transaction idempotency keys. Replay sequencing enforces FIFO rules to prevent database out-of-order anomalies.
  *   **Zero-Trust Multi-Tenancy:** PostgreSQL row-level security (RLS) is strictly enforced. Mobile sync payloads must match the active `tenant_id` claims injected by SPIFFE/SPIRE device credentials. Unauthenticated sync replays are blocked at the cloud gateway.

  ---

  ## 6. Implementation Prompt (For the Engineering Swarm)
  **Feature Name:** Hardware-Free Tap-to-Pay POS Sync & Replay Ledger

  **Outcome:** Implement the backend services and API handlers to manage hardware-free mobile terminal sessions, offline event buffering, and automatic database replay reconciliations.

  **Critical User Journey (CUJ):**
  1.  Priya (boutique owner) opens her OHC companion app (375px viewport) and starts a new point-of-sale terminal session.
  2.  Priya processes a $120.00 Tap-to-Pay transaction.
  3.  Due to poor network coverage, her phone loses internet connectivity. The UI transitions to an offline state but remains fully functional.
  4.  Priya registers a $50.00 cash transaction. The app saves the payload into the `OfflineTransactionQueue` and optimistically updates the local sales ledger and inventory dashboard.
  5.  Upon network restoration, the queue automatically triggers a background replay to the OHC sync server.
  6.  The cloud server validates transaction hashes, verifies organization-level tenant bounds (RLS), processes the queued entries, and reconciles the central inventory level without duplication.

  **Acceptance Criteria:**
  - Create a multi-tenant PostgreSQL schema mapping `TerminalSessions`, `OfflineTransactionQueue`, and double-entry `LedgerEntries` with active RLS policies.
  - Expose gRPC endpoints for `/Terminal/InitializeSession` (returning a mock Stripe Terminal SDK token) and `/Terminal/SubmitSyncBatch`.
  - The synchronization sync endpoint must process the batch sequentially, verify cryptographic hashes, enforce exact transaction idempotency, and update central inventory levels atomically.
  - Implement 100% unit test coverage for the sync processing and queue replay logic.
  - Create Playwright E2E tests simulating transition from offline to online states, asserting ledger consistency.

  ---

  ## 7. References & Sources Catalog
  1. https://stripe.com/docs/terminal/features/tap-to-pay
  2. https://stripe.com/docs/terminal/sdk/android
  3. https://stripe.com/docs/terminal/sdk/ios
  4. https://developer.apple.com/tap-to-pay/
  5. https://developer.android.com/guide/topics/connectivity/nfc
  6. https://www.shopify.com/pos
  7. https://squareup.com/us/en/hardware/contactless-chip-reader
  8. https://durable.co/features/invoicing
  9. https://github.com/spiffe/spire
  10. https://www.postgresql.org/docs/current/ddl-rowsecurity.html
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
