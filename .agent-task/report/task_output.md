issue_title: "[Architecture] Hardware-Free Tap-to-Pay POS Integration"
issue_description: |
  # [Architecture] Hardware-Free Tap-to-Pay POS Integration

  ## Problem Statement
  Small business owners who operate in person—like Priya the boutique owner or Fatima the food cart operator—need a frictionless way to accept in-person payments without purchasing, pairing, and maintaining expensive external POS hardware (e.g., Square readers or dedicated terminals). Currently, OHC lacks a native, seamless in-person POS capability that connects directly to the unified OHC mobile app. The "Setup Complexity" of legacy hardware POS solutions creates significant friction and fails the "grandmother test." We need an integrated, zero-hardware solution utilizing native mobile NFC (Tap-to-Pay on iPhone/Android) to unify their online and offline sales, inventory, and ledger in real-time.

  ## Research Report
  ### Context and Market Analysis
  In-person sales remain a vital revenue channel for many SMBs. Traditional solutions require external bluetooth or physical plug-in card readers.
  - **Shopify:** Requires external POS hardware or specific Tap-to-Pay iOS/Android apps that are often separate from the primary management app, causing fragmentation.
  - **Square:** Known for their hardware, but shifts to Tap-to-Pay require their specific ecosystem which may lock users out of a unified platform.
  - **Wix/Squarespace:** Point of Sale capabilities exist but often rely heavily on third-party hardware integrations (like Stripe Terminal external readers), causing setup complexity.
  - **Stripe Terminal:** Offers Tap-to-Pay SDKs that allow merchants to accept payments directly on their mobile devices using NFC without extra hardware. This is the ideal technology enabler for OHC.

  By directly embedding Tap-to-Pay via Stripe Terminal SDKs into the primary OHC app, we can completely bypass external hardware. This positions OHC in the "Leapfrog Zone" (High Autonomy, Radical Simplicity), allowing a merchant to open the app, enter an amount, and instantly have a customer tap their card on the merchant's phone.

  ### Key Learnings
  1. **Hardware is Friction:** External readers battery dies, lose bluetooth pairing, or break.
  2. **Unified Data is Critical:** Inventory, sales, and analytics must reflect in-person sales instantly alongside online sales.
  3. **Offline Resilience:** Food carts or pop-up shops (e.g., Fatima) may have spotty cellular connections; the POS flow must gracefully handle low connectivity.

  ## Design Doc
  ### Key Design Decisions
  - **Zero-Hardware Approach:** Fully leverage Apple Tap to Pay on iPhone and Android native NFC Tap-to-Pay. No bluetooth readers.
  - **Unified Ledger & Inventory:** In-person transactions must directly mutate the same core Ledger and Inventory entities as online sales to prevent double-selling.
  - **Offline Mode & Queueing:** Implement an offline-capable transaction queue. If the network is unavailable, transactions are queued locally securely and synced when connectivity is restored, ensuring no lost sales in spotty environments.
  - **Zero Trust Security:** Enforce strict multi-tenant isolation at the terminal session level using SPIFFE/SPIRE-backed identities to guarantee one merchant cannot access another's transactions.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      MERCHANT ||--o{ TERMINAL_SESSION : initiates
      TERMINAL_SESSION ||--|| TRANSACTION : processes
      TRANSACTION }|--|| LEDGER : records
      TRANSACTION }|--o{ INVENTORY : decrements

      MERCHANT {
          string id PK
          string name
          string currency
      }
      TERMINAL_SESSION {
          string id PK
          string merchant_id FK
          string status "active/closed/offline"
          timestamp started_at
      }
      TRANSACTION {
          string id PK
          string session_id FK
          decimal amount
          string status "pending/queued/completed/failed"
          timestamp created_at
      }
      LEDGER {
          string id PK
          string transaction_id FK
          decimal amount
          string type "credit"
      }
      INVENTORY {
          string item_id PK
          int quantity
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Home Dashboard:** Large primary CTA: "Accept Payment".
  2. **Amount Entry:** Full-screen numeric keypad. High contrast, large touch targets (>44x44px).
  3. **Checkout Selection:** "Tap to Pay" is the default. Other options (Cash, QR Code) are secondary.
  4. **NFC Interaction:** Native OS Tap-to-Pay overlay appears. The merchant holds out their phone; the customer taps their card or phone.
  5. **Success & Receipt:** Haptic feedback + green checkmark. Immediate option to send a digital receipt (SMS/Email).

  ### AI Agent Integration Points
  - **Finance & Payments Agent:** Monitors completed Tap-to-Pay transactions, updates daily revenue targets, and reconciles the unified ledger.
  - **Operations Agent:** Deducts sold items from the shared inventory immediately. If stock drops below a threshold, it queues a restock alert for the merchant.
  - **Business Advisory Agent:** Includes offline sales data in the weekly plain-language briefing (e.g., "In-person sales at the farmer's market made up 40% of your revenue this weekend!").

  ## Implementation Prompt
  **Task:** Implement the Mobile Tap-to-Pay Backend Service and Terminal Session Manager.
  **User Story:** As a merchant, I want to initiate a Tap-to-Pay session from my mobile app so that I can accept in-person card payments without external hardware, and have those sales automatically update my central inventory and ledger.
  **Acceptance Criteria:**
  - Define the data schema for a `TerminalSession` and `OfflineTransactionQueue` with strict multi-tenant isolation.
  - Create an API endpoint for the mobile app to initialize a new Terminal Session (mocking the Stripe Terminal SDK connection token generation).
  - Create an API endpoint to record a completed transaction, which must atomically update the centralized `Ledger` and `Inventory` domains.
  - Implement robust offline-handling logic: API must accept a batch of offline-queued transactions and replay them sequentially, ensuring idempotency and eventual consistency.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
