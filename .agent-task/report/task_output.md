issue_title: "[Architecture] Stripe Terminal & Unified Point of Sale (POS) Architecture"
issue_description: |
  # Architecture Design: Stripe Terminal & Unified Point of Sale (POS)

  ## 1. Problem Statement
  For omni-channel personas like Priya (boutique owner) and Carlos (handyman), the inability to take in-person payments natively through OHC forces them into fragmented workflows. Currently, OHC handles online payments (Stripe Checkout) but lacks integration for physical retail/in-person transactions. If users can't accept contactless payments (Tap-to-Pay) directly through the OHC mobile app, they cannot trust OHC as their primary business operating system.

  ## 2. Research Report
  - **Competitor Analysis**: Shopify and Square provide heavily integrated online/offline POS capabilities. Traditional website builders (Wix/Squarespace) treat POS as a secondary add-on. OHC has an opportunity to offer a native "Tap-to-Pay" experience that works invisibly via the mobile app, powered by the Operations Agent.
  - **The Gap**: `src/server/integrations/stripe/` supports checkout and subscriptions, but entirely lacks Stripe Terminal support (connection token generation, POS payment intent flow).
  - **Data Sync Constraints**: OHC currently lacks a real-time, strongly consistent distributed inventory lock for hybrid checkout to prevent an online customer from purchasing the same item someone is currently buying in-store.

  ## 3. Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      MobileApp[OHC Mobile App - POS] -->|1. Request Terminal Token| Backend[OHC Rust Backend]
      Backend -->|2. Create ConnectionToken| StripeAPI[Stripe API]
      StripeAPI -->|3. Token| Backend
      Backend -->|4. Token| MobileApp
      MobileApp -->|5. Initialize Stripe Terminal SDK| TapToPay[Tap-to-Pay / Card Reader]
      TapToPay -->|6. Process Card| MobileApp
      MobileApp -->|7. Create PaymentIntent| Backend
      Backend -->|8. Inventory Redlock & Confirm Payment| StripeAPI
      StripeAPI -->|9. Capture Success Webhook| Backend
      Backend -->|10. Ledger & Inventory Sync| DB[(PostgreSQL)]
      Backend -->|11. Trigger AI Events| OperationsAgent[Operations Agent]
  ```

  ### Mobile UX Flow (375px)
  1. **POS Tab**: OHC app presents a calculator-style keypad and catalog grid.
  2. **Charge Button**: Tapping "Charge" initializes the Stripe Terminal Tap-to-Pay SDK natively.
  3. **Payment Capture**: A modal overlay prompts the customer to tap their card.
  4. **AI Workflow**: Post-payment, the Operations Agent sends a summary ("Sale complete. Red dress inventory updated.")

  ### Key Design Decisions
  - **Token Generation Service**: The Rust backend must expose secure `/api/v1/payments/terminal/token` endpoints.
  - **Unified Inventory Context**: Connect in-person payment intents strictly to OHC order and inventory ledgers to trigger proper stock reduction and accounting via the Finance Agent.
  - **Zero Trust Isolation**: All tokens and sessions strictly bound to `tenant_id` claims via SPIFFE/Auth.

  ## 4. Implementation Prompt
  **Feature Name**: Stripe Terminal Connection & Unified POS Backbone
  **Target Persona**: Priya (Boutique Owner)
  **Outcome**: Priya can process in-person payments through the OHC mobile app, which instantly updates central inventory and accounting.

  **Next Actions for Implementer**:
  1. Expand the existing `terminal.rs` module under `src/server/integrations/stripe/` to act as the core for Stripe Terminal integration.
  2. The system should define secure backend API routes to generate Stripe Terminal Connection Tokens.
  3. Support recording POS transactions and connecting them to inventory systems.
  4. Integrate Redis Redlock inventory locking during checkout (Terminal transaction lifecycle).
  5. Add end-to-end (E2E) tests in Playwright covering the Terminal token request flow using Stripe test-mode constraints. Focus entirely on the Rust backend/API definitions and database synchronization. Do not prescribe specific database schemas or API signatures; design those during implementation.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
