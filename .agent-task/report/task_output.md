issue_title: "F08: Invoice and booking paths fabricate checkout links"
issue_description: |
  **Title**: F08: Invoice and booking paths fabricate checkout links

  **Problem Statement**:
  The system currently creates fake, checkout-looking URLs for invoices and bookings instead of integrating with a real payment provider session. The invoice creation logic explicitly notes: `// This operation creates a local draft, not a provider checkout session. // Empty means payment has not been configured; never invent a payable URL. let stripe_payment_link = String::new();`. This lack of an actual payment integration provides a false sense of completion and violates the system's mandate for truthfulness and evidence of outcome, failing to support a vital owner capability (collect and reconcile money).

  **Research Report**:
  - The `docs/research/business_capability_and_usage_economics_audit.md` explicitly lists `F08: fabricated checkout links` as an open issue.
  - The audit states: `Invoice creation separately generates a checkout-looking UUID URL without creating a provider session (invoice.rs:45-48). Real Stripe plumbing and placeholders coexist; consolidate and verify rather than claiming all payments are fake or all are ready.`
  - In `src/server/api/invoice.rs`, the code correctly assigns an empty string to `stripe_payment_link`, leaving the issue of the "fabricated checkout URL" unaddressed, or at least requiring a real payment implementation.
  - In `src/server/api/booking/public.rs`, `stripe_url` is hardcoded to `None` when a deposit is required (`if requires_deposit && deposit_cents > 0 { stripe_url = None; ... }`).
  - This requires replacing the placeholder logic with a real provider test session (e.g., Stripe Checkout Session) or returning an explicit unavailable state, while ensuring idempotency and persistence of provider IDs.

  **Design Doc**:
  - **Architecture diagram**:
    ```mermaid
    sequenceDiagram
      participant User
      participant Invoice/Booking Route
      participant Payment Service (Stripe)
      participant Database

      User->>Invoice/Booking Route: Request creation with deposit/payment
      Invoice/Booking Route->>Database: Insert record (Pending/Draft)
      Invoice/Booking Route->>Payment Service (Stripe): Create Checkout Session
      Payment Service (Stripe)-->>Invoice/Booking Route: Return Session URL & ID
      Invoice/Booking Route->>Database: Update record with Stripe Session URL & ID
      Invoice/Booking Route-->>User: Return record with Stripe URL
    ```
  - **Mobile UX Flow**: For a 375px viewport, the user submits a booking or invoice generation form. Instead of seeing a fake success or local URL, the user is presented with a clear action to "Pay Deposit" or "Pay Invoice", which links directly to the generated Stripe Checkout URL. If the Stripe integration is not configured, the UI explicitly states "Payment configuration pending".
  - **AI Agent Integration Points**: The AI coordinator handling money/records must be able to read the returned Stripe Session URL to provide it to the customer, and must listen for webhook events to mark the invoice/booking as paid, instead of relying on fake status changes.
  - **Key design decisions**: Real Stripe checkout session integration is required. If the system is not configured with Stripe credentials, it must return an explicit `pending`/`unavailable` state for the payment link rather than a fabricated one.

  **Implementation Prompt**:
  Update `src/server/api/invoice.rs` and `src/server/api/booking/public.rs` to generate actual Stripe Checkout Sessions when payment is required, using the existing Stripe client if available. If Stripe is not configured or fails, do not invent a URL; return an explicit empty or unavailable state. Persist the real Stripe Session ID and URL in the database. Ensure the response clearly reflects whether a real payment session was successfully created. Ensure the E2E tests are updated to handle the new real checkout behavior or the explicit pending state if running without Stripe credentials.

  **Priority**: P0 (critical for correctness and money handling).

  **Estimated Scope**: Medium.

  **Strategy Admission**:
  - Stable OHC target ID: OHC-01 (Trace the active service journey; eliminate false booking/invoice payment success)
  - Selected customer/stage: Nora (solo web/design agency)
  - Baseline/result metric: Verified business outcomes (collected/reconciled invoice)
  - Dependencies/reuse: Stripe client, existing database schema.
  - Non-goals: Implementing alternative payment providers like Mercado Pago (this is a separate epic).
  - Authority class: Routine external work (creating checkout sessions).
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
