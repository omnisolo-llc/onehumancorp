issue_title: "Implement Omnichannel Identity Resolution and Unified AI Inbox for AgentFeed"
issue_description: |
  ## Title
  Implement Omnichannel Identity Resolution and Unified AI Inbox for AgentFeed

  ## Problem Statement
  Small business owners (such as Maya the Baker or Carlos the Handyman) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.

  Currently, the OHC `AgentFeed` and `AgentFeedCard` simply render pending action drafts from `/api/inbox/action_required` but there is no underlying architecture to resolve the omnichannel identity of the customer, nor the deep RAG context to properly draft these messages proactively.

  ## Research Report
  **Market Mapping & Competitor Discovery:**
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history across all channels.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **Zendesk/Intercom:** Enterprise-grade and far too complex/expensive for a single-person SMB.
  - **Durable / 10Web:** AI native website builders that focus on generation but lack an ongoing Customer Success Agent for post-launch operations.

  **OHC Opportunity:**
  Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

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
  - **Home Feed (Mobile):** The first screen is a vertical feed of Action Required cards. Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** The `AgentFeedCard` shows a summary. Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** A prominent primary button "Approve & Send" and a secondary "Edit Draft". Touch targets are at least 44x44px.
  - **Visual Design:** Use OHC Premium Tokens. Glassmorphism cards with translucent materials, blurred background to maintain focus, native keyboard integration if editing. No horizontal scroll.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh. Uses RAG (Retrieval-Augmented Generation) against the tenant's product catalog and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** If the message implies an order change or booking request, The Manager agent is called to verify inventory/calendar availability before The Ambassador drafts the reply.
  - **Distributed Locks:** Redis Redlock ensures that two agents do not process the same incoming webhook simultaneously.

  ### Key Design Decisions
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response *before* the user opens the app.
  - **Identity Resolution:** Crucial to link an Instagram handle to an email address if they've purchased before, creating a single `Customer` entity per tenant with isolated row-level security.
  - **Zero-Touch Fallback:** If the AI confidence is low, it escalates to a human-only reply but provides suggested data points (e.g., "Sarah's last order was #1234").

  ## Implementation Prompt
  **User-Facing Outcome:** As a business owner (e.g., Maya), when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted in my Agent Feed. I tap one button to send it, taking 2 seconds instead of 2 minutes.

  **CUJ & Acceptance Criteria:**
  1. Implement the `Omnichannel Gateway` in Go to ingest webhooks from simulated external channels.
  2. Implement the `Customer Identity Resolution Engine` to match incoming identifiers (e.g., social handle) to an existing customer record.
  3. Wire the `Ambassador Agent` to trigger on new messages, query the customer's past orders, and generate a draft reply in the `ActionRequiredQueue` (modifying the corresponding Go code layer handling `action_required_queue_repo`).
  4. Ensure `AgentFeedCard.tsx` beautifully displays this draft on a 375px viewport with a minimum 44x44px touch target for the "Approve & Send" button.
  5. Implement 100% unit test coverage for the backend Go logic.
  6. Implement at least FIVE Playwright E2E tests covering the full CUJ: A user logs in, sees the drafted message card on the mobile-sized feed, taps "Approve", and the system dispatches the message.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
