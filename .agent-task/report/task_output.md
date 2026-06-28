issue_title: "Architecture Design: AI-Driven Chargeback & Dispute Defense System"
issue_description: |
  ## Title
  AI-Driven Chargeback & Dispute Defense System

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Maya (baker) face a massive operational burden when dealing with payment disputes and chargebacks. Compiling evidence (receipts, shipping confirmation, customer communications, and signed contracts) is a manual, highly stressful, and time-sensitive process. Missing a deadline means lost revenue and potential penalties from payment processors like Stripe. Current platforms do not automatically synthesize the context of a sale across omnichannel touchpoints into a unified defense.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Stripe/Square:** Provide dispute dashboards and basic webhook notifications, but require the merchant to manually collect and submit evidence via their portals.
  - **Shopify:** Offers basic fraud analysis, but the merchant still has to manually upload documents for chargeback responses.
  - **OHC Opportunity:** Utilize the OHC multi-agent architecture (specifically, the Finance Agent "The Accountant" and Customer Success Agent "The Ambassador") to automatically detect a dispute webhook, fetch the omnichannel customer memory graph (order history, communication logs, delivery tracking, tap-to-pay location data), and instantly draft a comprehensive evidence package for the owner to review and submit with one tap. This turns a complex 2-hour legal task into a 30-second approval.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Payment Processor Webhook - e.g. Stripe Dispute] -->|Ingress| B[Payment Event Webhook Handler]
      B --> C[PostgreSQL Ledger & Job Queue]
      C --> D[Finance Agent - The Accountant]
      D --> E[Omnichannel Memory Graph: Fetch Comms & Orders]
      D --> F[Operations Agent: Fetch Delivery/Fulfillment Proof]
      E & F --> D
      D --> G[Draft Chargeback Evidence Package]
      G --> H[Owner Feed: Action Required - Approve Defense]
      H -->|Owner Approves via Mobile 375px UI| I[Submit Evidence via Payment Processor API]
  ```

  ### Mobile UX Flow (375px Viewport)
  1. **Notification:** The owner receives a push notification and sees an urgent card in their OHC Work Feed: "Chargeback Dispute Received - $120.00. Action required within 48h."
  2. **Review Screen:** Tapping the card opens a unified view showing the drafted evidence package. The screen is mobile-optimized (large typography, clear sections). It displays the customer's purchase history, the specific transaction, tracked shipping/delivery proof, and a summary of any past communications.
  3. **Action:** A translucent floating action bar (macOS Glass style) presents a clear primary CTA (≥44x44px target): "Submit Defense" and a secondary option "Accept Dispute (Refund)".
  4. **Confirmation:** If submitted, the app shows an optimistic success state and moves the item to a "Pending Resolution" tab in the finance dashboard.

  ### AI Agent Integration Points
  - **Finance Agent (The Accountant):** Listens to `tenant.dispute.created` events, orchestrates the evidence gathering, and interfaces with the Stripe API for evidence submission.
  - **Customer Success Agent (The Ambassador):** Provides the chat and email logs from the `Omnichannel Customer Memory Graph`.
  - **Operations Agent (The Manager):** Verifies the fulfillment status, tracking numbers, or booking attendance (for service personas like Leo or Carlos).

  ### Data Model & Security Invariants
  - **Dispute Table:** `chargeback_disputes` linked to `tenant_id`, `order_id`, and `customer_id` with Row-Level Security (RLS) enabled.
  - **Multi-Tenant Isolation:** Evidence documents and API calls must strictly enforce SPIFFE/SPIRE identity and tenant context bounds.

  ## Implementation Prompt
  **User-Facing Outcome:** The owner gets a pre-packaged dispute defense ready for a 1-tap submission directly from their phone, bypassing the need to manually gather receipts and logs.
  **CUJ:**
  1. A `chargeback.created` webhook is received from the payment gateway.
  2. The Finance Agent triggers a background job to collect order, shipping, and communication data.
  3. The agent formats this into Stripe's required evidence payload format.
  4. The owner reviews the compiled evidence in the mobile feed and taps "Submit Defense".
  5. The defense is transmitted via API and the UI updates to reflect the submitted status.

  **Acceptance Criteria:**
  - `chargeback_disputes` schema with RLS and tenant isolation.
  - Webhook endpoint for dispute ingestion with idempotency handling.
  - Finance Agent workflow for drafting evidence.
  - Playwright E2E test verifying a mock webhook ingestion, evidence draft generation, and successful owner submission via the UI.
  - 100% Mobile UI parity (375px) with translucent glass styling and accessible touch targets.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
