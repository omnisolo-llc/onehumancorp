issue_title: "Zero-Hardware Tap-to-Pay POS with Autonomous Multi-Tenant Inventory Sync & AI Revenue Reconciliation"
issue_description: |
  # 🔬 OHC Research Report: Zero-Hardware Tap-to-Pay POS with Autonomous Multi-Tenant Inventory Sync & AI Revenue Reconciliation

  ## 1. Executive Summary & Problem Statement
  Small business owners and field operators—such as **Priya (Boutique Operator)**, **Carlos (Field Service Owner)**, and **Fatima (Food Cart Operator)**—frequently conduct business in physical environments with spotty internet connectivity. Operating legacy POS hardware systems introduces critical paint points:
  - **Hardware Friction:** External card readers require bluetooth/Wi-Fi pairing, regular battery charging, and constant firmware updates. Hardware failures directly halt business operations.
  - **Siloed Systems:** Legacy POS tools (e.g., Square) operate independently of online store-builders (Wix/Shopify), leading to disconnected inventory layers, double-sold items, and fragmented accounting.
  - **Setup Complexity:** Convoluted device pairing flows fail the "grandmother test" for non-technical owners.
  - **Security & Multi-Tenancy Gaps:** Shared local POS terminal configurations are vulnerable to cross-tenant data leaks and lack secure workload-level identity isolation (SPIFFE/SPIRE).

  ### The OHC Solution
  We propose the **Zero-Hardware Tap-to-Pay POS** system. By utilizing the built-in NFC antennas of modern smartphones alongside the **Stripe Terminal SDK (Tap to Pay on iPhone/Android)**, merchants can instantly accept in-person card and mobile wallet payments. This feature is unified natively with the OHC backend to provide real-time ledger accounting, automatic inventory synchronization, and autonomous background coordination handled by AI Agent Departments.

  ---

  ## 2. Competitive Analysis: OHC vs. Market Leaders

  | Dimension | Shopify POS | Square POS | Wix / Squarespace POS | OHC Zero-Hardware POS |
  |---|---|---|---|---|
  | **Hardware Dependency** | High (Requires Shopify Tap & Chip Readers) | High (Requires Square Reader or Terminal) | High (Requires external card reader hardware) | **Zero (Native NFC Tap-to-Pay)** |
  | **Inventory Sync** | Semi-autonomous (Manual sync overrides common) | Real-time but confined to Square Catalog | Delayed or requires third-party plugins | **Instant & Autonomous (Operations Agent)** |
  | **Accounting Integration** | Manual export to QuickBooks/Xero | Standard integrations, often lacks cash reconciliation | Relies on external bookkeeping | **Autonomous AI Ledger & Financial Advisory** |
  | **Offline Resiliency** | Standard (No card processing without internet) | Offline mode exists but risk of chargebacks is unmitigated | Poor or no offline card capture | **Secure Offline Queueing & AI Chargeback Guard** |
  | **Auth & Isolation** | Simple session pin (Multi-tenant layer on cloud) | Proprietary tokenization | Basic OAuth | **Zero-Trust SPIFFE/SPIRE Tenant Isolation** |

  ---

  ## 3. End-to-End Persona Journey Map (Acquisition to Referral)

  ### The Persona: Fatima - Halal Food Cart Operator
  Fatima runs a busy food cart in a high-foot-traffic metropolitan park where mobile cellular data is extremely flaky. She has limited English and zero technical expertise.

  ```
  [Acquisition: Low Friction]
  Fatima hears about OHC on a community forum. She registers with 1-tap via her Android phone.
  No merchant ID setup required; OHC auto-provisions a secure, isolated tenant workspace.
        │
        ▼
  [Onboarding: The Grandma Test]
  The OHC onboarding assistant greets Fatima in Arabic.
  Instead of asking for bank routing or terminal IDs, the AI prompts: "What do you sell?"
  Fatima speaks/types: "Chicken over rice ($10) and Gyro over rice ($11)."
  The AI configures her catalog, creates a Tap-to-Pay terminal session, and generates her POS screen.
        │
        ▼
  [Activation: Zero-Hardware First Sale]
  A customer walks up and orders. Fatima opens the app, taps "Accept Payment," enters $10,
  and holds out her phone. The customer taps their physical Visa card.
  Transaction succeeds. The app displays a green checkmark with haptic feedback.
        │
        ▼
  [Retention: Secure Offline Processing]
  A severe storm degrades the cellular network to 1x/Offline.
  Two more customers tap to pay. OHC captures the transaction securely in the local
  "Offline Queue" using encrypted device storage (SIPDB), validating card signatures locally.
        │
        ▼
  [Revenue: Automated Inventory & Ledger Sync]
  Network returns. OHC's sync manager replays the queue.
  The Operations Agent detects 2 "Chicken over rice" sales, decrements the food cart inventory,
  and warns Fatima: "You are running low on chicken! Only 5 servings left."
        │
        ▼
  [Referral: Plain-Language Weekly Briefing]
  At the end of the week, the Business Advisory Agent sends Fatima a voice notification:
  "Fatima, you made $1,200 this week! In-person sales during offline mode saved 3 sales ($31).
  Your busiest day was Thursday. Tap here to invite a fellow cart owner and get a free month!"
  ```

  ---

  ## 4. High-Level Architectural Design

  ### Multi-Tenant Isolation & Zero Trust
  Every terminal session and offline payment payload is cryptographically tied to the active tenant domain.
  - **Identity Proof:** The Tauri client obtains a SPIFFE-issued SVID (via SPIRE) which signs every POS API call.
  - **RLS Enforcement:** The backend implements PostgreSQL Row-Level Security (`ENABLE ROW LEVEL SECURITY`) on all POS-related tables (`terminal_sessions`, `transactions`, `ledger_entries`, `inventory_levels`). Every SQL query is automatically bound to the authenticated `tenant_id`.

  ### Architecture Diagram (Mermaid.js)

  ```mermaid
  graph TD
      subgraph Mobile Client (375px Tauri/PWA)
          UI[NFC Keypad UI] -->|Initiates Payment| SDK[Stripe Terminal Android/iOS SDK]
          UI -->|Failsafe / Offline| OfflineQueue[Encrypted Local Offline Queue]
          OfflineQueue -->|Encrypted Playloads| LocalSQLite[(SQLite SIPDB)]
      end

      subgraph OHC API Gateway (Zero-Trust)
          GW[API Gateway / Router] -->|SPIFFE/SPIRE MTLS Auth| Auth[OIDC & Identity Service]
      end

      subgraph Backend Agent Core (Multi-Tenant Hub)
          TSM[Terminal Session Manager] -->|Tenant Query Filters| DB[(PostgreSQL with RLS)]
          SyncMgr[Offline Queue Sync Manager] -->|Replay Queue| DB

          %% AI Agent Departments
          OpAgent[Operations AI Agent] -->|Listen for Transactions| Inv[Inventory Mesh]
          FinAgent[Finance AI Agent] -->|Listen for Payments| Ledg[Central Ledger]
          AdvisAgent[Advisory AI Agent] -->|Analyze Revenue Streams| Briefing[Weekly Plain-Language Briefing]
      end

      %% Flow connections
      SDK -->|NFC Read / Tokenization| GW
      LocalSQLite -->|Network Available: Replay| GW
      GW -->|gRPC/REST| TSM
      GW -->|REST Replay Batch| SyncMgr
  ```

  ### Entity-Relationship Diagram (ERD)

  ```mermaid
  erDiagram
      TENANT ||--o{ TERMINAL_SESSION : "has active"
      TENANT ||--o{ LEDGER_ENTRY : "owns"
      TENANT ||--o{ INVENTORY_LEVEL : "manages"
      TERMINAL_SESSION ||--o{ TRANSACTION : "processes"
      TRANSACTION ||--|{ LEDGER_ENTRY : "creates"
      TRANSACTION }|--o{ INVENTORY_MUTATION : "triggers"
      INVENTORY_LEVEL ||--o{ INVENTORY_MUTATION : "records"

      TENANT {
          string tenant_id PK "UUID"
          string organization_name
          string primary_currency
          string locale
      }

      TERMINAL_SESSION {
          string session_id PK "UUID"
          string tenant_id FK "UUID"
          string device_fingerprint "SPIFFE Identity ID"
          string status "ACTIVE / CLOSED / COMPROMISED"
          timestamp created_at
      }

      TRANSACTION {
          string transaction_id PK "UUID"
          string session_id FK "UUID"
          string tenant_id FK "UUID"
          decimal amount
          string currency
          string status "COMPLETED / QUEUED / FAILED / REFUNDED"
          boolean is_offline_captured
          string idempotency_key "Unique client-generated key"
          timestamp captured_at
      }

      LEDGER_ENTRY {
          string entry_id PK "UUID"
          string tenant_id FK "UUID"
          string transaction_id FK "UUID"
          decimal amount
          string direction "DEBIT / CREDIT"
          string account_type "REVENUE / CASH_ON_HAND"
          timestamp recorded_at
      }

      INVENTORY_LEVEL {
          string item_id PK "UUID"
          string tenant_id FK "UUID"
          string SKU
          int stock_count
          int safety_stock_threshold
      }

      INVENTORY_MUTATION {
          string mutation_id PK "UUID"
          string item_id FK "UUID"
          string transaction_id FK "UUID"
          int change_quantity
          string mutation_reason "SALE / RESTOCK / SHRINKAGE"
          timestamp mutated_at
      }
  ```

  ---

  ## 5. Mobile-First UX Design (375px Breakpoint)

  The mobile interface is designed specifically for **one-handed, high-vibration environment operation** (e.g., holding a phone while working in Carlos's repair truck or Fatima's food cart). It adheres to the **"grandmother test"** with large tap targets and zero technical jargon.

  ### Screen Wireframe Flow (375px Viewport)

  ```
  +-----------------------------------+  +-----------------------------------+  +-----------------------------------+
  |  OHC POS                      [⚙️] |  |  OHC POS - SWIPE TO TAP       [X] |  |  OHC POS - TRANSACTION SUCCESS    |
  +-----------------------------------+  +-----------------------------------+  +-----------------------------------+
  |                                   |  |                                   |  |                                   |
  |         $120.00                   |  |          $120.00                  |  |             ✨ SUCCESS ✨          |
  |                                   |  |                                   |  |                                   |
  +-----------------------------------+  |      HOLD CARD TO BACK OF PHONE   |  |               $120.00             |
  | [ 1 ]      [ 2 ]      [ 3 ]       |  |                                   |  |                                   |
  |                                   |  |               ( NFC )             |  |   Ledger updated automatically.   |
  | [ 4 ]      [ 5 ]      [ 6 ]       |  |                                   |  |   Inventory decremented:          |
  |                                   |  |              [))))) ]             |  |   - 1x Custom Cake SKU            |
  | [ 7 ]      [ 8 ]      [ 9 ]       |  |                                   |  +-----------------------------------+
  |                                   |  |                                   |  | [✉️ Send Receipt via SMS/Email]    |
  | [ C ]      [ 0 ]      [ . ]       |  |                                   |  |                                   |
  +-----------------------------------+  |                                   |  | [✅ Back to Register            ] |
  |   [ ⚡ TAP TO PAY ON PHONE ]      |  |                                   |  |                                   |
  +-----------------------------------+  +-----------------------------------+  +-----------------------------------+
         (Screen 1: Keypad)                     (Screen 2: NFC Hold)                  (Screen 3: Success State)
  ```

  ### Visual Specifications & Design Tokens (Premium Glassmorphism)
  - **Keypad Buttons:** Sized at `80x80px` (exceeding the 44x44px minimum target), featuring high-contrast typography (`Inter Semibold`, `24px`).
  - **Translucent Glass Materials:** Backdrops use macOS-style backdrop filter: `backdrop-filter: blur(25px) saturate(180%); background: rgba(255, 255, 255, 0.4); border: 1px solid rgba(255, 255, 255, 0.3)`.
  - **Haptic Patterns:**
    - Keypad tap: Light feedback (`medium` impact).
    - Payment success: Double heavy vibration pulse.
    - Payment failure: Persistent triple buzzer vibration pulse.

  ---

  ## 6. AI Agent Department Collaboration Protocol

  The power of OHC lies in the background orchestration. Once the transaction completes, four autonomous departments instantly cooperate behind the scenes:

  ```
                        ┌─────────────────────────────────┐
                        │   Tap-to-Pay Transaction Saved  │
                        └────────────────┬────────────────┘
                                         │
                 ┌───────────────────────┼───────────────────────┐
                 ▼                       ▼                       ▼
      ┌─────────────────────┐ ┌─────────────────────┐ ┌─────────────────────┐
      │ Operations Agent    │ │ Finance Agent       │ │ Customer CS Agent   │
      └──────────┬──────────┘ └──────────┬──────────┘ └──────────┬──────────┘
                 │                       │                       │
      - Decrements stock      - Post entries directly - Auto-generate digital
        instantly.              to Ledger.              receipt and draft a
      - If threshold met,     - Match with online     - "Thank you!" SMS/Email
        triggers dynamic        payout schedules.       with feedback request.
        restock alert.
                 │                       │                       │
                 └───────────────────────┼───────────────────────┘
                                         │
                                         ▼
                              ┌─────────────────────┐
                              │ Advisory Agent      │
                              └──────────┬──────────┘
                                         │
                              - Aggregate weekly trends.
                              - Predict peak hours.
                              - Highlight saved offline
                                transactions.
  ```

  ---

  ## 7. Implementation Prompt for Swarm Agents

  ### Purpose
  Implement the Zero-Hardware Tap-to-Pay backend logic and its multi-tenant synchronization protocols to enable robust, offline-resilient, and safe in-person card transactions.

  ### High-Level User Journey (CUJ)
  1. The client requests a mock Stripe Terminal Connection Token scoped to their active tenant workspace.
  2. The client initiates a transaction of `$120.00` USD, creating an isolated `TerminalSession` and generating a pending `PaymentIntent`.
  3. The client captures the payment (simulating NFC tap). The server captures the intent, verifies the tenant signature, decrements the active inventory levels, records the revenue ledger entry, and schedules a post-purchase workflow.
  4. In the event of offline capture, the client submits a batched array of transaction payloads. The server processes them sequentially, validates the client-side signature, enforces idempotency using client-generated keys, resolves any inventory stock conflicts gracefully, and reconciles the entries.

  ### Non-Prescriptive Acceptance Criteria
  - **Terminal Sessions:** Expose endpoints to safely initialize, audit, and terminate active terminal connections with verified tenant scoping.
  - **Idempotent Card Captures:** Support robust Stripe payment intent setup and capture routines. Duplicate submissions with the same idempotency key must return the original capture receipt instead of creating duplicate debit entries.
  - **Automated Inventory & Ledger Hook:** All terminal captures must execute atomically within a transaction block, ensuring that if inventory decrement or ledger entry creation fails, the payment status rolls back to safe states.
  - **Offline Sync Queue:** Provide a bulk replay API endpoint. It must handle high-throughput sync payloads, process them in order of the client timestamps, ignore duplicate entries gracefully, and flag processing exceptions.
  - **Zero Trust Compliance:** Validate that every request contains an authenticated workload organization header or JWT claiming SPIRE identity. Reject cross-tenant queries with a clean `403 Forbidden` response.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
