issue_title: "[sales] Autonomous Conversational Cart and Booking Recovery"
issue_description: |
  ## Title
  [sales] Autonomous Conversational Cart and Booking Recovery

  ## Problem Statement
  Small business owners (like Leo the Music Tutor or Priya the Boutique Owner) lose significant revenue when customers abandon their online shopping carts or service booking flows mid-way. Existing platforms (Shopify, Wix) attempt to solve this via static, delayed "Abandoned Cart" emails. These emails are easily ignored and do not address the underlying *reason* the customer left (e.g., price objection, scheduling confusion, or shipping cost). Non-technical owners do not have the time to personally follow up with every dropped lead to negotiate or answer questions. They need an invisible sales teammate that proactively reaches out, handles objections conversationally, and closes the sale dynamically.

  ## Research Report
  - **Competitive Audit**:
    - **Shopify / Wix**: Offer automated, rules-based abandoned cart emails. They are one-way broadcasts.
    - **Klaviyo / Omnisend**: More advanced segmentation, but still fundamentally static email/SMS workflows. Not conversational.
    - **OHC Advantage**: Leveraging the "Sales & Acquisition" department (The Salesperson) and the Unified Omni-channel Inbox. When an "Intent Session" (cart or booking) is abandoned, the OHC agent can initiate a two-way conversation via SMS or WhatsApp, acting as the business owner's assistant.
  - **Key Findings**:
    - Average cart abandonment rate in eCommerce is nearly 70%.
    - Personalized, conversational SMS follow-ups have a 4x higher conversion rate than standard abandoned cart emails.
    - Top reasons for abandonment include unexpected costs (shipping/taxes) or needing more information (especially for services). A conversational agent can offer dynamic discounts or answer FAQs to overcome these specific objections.

  ## Design Doc
  ### Data Model (Intent Sessions)
  We must introduce an `intent_session` tracking mechanism within the Universal Ledger that records a user's progress through a checkout or booking flow.

  ```mermaid
  erDiagram
      TENANT ||--o{ INTENT_SESSION : "owns"
      INTENT_SESSION ||--o{ INTENT_ITEM : "contains"
      INTENT_SESSION ||--o| CONVERSATION : "triggers"

      INTENT_SESSION {
          uuid id
          string tenant_id
          string customer_id
          string status "Active, Abandoned, Recovered"
          timestamp last_activity_at
      }

      INTENT_ITEM {
          uuid id
          string product_or_service_id
          int quantity
          json pricing_snapshot
      }
  ```

  ### Architecture Flow (The Salesperson)
  1. **Detection**: The KAIROS Orchestrator monitors `intent_session` records. If a session is inactive for 1 hour, it transitions to `Abandoned`.
  2. **Queueing**: A recovery job is dispatched to the Sub-Agent Orchestration Queue for "The Salesperson" agent.
  3. **Contextualization**: The agent reads the `intent_session` and the `Customer360` profile to draft a highly personalized message.
  4. **Outreach**: The message is sent via the preferred channel (SMS/WhatsApp) via the Unified Inbox.
  5. **Negotiation**: The customer replies with an objection (e.g., "Shipping was too high"). The agent, equipped with specific tools (e.g., `generate_discount_link(max_percent: 10)`), negotiates the sale.
  6. **Conversion**: The customer completes the transaction via the dynamically generated link.

  ### Mobile-First UX & Wireframes (375px First)
  - **Dashboard Card**: "Recovered Revenue" glassmorphic card showing $ value saved by the AI this week.
  - **Settings Toggle**: Deep inside the "Sales Agent" settings: "Enable Conversational Recovery" (On/Off) and a simple slider for "Maximum Allowed AI Discount" (0% - 20%). No complex workflow builders.

  ## Implementation Prompt
  **Goal**: Build the "Autonomous Conversational Cart & Booking Recovery" system.

  **Core User Journey (CUJ)**:
  1. **The Drop-off**: A customer adds a "$50 Guitar Lesson" to their cart on Leo's storefront but closes the browser at the payment step.
  2. **The Outreach**: One hour later, the KAIROS engine triggers The Salesperson agent. The agent texts the customer: "Hi there! I saw you were looking at booking a guitar lesson with Leo but didn't finish. Did you have any questions about his teaching style?"
  3. **The Objection**: The customer replies: "I'm just not sure if he teaches beginners."
  4. **The Close**: The agent checks Leo's profile, replies ("Yes! Leo specializes in absolute beginners."), and provides a direct link to complete the booking. The booking is secured.

  **Acceptance Criteria**:
  - Implement the `intent_session` tracking table with tenant isolation.
  - Create a KAIROS background worker that identifies abandoned sessions and enqueues recovery jobs.
  - Equip "The Salesperson" agent with a tool to generate dynamic discount checkout links within owner-defined limits.
  - Integrate the agent's outreach with the existing Unified Inbox so the owner can seamlessly take over the conversation if needed.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
