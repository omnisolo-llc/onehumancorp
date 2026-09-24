issue_title: "Implement Persistent Provider Receipts & Checkout Verification"
issue_description: |
  # Architecture Brief: Persistent Provider Receipts & Checkout Verification

  ## Title
  Implement Persistent Provider Receipts & Checkout Verification

  ## Problem Statement
  Currently, as identified in finding F08 ("fabricated checkout links"), OHC returns unavailable placeholder checkout states instead of generating a URL. Small business owners (like Carlos the handyman or Nora the agency principal) need a way to reliably convert generated invoices into real provider sessions (like Stripe) and accurately reconcile payments back into the OHC ledger without assuming false completion. Without a persistent provider receipt boundary, OHC risks sending out fake payment success notifications or losing track of a customer's payment status, causing trust issues and lost revenue.

  ## Strategy Admission
  - **OHC Target ID**: OHC-08 (booking/deposit, delivery/review, final payment exceptions)
  - **Customer/Stage**: Run Stage (Nora, Carlos, Maya).
  - **Evidence Level**: Documented in RESEARCH.md (F08).
  - **Metric**: Zero duplicate payment records, 100% provider webhook reconciliation.
  - **Dependencies**: Stripe integration, OHC invoice engine.
  - **Non-Goals**: Building a generic ERP or replacing Stripe completely.

  ## Research Report
  Our current invoice creation logic generates draft invoices without valid provider sessions (previously fabricating checkout links). Competitors like HoneyBook and Square provide seamless, atomic generation of provider links mapped directly to their internal invoice state. To ensure financial integrity and user trust, we must build a system that:
  1. Issues a real provider checkout session (e.g. Stripe Checkout) only when requested.
  2. Persists the provider `session_id` and receipt status directly in the `ohc_universal_ledger`.
  3. Relies strictly on idempotent webhooks from the provider to transition the invoice from `PENDING` to `PAID`.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Nora (Mobile App)
      participant OHC as OHC Gateway
      participant Agent as Billing Agent
      participant Ledger as ohc_universal_ledger
      participant Provider as Stripe (Provider)

      Owner->>OHC: Finalize & Send Invoice
      OHC->>Agent: Generate Payment Link Request
      Agent->>Provider: POST /v1/checkout/sessions (Idempotent)
      Provider-->>Agent: session_id, url
      Agent->>Ledger: UPDATE Invoice (status=PENDING, stripe_session_id=session_id)
      Agent-->>Owner: Invoice Sent Notification

      Note over Provider, Ledger: Customer pays via Stripe

      Provider->>OHC: Webhook: checkout.session.completed
      OHC->>Ledger: Verify session_id & UPDATE Invoice (status=PAID)
      Ledger-->>Agent: Trigger Follow-up AI Workflow
  ```

  ### Mobile UX Flow (375px)
  1. **Dashboard**: Nora sees a list of drafts. She taps "Send Invoice".
  2. **Confirmation Sheet**: A bottom sheet slides up, applying Translucent Glass (`backdrop-filter: blur(30px) saturate(210%)`, `background: rgba(255,255,255,0.65)` in Light Mode). It shows the total and a "Generate & Send Link" button.
  3. **Loading State**: An inline skeleton loader appears while the agent communicates with Stripe.
  4. **Success Toast**: "Invoice sent to client. Tracking payment."

  ### AI Agent Integration Points
  - **Billing Agent (Sub-Agent Orchestration)**: Handles the API call to Stripe. Uses explicit idempotency keys based on the invoice ID.
  - **Webhook Listener**: A passive system component that validates Stripe signatures and updates the database, notifying the agent to trigger the "Thank You" or "Service Delivery" flow.

  ## Implementation Prompt
  Implement the checkout verification and provider receipt persistence flow. Modify the invoice generation process to request a real Stripe session when finalizing an invoice, persisting the `stripe_session_id` in the database. Ensure the webhook endpoint correctly matches incoming `checkout.session.completed` events to the persisted session ID, updating the ledger atomically. The UI must cleanly reflect `DRAFT`, `PENDING` (link generated), and `PAID` states. Ensure no live customer accounts are used in tests, and mock the provider boundaries.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
