issue_title: "Autonomous Reputation & Review Recovery System"
issue_description: |
  # Mission Queue Protocol: Agent-Driven Customer Review & Reputation Recovery System

  ## Problem Statement
  Local service operators (e.g., Carlos the Handyman) and retail owners (e.g., Priya the Boutique Operator) rely heavily on Google, Yelp, and social media reviews for new customer acquisition. However, they frequently forget to ask for reviews after successfully completing a job or sale. Conversely, negative experiences often go unnoticed until a 1-star review is publicly posted, permanently damaging their local reputation. Existing tools (like Podium or Birdeye) are expensive, disconnected from the core POS/booking system, and require the owner to manage yet another dashboard. Small business owners need an invisible system that proactively builds their 5-star reputation and intercepts poor experiences before they become public.

  ## Research Report
  - **Market Context:** Reputation management platforms like Podium charge $250+/month just for SMS review requests. Shopify requires third-party apps (e.g., Yotpo, Loox) which add $30-$100/month and focus solely on product reviews, ignoring the service/local business aspect.
  - **The OHC Opportunity:** Since OHC natively handles the payment, booking, and omnichannel communication (Omni Payment Ledger, Service Bookings), we have the exact trigger moment (e.g., "Job Completed" or "Package Delivered"). We can use the Customer Success Agent ("The Ambassador") to handle the entire reputation lifecycle autonomously, saving the owner hundreds of dollars and manual effort.
  - **Competitor Gaps:**
    - *Shopify/Wix:* Passive review collection; requires customer to initiate.
    - *Podium:* Expensive, standalone, requires API integrations to know when a job is done.
    - *OHC Advantage:* Native integration. The AI agent can assess sentiment in real-time through text/WhatsApp and route happy customers to public review sites while routing unhappy customers directly to the owner's Agent Feed for remediation.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Order/Booking Completed] -->|State Change Event| B(Event Bus)
      B --> C[The Ambassador Agent]
      C -->|Wait 24h| D[Send SMS/WhatsApp Check-in]
      D --> E[Customer Replies]
      E --> F{LLM Sentiment Analysis}
      F -->|Positive| G[Send Google Review Link]
      F -->|Negative| H[Create Urgent Action Card]
      H --> I[Owner Agent Feed 375px]
      I -->|1-Tap Approve Remediation| J[Send Apology + Discount/Refund]
  ```

  ### Mobile UX Flow (375px)
  1. **Customer View:** Receives a friendly, native SMS/WhatsApp message: "Hi! Carlos here from OHC Repairs. How is the new sink working out?" If they reply "Great!", they receive a frictionless link to Google Reviews. If they reply "It's leaking a bit", the flow intercepts.
  2. **Owner View (Agent Feed):** Carlos opens his OHC app (375px). At the top of his feed is an urgent Action Card: "⚠️ Reputation Alert: John Smith is unhappy with the sink repair." The card includes a drafted response: "I'm so sorry John, I can swing by tomorrow at 10 AM to fix this for free." Carlos taps "Approve" and the crisis is averted.
  3. **Settings:** A simple toggle in Advanced Settings: "Auto-request reviews after successful jobs."

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Hooks into the `service_bookings` and `ledger` state changes. Triggers the timed follow-up. Performs NLP sentiment analysis on the customer's reply.
  - **The Manager (Operations Agent):** If a remediation requires a follow-up visit or replacement product, the Operations Agent automatically drafts a new zero-cost booking/order.

  ### Key Design Decisions
  - **Conversational Interception:** Do not send a direct "Click here to rate 1-5 stars" link immediately. Start with a human-like text check-in to intercept negative feedback privately.
  - **Omnichannel:** Use the customer's preferred channel (WhatsApp, SMS, or Instagram DM) based on their Identity Graph.
  - **Zero-Touch for the Owner:** The owner only intervenes for negative feedback (via the Agent Feed). Positive reviews happen while they sleep.

  ## Implementation Prompt
  **Feature Name:** OHC Autonomous Reputation & Review Recovery System

  **Target Persona:** Carlos the Handyman

  **Outcome:** Carlos automatically gets more 5-star Google Reviews from happy clients, while unhappy clients are intercepted via SMS and escalated to his Agent Feed before they can post publicly.

  **Acceptance Criteria / Critical User Journey (CUJ):**
  1. Implement a background job/trigger that listens for a "Booking Completed" or "Order Fulfilled" event in the database.
  2. After a configured delay (e.g., 24h), The Ambassador agent sends a natural language check-in message via the mocked Omnichannel integration.
  3. Create an endpoint to simulate the customer's reply.
  4. Integrate the LLM to classify the reply sentiment (Positive, Neutral, Negative).
  5. If Positive, auto-reply with a review link.
  6. If Negative, generate an Action Card in the Owner's Agent Feed (mobile UI) with a proposed drafted response (e.g., offering a fix or refund).
  7. The owner must be able to tap "Approve" on the Action Card to send the drafted response.
  8. All UI must be fully responsive and optimized for a 375px mobile viewport.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
