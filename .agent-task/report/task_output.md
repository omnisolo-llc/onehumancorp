issue_title: "[Architecture] Mobile POS & Inventory Unification (Stripe Terminal Tap-to-Pay)"
issue_description: |
  ## Problem Statement
  For omni-channel personas like **Priya (Boutique Operator)** and **Carlos (Field Service Owner)**, taking payments in-person while keeping online inventory and ledgers perfectly synced is a hard requirement. Currently, OHC lacks native Point-of-Sale (POS) infrastructure. If Priya sells her last "Red Dress" in-store, she must manually deduct it online, risking double-selling. Carlos cannot easily take an in-person deposit without redirecting the customer to an online link. They need an integrated POS that bridges the physical and digital seamlessly, without the friction of complex third-party setups.

  ## Research Report
  ### Competitor Analysis
  - **Shopify POS:** Offers excellent native POS and proprietary hardware, keeping inventory synced. However, the ecosystem feels disjointed unless merchants use Shopify's highest tiers or dedicated POS hardware.
  - **Square:** Unbeatable hardware-to-software integration for micro-SMEs, but their e-commerce/online-store offering feels secondary to the physical POS.
  - **Wix/Squarespace:** Point-of-Sale integrations often require complex third-party app connections, confusing non-technical users.
  - **Stripe Terminal:** Provides "Tap to Pay" SDKs that allow merchants to accept payments directly on their mobile devices using NFC without extra hardware.

  ### Opportunity for OHC
  By directly embedding **Tap-to-Pay via Stripe Terminal SDKs** into the primary OHC app, we can bypass external hardware completely. This places OHC in the "Leapfrog Zone" (High Autonomy, Radical Simplicity). A merchant can open the app, enter an amount, and have a customer tap their card on the merchant's phone. Crucially, the AI Operations Agent will invisibly reconcile the payment with the central PostgreSQL ledger and decrement inventory in real-time, preventing overselling.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      MobileApp[OHC Flutter Mobile App] -->|1. Request Connection Token| API[OHC Go API Layer]
      API -->|2. POST /v1/terminal/connection_tokens| StripeAPI[Stripe API]
      StripeAPI -->|3. Return Token| API
      API -->|4. ConnectionToken| MobileApp
      MobileApp -->|5. Initialize Session| TerminalSDK[Stripe Terminal Tap-to-Pay SDK]
      TerminalSDK -->|6. NFC Read| Card[Customer Card / Apple Pay]
      TerminalSDK -->|7. Confirm Payment| StripeAPI
      StripeAPI -->|8. Webhook: payment_intent.succeeded| WebhookHandler[OHC Webhook Handler]
      WebhookHandler --> OperationsAgent[AI Operations Agent]
      OperationsAgent -->|9. Deduct Inventory & Record Ledger| DB[(PostgreSQL Row-Level Security)]
  ```

  ### Mobile UX Flow (375px First)
  1. **Checkout Action:** Priya taps a "Charge" button on an order or directly enters an amount on the POS tab.
  2. **Payment Modality:** The app presents "Tap to Pay on Phone" as the primary, zero-friction option.
  3. **Tap to Pay UI:** The native iOS/Android contactless payment UI slides up over the OHC app.
  4. **NFC Interaction:** The customer taps their physical card or phone to the merchant's device.
  5. **Completion State:** A premium, translucent glass loading spinner appears, transitioning to a checkmark.
  6. **Agent Notification:** A brief, transient toast from the AI Finance Agent confirms: "Payment recorded and inventory synced."

  ### AI Agent Integration Points
  - **AI Operations Agent:** Upon receiving the successful webhook, this agent instantly decrements the unified inventory count in the database.
  - **AI Finance Agent:** Reconciles the Stripe Terminal payment into the unified ledger, ensuring it appears correctly in the daily/weekly revenue summaries.

  ### Key Design Decisions
  - **Tap-to-Pay First:** Focus on the hardware-free SDK integration first to dramatically lower the barrier to entry.
  - **Single Source of Truth:** `tenant_id` isolated PostgreSQL tables will serve as the absolute source of truth for both online and offline inventory, ensuring perfect synchronization.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your task is to implement the robust backend infrastructure to support Stripe Terminal Tap-to-Pay operations.

  1. Implement the API endpoints in a new Go module (e.g., `src/server/api/terminal_api.go`) to handle Stripe Terminal operations securely. Specifically, you need to implement token generation, payment intent creation, and offline sync handlers.
  2. Define and implement the database schema/models (e.g., in a new `terminal.go` or `pos.go` module under `src/server/db/`) for recording POS sessions and linking them to transactions and inventory changes. Use proper row-level security (`tenant_id`).
  3. Integrate the backend with Stripe's API to fetch real `ConnectionTokens` and manage `PaymentIntents` for Terminal.
  4. Ensure the Webhook handlers can correctly identify Terminal payments and trigger the necessary inventory/ledger updates.
  5. Write comprehensive unit tests for the new database models and API logic, ensuring 100% test coverage for the new code.
  6. Create E2E/integration tests verifying the backend flow from connection token request to mock webhook processing.

  *Note: Do not build the Flutter UI or the actual mobile SDK integration in this step. Focus purely on the Go backend architecture, database schemas, and API contracts.*

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, pos]
assignees: []
