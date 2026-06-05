issue_title: "[Research] Enhance Stripe Terminal Integration with Terminal Hardware Discovery and Payment Intents"
issue_description: |
  # Research Report: Stripe Terminal POS Architecture for In-Person Payments

  ## Gap Analysis & Findings
  Currently, the OneHumanCorp (OHC) platform aims to serve omni-channel personas like Priya (The Boutique Owner) and Carlos (The Freelance Handyman) by providing native in-person Point of Sale (POS) functionality. Integrating Stripe Terminal is critical to achieve this.

  Based on an audit of the codebase:
  1. The backend has mock implementations for Stripe Terminal in `src/server/integrations/stripe/terminal.rs` (generating mock connection tokens and payment intents).
  2. The backend has an API exposed in `src/server/api/terminal_api.rs` (`/token` and `/intent`).
  3. The frontend has a mock `StripeTerminalClient.tsx` in `src/ui/next/src/app/pos/terminal/` containing basic logic for Terminal SDK initialization, discovering readers, connecting, and processing payment intents. However, it seems isolated and not deeply integrated with the cart/catalog UI.

  ## Proposed Architecture & Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
    Client[OHC Mobile App / Next.js POS View] -->|Request Connection Token| API[OHC Backend API /api/v1/payments/terminal/token]
    API -->|Create ConnectionToken| Stripe[Stripe API]
    Stripe -->|Token| API
    API -->|Token| Client
    Client -->|Initialize Terminal SDK| Reader[Stripe Reader / Tap to Pay]
    Reader -->|Read Card Data| Client
    Client -->|Create PaymentIntent| API2[OHC Backend API /api/v1/payments/terminal/intent]
    API2 -->|PaymentIntent ID| Client
    Client -->|Confirm PaymentIntent| Stripe
    Stripe -->|Capture Success Webhook| API3[OHC Webhook Handler]
    API3 -->|Update Ledger & Inventory| DB[(PostgreSQL)]
    API3 -->|Notify Success| Client
  ```

  ### Mobile UX Flow (375px Mobile First)
  1. **Checkout UI**: A dedicated "In-Person POS" tab inside the Operations Dashboard.
  2. **Device Discovery**: When a user clicks "Charge", the application presents an option to connect to a nearby physical Stripe Reader or use "Tap to Pay on iPhone/Android" (via native SDKs when deployed on mobile).
  3. **Payment Processing**: The frontend initiates the Payment Intent with the OHC backend, securely proxies it to the Stripe Terminal SDK, and waits for card interaction.
  4. **Confirmation**: A glassmorphic success overlay appears on successful card read and capture.
  5. **Post-Sale Actions**: The AI Operations Agent deducts the sold items from inventory and logs the transaction to the unified `ledger_entries` table.

  ### Key AI Integration Points
  - **The Accountant (Finance Agent)**: Seamlessly reconciles the in-person transaction in the daily and weekly financial summaries.
  - **The Manager (Operations Agent)**: Receives the success event to update multi-channel inventory synchronously.

  ## Implementation Prompt
  As an implementer, your task is to enhance the existing Stripe Terminal POS functionality:
  1. **Backend Integration**: Replace the mock implementation in `src/server/integrations/stripe/terminal.rs` with real calls to the Stripe API using `reqwest` or the `stripe-rust` crate to create `ConnectionToken` and `PaymentIntent` for Stripe Terminal. Ensure these use `payment_method_types=["card_present"]` and `capture_method="manual"`.
  2. **Frontend Wiring**: Integrate `StripeTerminalClient.tsx` into a real cart checkout flow, allowing users to add items to a cart and select "Tap to Pay / Card Reader" as a checkout option.
  3. **State Management**: Ensure that successful payments trigger a backend call to record the ledger entry and update inventory.
  4. **Testing**: Write end-to-end Playwright tests (e.g. updating `src/e2e/e2e_pos_flow.md`) simulating the POS checkout flow using Stripe's simulated readers.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
