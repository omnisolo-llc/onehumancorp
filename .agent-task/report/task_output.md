issue_title: "Implement Stripe Webhook Validation & Deduplication"
issue_description: |
  **Problem Statement:**
  Currently, OneHumanCorp's payment generation creates checkout URLs without properly tracking provider sessions, leaving the system vulnerable to duplicate payments and missed fulfillments. For a non-technical owner, a missing or duplicate charge damages customer trust and introduces manual reconciliation work.

  **Research Report:**
  Our audit found that Stripe placeholder code coexists with real plumbing, and `invoice.rs` creates checkout UUIDs without persistent provider sessions. Stripe specifically advises using Webhook endpoints for fulfillment, and notes that webhooks can arrive out-of-order or duplicate. Without webhook signature verification, durable deduplication (e.g., tracking `stripe_event_id`), and idempotency logic, OHC risks executing fulfillments multiple times or missing them entirely.

  **Design Doc:**
  - **Webhook Endpoint:** Introduce a new securely mounted route (`/webhooks/stripe`) that validates Stripe signatures using a securely injected endpoint secret.
  - **Deduplication Ledger:** Introduce a persistent table or state tracking for `stripe_event_id` to ensure each event is processed exactly once.
  - **Fulfillment Engine:** Parse `checkout.session.completed` events, match them against internal invoices/proposals, update the status to paid, and trigger downstream fulfillment workflows.

  **Implementation Prompt:**
  Implement a Stripe webhook listener that validates signatures, deduplicates incoming events, and safely updates invoice/proposal statuses. Do not invent a new generic payment layer; use the existing `StripeClient` where applicable, but replace simulated checkout success with real session tracking and webhook-driven fulfillment logic. Provide concrete test coverage for signature validation failure and duplicate event drops.

  **Priority:** P0 (critical for billing reliability)
  **Estimated Scope:** Medium

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
