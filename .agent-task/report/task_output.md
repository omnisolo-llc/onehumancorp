issue_title: "Implement The Ambassador Agent (Customer Success Auto-Responder)"
issue_description: |
  # The Ambassador Agent (Customer Success Auto-Responder)

  ## Problem Statement
  Solopreneurs like Maya (the baker) miss critical sales because they cannot monitor social media DMs (Instagram/WhatsApp) while running physical operations like baking or deliveries. Existing solutions require complex logic builders (e.g., ManyChat) or "app taxes" on platforms like Shopify, which are too technical or fragmented for the OHC target audience. OHC needs a seamless, zero-config agent that proactively drafts replies to customer inquiries based on the business's actual context (inventory, policies, prior orders) and pushes them to the owner for a 1-tap approval.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **Zendesk/Intercom:** Enterprise-grade and far too complex/expensive for a single-person SMB.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs) and real-time inventory, and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM / External Webhook] -->|Webhook Event| B(Omnichannel Gateway)
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup / Create| F[(Unified Customer Graph DB)]
      E --> G[Event Mesh / Message Bus]
      G --> H[The Ambassador Agent]
      H -->|Query Context (RAG/Catalog)| F
      H -->|Draft Reply| I[(Action Required Queue)]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
      K --> A
  ```

  ### Mobile UX Flow (375px First)
  1. **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  2. **Interaction:** Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  3. **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  4. **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh. Uses RAG (Retrieval-Augmented Generation) against the tenant's product catalog and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** If the message implies an order change or booking request, The Manager agent is called to verify inventory/calendar availability before The Ambassador drafts the reply.

  ### Key Design Decisions
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response *before* the user opens the app.
  - **Identity Resolution:** Link social handles to email addresses to create a single `Customer` entity per tenant.
  - **Zero-Touch Fallback:** If AI confidence is low, escalate to a human-only reply but provide suggested data points.

  ## Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes.

  **CUJ & Acceptance Criteria:**
  1. Provide an endpoint/webhook for a simulated external message.
  2. The system correctly identifies or creates a customer record for the tenant.
  3. "The Ambassador" Agent is triggered, pulls relevant context (catalog, past orders), and drafts a reply.
  4. The draft is placed in the Action Required Queue for the tenant.
  5. Provide Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized feed, taps "Approve", and the system dispatches the message back.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
