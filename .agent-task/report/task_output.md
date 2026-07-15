issue_title: "Feature Request: Native Agentic Quoting & Invoicing System"
issue_description: |
  ## 1. Problem Statement
  Service-based and field-service small business owners (e.g., Carlos the Handyman, Nora the Agency Principal) operate primarily from their mobile devices. They currently face high friction when converting a customer inquiry (like an Instagram DM or a phone call) into a formal estimate, and later, an invoice. Existing platforms (Shopify, Wix) treat services as static products, requiring clunky third-party apps to generate dynamic quotes. This forces owners into fragmented workflows: using a CRM for the conversation, a separate app for the quote, and Stripe or PayPal manually for the invoice, with zero proactive AI assistance to follow up on unpaid quotes or invoices.

  ## 2. Research Report
  - **Market Context**: Our research (Track 1) shows that legacy builders focus on catalog-based e-commerce. AI-native tools (Durable, Framer) focus on website generation but lack deep operational backends. There is a massive gap in handling the "service-to-cash" pipeline for field operators.
  - **The OHC Opportunity**: OHC can differentiate by offering a unified "Quote-to-Cash" pipeline managed by the Sales & Finance Agent. When an owner receives a request, the AI can draft a quote based on historical pricing, push it for 1-tap mobile approval, and autonomously follow up if the customer hasn't responded.
  - **Competitor Gaps**:
    - *Shopify*: Excellent for physical products; terrible for dynamic, labor-based quoting without expensive apps like "Sufio" or "Quote Builder".
    - *Invoice2go / Jobber*: Great vertical solutions, but completely disconnected from the owner's primary website, marketing tools, and unified inbox.
    - *Stripe Invoicing*: Powerful backend, but lacks the conversational AI front-end to draft the quote based on a chat message.

  ## 3. Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Inbox as OHC Unified Inbox
      participant SalesAgent as Sales & Finance Agent
      participant OHCDB as OHC PostgreSQL
      participant Stripe as Stripe API
      participant Owner as Mobile Owner (Carlos)

      Customer->>Inbox: "How much to fix a leaky pipe?"
      Inbox->>SalesAgent: Trigger Intent Classification
      SalesAgent->>OHCDB: Query historical pricing for "leaky pipe"
      SalesAgent->>Inbox: Drafts Quote & Message
      SalesAgent->>Owner: Push Notification: "Quote drafted for approval"
      Owner->>SalesAgent: 1-Tap Approve (Mobile UI)
      SalesAgent->>Customer: Sends PDF Quote + Payment Link
      SalesAgent->>Stripe: Creates Stripe Payment Link/Intent
  ```

  ### Data Model (PostgreSQL)
  - `Quote`: The drafted estimate (linked to Customer, LineItems, Status: Draft/Sent/Approved/Rejected).
  - `Invoice`: The final bill (linked to Quote, Stripe Payment Intent, Status: Unpaid/Paid/Overdue).
  - `LineItem`: Specific service/product charges on the quote or invoice.

  ### AI Integration Points
  - **Sales & Revenue Assistant**: Triggers on specific inbox intents (e.g., "pricing", "estimate"). Uses RAG against past jobs to estimate cost. Drafts the message and the structured quote data simultaneously.
  - **Finance Assistant**: Runs a daily CRON job to check for unpaid invoices or unapproved quotes over 48 hours old. Drafts follow-up messages for owner approval.

  ### Mobile UX Flow (375px)
  1. **Triage Feed**: Carlos opens the app and sees an Action Card: "New request for Pipe Repair. Quote drafted."
  2. **Quote Review**: Tapping the card opens a full-screen, mobile-optimized view of the quote. The items, hours, and total are clearly visible with large text.
  3. **Edit/Approve**: Large touch targets (44x44px minimum) for "Edit Line Items" or "Approve & Send".
  4. **Native Keyboard**: If editing, the number pad automatically opens for price adjustments.
  5. **Offline Support**: The approval action is queued locally if the network is flaky, ensuring Carlos doesn't lose his work while in a customer's basement.

  ## 4. Implementation Prompt
  **Feature Name**: Native Agentic Quoting & Invoicing Pipeline
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos receives an inquiry, reviews an AI-drafted quote on his Android phone, taps "Approve", and the system handles sending the estimate and the subsequent invoice/payment link via Stripe.

  **Critical User Journey (CUJ) & Acceptance Criteria**:
  1. Owner logs into the OHC mobile web app (375px view).
  2. Owner navigates to the Inbox/Triage feed and sees a message from a customer asking for an estimate.
  3. The Sales Agent has proactively drafted a `Quote` based on the conversation text. The UI displays an Action Card summarizing the quote total.
  4. Owner taps the card, reviews the line items, and taps "Approve & Send".
  5. The system must transition the `Quote` to 'Sent' and generate a valid Stripe Checkout/Payment Link.
  6. The UI must render correctly without horizontal scrolling on a 375px viewport.
  7. Automated Playwright tests MUST verify the mobile layout, the agent drafting simulation (using the AI judge helper), and the successful transition of the quote state.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []