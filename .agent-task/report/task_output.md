issue_title: "Implement POS Tap-to-Pay and Stripe Terminal Flow"
issue_description: |
  # Problem Statement
  Small business owners (like Priya with her boutique or Carlos the handyman) need to accept in-person payments securely without relying entirely on complex external POS hardware setups or carrying multiple payment terminals. Currently, OmniSolo does not provide a streamlined, native Tap-to-Pay or integrated Stripe Terminal flow within the mobile-first interface. This forces owners to switch apps or use disconnected hardware, leading to broken data sync (inventory/financials) and manual reconciliation.

  # Research Report
  - **Shopify POS:** Offers deep integration with Stripe terminal and mobile Tap-to-Pay, providing unified inventory and payment tracking. (Source: Shopify POS Documentation, https://help.shopify.com/en/manual/sell-in-person)
  - **Square:** Dominates the physical POS space but forces users into their ecosystem, often lacking the broader workflow automation (like automated follow-ups and invoicing) OmniSolo provides. (Source: Square Developer Documentation, https://developer.squareup.com/docs)
  - **Stripe Terminal / Tap-to-Pay on iPhone/Android:** Stripe provides SDKs and APIs to enable mobile devices to act as payment terminals directly (Tap-to-Pay) or connect to physical readers (like BBPOS WisePad 3 or Stripe Reader M2). (Source: Stripe Terminal Documentation, https://docs.stripe.com/terminal)
  - **Owner Pain Points:** Switching contexts during a sale, hardware pairing issues, and disconnected reporting. The "Grandmother Test" requires this to be a one-tap flow from the OmniSolo POS interface. (Source: Operator Community Forums and Reviews)
  - **Zettle by PayPal:** Integrates smoothly but similarly locks users into their specific hardware rather than allowing bring-your-own-device Tap-to-Pay effectively. (Source: Zettle Support, https://www.zettle.com/us/help)
  - **LightSpeed POS:** Offers complex inventory but lacks the modern, simple, phone-first Tap-to-Pay that solo operators need. (Source: Lightspeed Retail POS, https://www.lightspeedhq.com/pos/retail/)

  # Design Doc
  ## Architecture

  ```mermaid
  sequenceDiagram
      participant User as Mobile UI (Priya)
      participant API as OmniSolo API (Rust)
      participant Stripe as Stripe API
      participant Reader as Stripe Reader / Tap

      User->>API: GET /api/v1/payments/terminal/token
      API->>Stripe: Request Connection Token
      Stripe-->>API: Connection Token Secret
      API-->>User: Token Secret

      User->>User: Initialize Stripe Terminal SDK
      User->>User: Connect to Reader

      User->>API: POST /api/v1/payments/terminal/intent (amount)
      API->>Stripe: Create PaymentIntent (metadata: tenant_id)
      Stripe-->>API: client_secret
      API-->>User: client_secret

      User->>Reader: processPayment(client_secret)
      Reader-->>User: Payment Processed

      User->>API: POST /api/v1/payments/terminal/capture
      API->>Stripe: Capture PaymentIntent
      Stripe-->>API: Capture Success
      API->>API: Update Ledger & Inventory
      API-->>User: Success Notification
  ```

  ## Data Model & Invariants

  ```mermaid
  erDiagram
      Tenant ||--o{ Transaction : has
      Transaction ||--o{ LedgerEntry : creates
      Tenant ||--o{ InventoryItem : owns
      Transaction }o--|| InventoryItem : deducts

      Tenant {
          UUID tenant_id PK
          string name
      }
      Transaction {
          UUID id PK
          UUID tenant_id FK
          string stripe_payment_intent_id
          integer amount_cents
          string currency
          string status
          datetime created_at
      }
      LedgerEntry {
          UUID id PK
          UUID tenant_id FK
          UUID transaction_id FK
          integer amount_cents
          string account_type
      }
      InventoryItem {
          UUID id PK
          UUID tenant_id FK
          string name
          integer quantity
      }
  ```

  **Strict Multi-Tenant Isolation Rules:**
  - All database queries for `Transaction`, `LedgerEntry`, and `InventoryItem` must include a `WHERE tenant_id = $1` clause.
  - The Stripe `PaymentIntent` creation must include `metadata[tenant_id]` to track transactions securely at the gateway level.
  - The `StripeClient` must retrieve the API key specific to the current `tenant_id` from the environment or secure config.

  ## Mobile UX Flow (375px first)
  1.  **Cart View:** User adds items. Large "Charge $X.XX" button at the bottom (sticky). The layout uses `.glass-control` (8px radius) for the button.
  2.  **Payment Method Selection:** Slide-up drawer offering "Tap to Pay (Phone)", "Bluetooth Reader", or "Manual Entry". The drawer uses `.translucent-glass-light` (16px radius).
  3.  **Connecting State:** If a reader is needed, a simple spinner with "Connecting to Reader..." to keep the user informed.
  4.  **Ready State:** Large, clear icon indicating "Tap Card or Phone Now".
  5.  **Success:** Confetti animation, "Payment Successful", and quick action buttons for "Email Receipt" or "New Sale".

  ## AI Agent Integration Points
  - **Accounting & Tax Agent:** Will automatically parse the captured `Transaction` and categorize the corresponding `LedgerEntry` as revenue, updating daily cash flow projections.
  - **Inventory Agent:** Will listen for successful `Transaction` events and automatically deduct `InventoryItem` quantities based on the items in the POS cart, triggering low-stock alerts if necessary.
  - **Support & Voice Agent:** Can reference the physical POS transaction if the customer calls or messages later regarding a return or receipt.

  ## Key Decisions
  - Use Stripe Terminal as the backend payment processor to unify online and in-person payments under one system, reducing the need for disjointed systems.
  - Prioritize a smooth, error-resilient connection flow for the hardware. If the SDK fails to load, gracefully fallback or show a clear error.
  - Maintain zero trust by ensuring the connection token generation always verifies the `tenant_id` before calling Stripe.

  # Implementation Prompt
  Implement the full POS Tap-to-Pay flow using Stripe Terminal.
  1.  Enhance the existing Rust backend endpoints in `src/server/api/terminal_api.rs` to handle connection tokens, intent creation, and intent capture using the existing `StripeClient` integration. Ensure strict multi-tenant isolation by passing `tenant_id` consistently.
  2.  Build the Next.js frontend UI (`/pos/terminal`) that simulates connecting to a reader, creating an intent, and simulating a successful payment. Use the OHC Premium Design Standards (`.glass-control` for buttons, `.translucent-glass-light` for containers).
  3.  Ensure the UI follows the 375px mobile-first standard with large, accessible buttons (min 44x44px touch targets).
  4.  Write robust Playwright E2E tests verifying the complete flow from adding an item to the cart, connecting the reader (mocked), and confirming the successful payment intent.
  5.  Ensure zero mock data is used for products/inventory; the transaction must reflect in the real database ledger. Create necessary setup flows to provision test inventory data if needed.

  # Priority
  P1

  # Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
