issue_title: "Research: OmniChannel Dispute Auto-Resolution & Fraud Defender"
issue_description: |
  # Research Report: OmniChannel Dispute Auto-Resolution & Fraud Defender

  ## Problem Statement
  Small business owners face significant pain when dealing with chargebacks and payment disputes. Traditional platforms (like Shopify or the Stripe Dashboard) notify the user of a dispute but require manual gathering of evidence (receipts, delivery logs, communication history) and manual submission. This is time-consuming and often results in lost revenue for non-technical owners (like Maya the home baker) who miss deadlines or struggle to compile complete evidence across various channels.

  ## Research Report
  Our competitive analysis reveals a major gap in the market for micro-SMEs:
  - **Stripe / Square:** Provide APIs and dashboards to manage disputes, but the burden of proof compilation rests entirely on the merchant.
  - **Shopify / BigCommerce:** Provide basic order timelines, but do not integrate omnichannel chat logs (e.g., Instagram DMs, WhatsApp) into the dispute evidence natively.
  - **Chargehound / Midigator:** Expensive enterprise solutions that automate chargebacks, which are inaccessible to small businesses due to high fixed costs and complex integrations.
  - **The OHC Opportunity:** By leveraging our existing Unified Inbox, Order Ledger, and LLM-powered agents, OHC can automatically compile shipping evidence, customer communications, and transaction details into a Stripe-ready dispute response, saving the owner time and recovering lost revenue.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Stripe as Payment Gateway (Stripe)
      participant OHC_Webhooks as OHC Webhook Handler
      participant FinanceAgent as Finance Agent
      participant LegalAgent as Legal/Compliance Agent
      participant OHC_DB as PostgreSQL Ledger & Inbox
      participant Owner as OHC Owner App (Mobile)

      Stripe->>OHC_Webhooks: POST charge.dispute.created
      OHC_Webhooks->>FinanceAgent: Trigger Dispute Flow
      FinanceAgent->>OHC_DB: Fetch Transaction & Order ID
      FinanceAgent->>LegalAgent: Request Evidence Compilation
      LegalAgent->>OHC_DB: Query Order Status, Shipping Logs, Omnichannel Chat
      LegalAgent->>LegalAgent: Generate AI Dispute Response Letter
      LegalAgent->>Owner: Push Notification: "New Dispute. Evidence ready for review."
      Owner->>Owner: Reviews Evidence on 375px Mobile App
      Owner->>FinanceAgent: Taps "Approve & Submit"
      FinanceAgent->>Stripe: POST /v1/disputes/{id} (Evidence Payload)
      FinanceAgent->>Owner: Status Update: "Dispute Submitted Successfully"
  ```

  ### AI Agent Integration Points
  - **Finance Agent ("The Accountant"):** Listens for dispute webhooks, tracks dispute deadlines, and handles the API submission to the payment gateway.
  - **Legal/Compliance Agent:** Responsible for the LLM prompt that takes raw OHC data (chat logs, tracking numbers, invoices) and formats it into a compelling, professional response letter tailored to the specific dispute reason code (e.g., `product_not_received`, `fraudulent`).

  ### Mobile UX Flow (375px First)
  1. **Push Notification:** Maya receives an alert on her iPhone: "Dispute opened for $45. We've drafted a response. Tap to review."
  2. **Work Triage Feed (Home Screen):** A high-priority card appears at the top of the feed: "Urgent: Resolve Dispute by Friday".
  3. **Dispute Review Screen:**
     - **Header:** Amount ($45) and Customer Name.
     - **Summary:** AI-generated one-liner ("We have Instagram proof they picked up the cake").
     - **Evidence Section:** A translucent glass card showing the auto-compiled timeline:
       - *Oct 12:* Invoice Paid.
       - *Oct 14:* Customer DM: "Loved the cake!" (Pulled from Unified Inbox).
     - **Action:** A sticky, full-width primary button at the bottom: "Submit Evidence".

  ### Key Design Decisions
  - **No Auto-Submit:** While we auto-compile the evidence, we intentionally require the owner to tap "Submit" to maintain Trust and control.
  - **Data Minimization:** The AI only selects chat logs relevant to the specific transaction, avoiding privacy leakage of unrelated conversations.
  - **Unified Webhook Handling:** Dispute processing must hook into our existing robust idempotency layer to handle Stripe retries safely.

  ## Implementation Prompt
  **User-Facing Outcome:** When a customer disputes a payment, OHC automatically gathers all chat history, order details, and delivery proof, and drafts a compelling response for the owner to approve with one tap on their phone.

  **Critical User Journey (CUJ):**
  1. A webhook triggers a dispute notification for a recent order.
  2. The Legal Agent automatically compiles a timeline showing the customer's communication confirming receipt, the finalized invoice, and the pickup confirmation.
  3. The owner opens the OHC app, sees the Dispute card in their Work Triage feed, reviews the AI-generated evidence, and taps "Submit Evidence".
  4. The backend calls the Stripe API to attach the evidence and submit the response, updating the UI to "Submitted".

  **Acceptance Criteria:**
  - Database schema handles dispute records and links them to specific tenants and transactions.
  - Webhook handler correctly processes Stripe dispute events with idempotency.
  - Service layer aggregates data from the Order and Message domains.
  - The Legal Agent successfully formats this data into a valid evidence payload.
  - A mobile-responsive (375px) endpoint provides the evidence review UI.
  - E2E Playwright tests verify the dispute webhook receipt and the owner's UI approval flow.

  ## Priority
  P2

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
