issue_title: "Architecture Gap: Missing Stripe Payment Webhook Reconciliation & End-to-End Persistence"
issue_description: |
  ## Title
  Architectural Gap: Missing Payment Webhook Reconciliation and Invoice Fulfillment Tracking

  ## Problem Statement
  Currently, the platform can draft an invoice, but it lacks a resilient architecture to process external payment provider (Stripe) webhook events and reconcile them back to the persistent business ledger. A service professional like Nora (agency principal) needs to know definitively when a client has successfully paid a deposit or final invoice, without having to manually check Stripe. If a payment succeeds but the platform's connection drops, the invoice remains "unpaid" in OHC, preventing the business from proceeding to fulfillment.

  ## Research Report
  - **Market Evidence:** Current workflows for independent service professionals heavily rely on real-time deposit confirmation to begin work. Without automated reconciliation, owners perform double-entry bookkeeping across their payment processor and their CRM/Operations tool.
  - **Codebase Findings:**
    - `src/server/api/invoice.rs`: The invoice system can create drafts and map invoices to views, but does not natively handle automated webhook state transitions from external providers.
    - `F08: fabricated checkout links`: The audit (`docs/research/business_capability_and_usage_economics_audit.md` and `docs/research/native_migration_and_remediation.md`) highlights that provider receipts must be persisted across every workflow, and full provider sandbox replay/payment event reconciliation is outstanding.
  - **Competitive Analysis:** Platforms like Shopify, Wix, and Squarespace provide instant, durable reconciliation of payments that automatically trigger fulfillment workflows. OHC must offer the same reliability to replace manual tracking.

  ## Design Doc
  ### Architecture
  - **Webhook Gateway:** A dedicated, authenticated endpoint to receive signed events from the payment provider (e.g., Stripe `checkout.session.completed`, `invoice.paid`, `invoice.payment_failed`).
  - **Event Deduplication & Persistence:** A new robust transactional layer (e.g. `payment_events` table) that idempotently records the webhook payload before processing, guarding against duplicate deliveries and allowing replay.
  - **Reconciliation Engine:** A worker that consumes validated payment events, verifies the amounts/currency, and updates the canonical `invoices` table (`payment_status`, `amount_paid_cents`).
  - **Agent Notification Hand-off:** Once reconciled, emit an internal domain event so that departments (e.g., Finance or Operations) can automatically notify the owner or transition the job to "ready for delivery".

  ### Mobile UX Flow
  - On the 375px viewport (mobile app), the "Invoices" view will seamlessly transition a card from "Draft" -> "Sent" -> "Paid" using macOS-style Translucent Glass materials.
  - The user receives a brief, plain-language contextual push notification (e.g., "Client X paid the deposit. Ready to start work.") when the webhook is processed.

  ### AI Agent Integration Points
  - The Finance agent monitors the domain events emitted by the reconciliation engine to update revenue forecasting.
  - The Coordinator agent uses the "paid" state change to authorize the next workflow step (e.g. sending a welcome packet).

  ### Key Design Decisions
  - **Idempotency First:** Webhooks can be delivered multiple times. The system must process each unique event ID only once.
  - **Decoupled Processing:** Ingesting the webhook and updating the business logic are separate phases. This ensures we return a `200 OK` to Stripe quickly, avoiding timeouts.
  - **Multi-tenant Isolation:** Every webhook must securely bind to a specific tenant using `client_reference_id` or similar verified metadata.

  ## Implementation Prompt
  **Implementer Instructions:**
  Build the Payment Webhook Reconciliation pipeline. Create a new secure HTTP route to receive Stripe webhooks. Implement an idempotent event ingestion mechanism that records the raw event. Build a reconciliation service that safely updates an invoice's `payment_status` to "paid" upon receiving a valid `checkout.session.completed` event. Ensure you add robust integration tests covering duplicate deliveries, partial payments, and invalid signatures. Do not assume any specific ORM syntax or database table structure; design the entities necessary to meet the idempotency and reconciliation requirements. Verify the flow using test credentials in the sandbox.

  ## Priority
  P0

  ## Estimated Scope
  Medium

  ## Strategy Admission
  - **Target ID:** OHC-05 (Booking/Deposit Payment Reconciliation)
  - **Selected Customer:** Nora (agency principal, requires secure intake/deposit tracking)
  - **Evidence Level:** Code audit (F08) and public workflow observation
  - **Baseline/Result Metric:** Zero manual reconciliation errors, 100% of successful test payments automatically update the OHC ledger.
  - **Dependencies/Reuse:** Reuse existing Postgres pool, existing `Invoice` domain models, and standard Axum routing.
  - **Non-Goals:** Building a completely new ledger; replacing Stripe entirely.
  - **Authority Class:** Owner/System (Automated processing of verified external financial events).
  - **Cost Plan:** Minimal computational cost (webhook ingest); potential Stripe API retrieval costs (mitigated by processing payload directly).
  - **Acceptance Checks:** Happy path (valid webhook marks invoice paid); Failure path (invalid signature is rejected, duplicate webhook is ignored).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
