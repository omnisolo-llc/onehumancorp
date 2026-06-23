issue_title: "Implement Agentic Dispute & Chargeback Defense System"
issue_description: |
  ## Title
  Implement Agentic Dispute & Chargeback Defense System

  ## Problem Statement
  Small business owners (like Priya the boutique owner or Maya the baker) operate with thin margins. A single $200 chargeback can wipe out days of profit. Responding to Stripe chargebacks requires logging into a complex dashboard, reading bank rules, and manually hunting down order details, delivery tracking, and customer DMs to build an evidence packet. Most SMBs simply give up and accept the loss due to the time and technical friction involved.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify:** Provides a basic interface to upload evidence, but the merchant still has to manually write the explanation and gather external context (like Instagram DMs or email agreements).
  - **Wix / Squarespace:** Similar passive forms that pass data to Stripe. No intelligent assistance.
  - **Stripe Dashboard:** Comprehensive but designed for finance professionals, using terms like "Compelling Evidence" and "Reason Codes" which confuse non-technical users.
  - **OHC Opportunity:** OHC inherently holds the "Unified Memory" of the business. The platform knows when the quote was approved, what messages were exchanged with the customer, and when the booking or delivery was completed. The Finance/Operations Agent can autonomously compile these disparate pieces of data into a structured evidence packet formatted perfectly for the bank, requiring only a one-tap approval from the owner.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Stripe Webhook: charge.dispute.created] --> B[OHC Payment Gateway]
      B --> C[Dispute Triage Queue]
      C --> D[The Accountant Agent]
      D -->|Query 1| E[Unified Customer Messages]
      D -->|Query 2| F[Order/Booking Delivery Proof]
      D -->|Query 3| G[Signed Quotes/Terms]
      D --> H[Draft Evidence Packet]
      H --> I[Owner Mobile Feed 375px]
      I -->|Review & 1-Tap Submit| J[Stripe Dispute API]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Alert Card:** A high-priority red/orange card appears in the Agent Feed: "🚨 Urgent: $200 Chargeback from Sarah. Action needed by Friday."
  - **Detail View:** Tapping the card opens a translucent glass panel detailing the dispute reason (translated from bank-speak to plain English, e.g., "Sarah claims she didn't authorize this payment.").
  - **AI Evidence Packet:** Below the explanation, the Agent presents its drafted defense:
    - *Auto-attached:* Sarah's Instagram DM saying "Looks great, charge my card."
    - *Auto-attached:* Delivery completion photo or booking check-in time.
    - *Auto-generated Letter:* A professionally written bank response summarizing the evidence.
  - **Call to Action:** Two primary buttons: "Submit Evidence to Bank" or "Accept Chargeback (Refund)".

  ### AI Agent Integration Points
  - **The Accountant (Finance):** Listens for dispute events, categorizes the bank's reason code, and calls upon The Ambassador to pull the communication history.
  - **The Ambassador (Customer Success):** Searches the omnichannel inbox for any relevant context from that specific customer ID.
  - **LLM Evidence Formatting:** The LLM takes the raw JSON data and formats it into a compelling, clear narrative for the bank reviewer.

  ### Key Design Decisions
  - **No Bank Jargon:** Abstract away Stripe's specific `evidence` object keys into plain-language summaries for the owner.
  - **Zero Trust/Security:** Ensure that the agent only pulls data strictly tied to the customer ID associated with the disputed payment.
  - **Proactive Not Reactive:** The system drafts the response *before* the owner even opens the notification, saving time.

  ## Implementation Prompt
  **Target Persona:** Priya (Boutique Operator)
  **Outcome:** When a dispute webhook arrives, the system autonomously queries the customer's history, drafts a comprehensive evidence submission, and presents it to the owner in their feed for a one-click submission back to Stripe.

  **Critical User Journey (CUJ):**
  1. The backend receives a `charge.dispute.created` webhook.
  2. The system triggers a background job for The Accountant Agent.
  3. The Agent queries the unified memory (messages, quotes, orders) for the associated customer.
  4. The Agent drafts an evidence package.
  5. Priya opens the OHC mobile app and sees a high-priority card: "Dispute from John. I've prepared our defense showing his text message approving the charge."
  6. Priya taps "Submit Defense". The system calls the payment provider API to submit the evidence.

  **Acceptance Criteria:**
  - Add webhook handling for dispute creation.
  - Create a new background agent task to compile dispute evidence from cross-domain sources (messages, orders).
  - Implement a mobile-first UI card to display the dispute and drafted evidence.
  - Integrate with the payment provider's dispute evidence submission API.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
