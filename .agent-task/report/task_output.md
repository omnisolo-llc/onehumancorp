issue_title: "Webhook Architecture Redesign for Idempotency and Scalability"
issue_description: |
  # Webhook System Design Deep Dive

  ## Problem Statement
  Currently, the webhook handler (`src/server/api/billing_webhook.rs`) performs direct, synchronous database and Redis updates upon receiving billing events (like `checkout.session.completed` from Stripe). This architecture lacks idempotency controls, retry mechanisms (Dead Letter Queues), and robust multi-tenant security isolating tier updates. If a webhook is processed twice or the database is briefly unavailable, it causes billing inconsistencies, which directly impacts our personas (e.g., Priya upgrading to Pro, or Carlos experiencing a failed renewal).

  ## Research Report
  - **Findings**: The existing `stripe_webhook_handler` and `mercadopago_webhook_handler` immediately attempt database mutation. There is no `event_id` tracking for idempotency.
  - **Competitor Analysis**: Leading SaaS platforms (Shopify, Stripe) decouple ingestion from processing. Webhooks are ingested, signature verified, and then enqueued. An asynchronous worker processes them, tracking idempotency via a `processed_events` table to prevent double-billing or incorrect plan downgrades.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  sequenceDiagram
      participant Stripe
      participant EdgeIngress as API (Webhook Handler)
      participant JobQueue as DB (webhook_events)
      participant Worker as Idempotent Processor
      participant DB as Tenant DB

      Stripe->>EdgeIngress: POST /api/v1/webhooks/stripe (event)
      activate EdgeIngress
      EdgeIngress->>EdgeIngress: Verify Signature
      EdgeIngress->>JobQueue: Insert raw event (pending)
      EdgeIngress-->>Stripe: 202 Accepted
      deactivate EdgeIngress

      Worker->>JobQueue: DEQUEUE (SKIP LOCKED)
      activate Worker
      Worker->>DB: Check Idempotency Key (event.id)
      alt Key exists
          Worker-->>JobQueue: Mark event complete
      else Key does not exist
          Worker->>DB: Process Tier Assignment (Tenant Isolation)
          Worker->>DB: Insert Idempotency Key
          Worker-->>JobQueue: Mark event complete
      end
      deactivate Worker
  ```

  ### Architecture Blueprint
  1. **Edge Ingress**:
     - Receive payload, strictly verify cryptographic signatures (Stripe/MercadoPago).
     - Respond 202 Accepted immediately.
  2. **Job Queue (PostgreSQL SKIP LOCKED)**:
     - Insert the raw event into a `webhook_events` table with state `pending`.
  3. **Idempotent Processor (Worker)**:
     - Dequeue event.
     - Check `idempotency_keys` table. If exists, skip.
     - Process tier assignment/downgrade applying strict Row Level Security multi-tenant isolation.
     - Update tenant plan.
     - Record idempotency key and mark event `completed`.
  4. **AI Agent Integration Points**:
     - When a subscription upgrade is successful, the Finance & Payments ("The Accountant") agent evaluates the new capacity and updates caching parameters.
     - If a payment fails repeatedly, the Customer Success ("The Ambassador") agent drafts and queues a plain-language follow-up email/SMS to the business owner explaining the issue securely.
  5. **Mobile-First UX**:
     - Optimistic UI updates on the 375px client for plan upgrades, resolving the final state via Server-Sent Events (SSE) or polling.

  ### Implementation Prompt
  **To the Implementer**:
  Implement the decoupled webhook processing pipeline.
  1. Create a `webhook_events` DB schema for queueing.
  2. Create an `idempotency_keys` table.
  3. Refactor `src/server/api/billing_webhook.rs` to enqueue events rather than processing synchronously.
  4. Implement an async background worker using Postgres `SKIP LOCKED` to process the events.
  5. Ensure 100% test coverage and Playwright E2E verification for the upgrade journey.

  **Estimated Scope**: Medium

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
