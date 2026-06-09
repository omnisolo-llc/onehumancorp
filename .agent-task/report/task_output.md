issue_title: "Implement Omnichannel Agentic Inbox & Customer Identity Graph"
issue_description: |
  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context and require manual responses, lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic auto-replies. It does not proactively draft contextual responses based on full customer history.
  - **Wix Inbox:** Good aggregation, but AI features are limited to generic tone improvements, not autonomous customer success actions.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs) and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

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
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows customer context. Bottom half shows the AI-drafted reply.
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards with `background: rgba(255, 255, 255, 0.65)`, `backdrop-filter: blur(30px) saturate(210%)`, and `border-radius: 16px`. Native keyboard integration for editing.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages. Uses RAG against the tenant's product catalog and customer history to draft personalized replies.
  - **Operations Agent (The Manager):** Verifies inventory/calendar availability if the message implies an order change before The Ambassador drafts the reply.

  ### Key Design Decisions
  - **Proactive Drafting:** The AI drafts the response before the user opens the app.
  - **Identity Resolution:** Links identifiers (social handle, email) to a single `Customer` entity per tenant.
  - **Zero-Touch Fallback:** Escalates to a human-only reply with suggested data points if AI confidence is low.

  ## Implementation Prompt
  **Feature:** Omnichannel Agentic Inbox & Customer Identity Graph
  **Target Persona:** Maya the Baker / Carlos the Handyman
  **User-Facing Outcome:** When a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes.
  **CUJ & Acceptance Criteria:**
  1. A simulated external message is ingested by the Omnichannel Gateway.
  2. The Customer Identity Resolution Engine correctly matches the incoming identifier to an existing customer record.
  3. The Ambassador Agent queries past orders and current product catalog to generate a draft reply.
  4. Provide Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized feed, taps "Approve," and the system dispatches the message back to the mocked external channel.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []