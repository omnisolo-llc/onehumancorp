issue_title: "Implement Proactive Omni-Channel Customer Agent (The Ambassador)"
issue_description: |
  ## Problem Statement
  Small business owners receive customer inquiries across multiple unlinked channels (e.g. Instagram DMs, WhatsApp, SMS, email). Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" merely aggregate messages without context, requiring owners to type manual responses and lacking purchase history or cross-channel interaction context. This reactive process doesn't scale for a solopreneur.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox:** Aggregates chat and email but relies on manual responses or basic auto-replies. It does not proactively draft contextual responses based on full omnichannel history.
  - **Wix Inbox:** Aggregation is good, but AI is limited to "improving tone" rather than acting as an autonomous customer success agent.
  - **Zendesk/Intercom:** Enterprise-grade, far too complex, and expensive for a single-person SMB.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Ambassador agent doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup| F[Unified Customer Graph DB]
      E --> G[Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
      K --> A/C/D
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** The top card in the Command Center / Unified Agent Feed shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card expands a unified view. The top half shows the customer context (e.g. "Sarah bought a vegan cake 2 months ago"). The bottom half shows the AI-drafted reply.
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit" button.
  - **Visual Design:** Glassmorphism cards with translucent backgrounds. Use native keyboard integration when editing.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh. Uses RAG against the tenant's product catalog and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** If the message implies an order change or booking request, The Manager agent is called to verify inventory/calendar availability before The Ambassador drafts the reply.

  ### Key Design Decisions
  - **Proactive Drafting:** Move from "read and reply" to "read and approve". The AI drafts the response before the user opens the app.
  - **Identity Resolution:** Crucial to link social handles to email addresses if they've purchased before, maintaining a single Customer entity per tenant.
  - **Zero-Touch Fallback:** If the AI confidence is low, it escalates to a human-only reply but provides suggested data points (e.g. "Sarah's last order was #1234").

  ## Implementation Prompt
  **User-Facing Outcome:** When a customer DMs the business owner on Instagram asking about a past order, the owner opens the OHC app to find a perfectly accurate, context-aware response already drafted in their Unified Agent Feed. The owner taps one button to approve and send it, saving time and mental load.

  **CUJ & Acceptance Criteria:**
  1. A simulated external message (e.g. via an API endpoint or test webhook) is ingested by the system.
  2. The backend matches the incoming identifier (e.g. email or handle) to an existing customer record, pulling their purchase/booking history.
  3. The Ambassador Agent is triggered and queries the customer's history.
  4. The Agent generates a draft reply and places it in the Action Required Queue (triage feed) for the tenant.
  5. Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized feed, taps "Approve", and the system records the message dispatch. Ensure the UI element handles interactions cleanly and respects the 375px design constraints.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
