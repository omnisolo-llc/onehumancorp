issue_title: "[Research] AI Unified Inbox Differentiation & Omnichannel Customer Memory"
issue_description: |
  ## Title
  AI Unified Inbox Differentiation & Omnichannel Customer Memory

  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history across all channels.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **Zendesk/Intercom:** Enterprise-grade and far too complex/expensive for a single-person SMB.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

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
  ```

  ### Mobile UX Flow
  - **Screen 1 (Feed):** Owner opens app to see top card "Reply Drafted: Carlos via WhatsApp."
  - **Screen 2 (Context):** Tapping card shows draft: "Hi Carlos! We can fit you in for the sink repair next Tuesday at 2 PM. It’s $150. Want me to book it?" Context chips below indicate: "Returning Customer (3 past jobs), mentioned leak."
  - **Screen 3 (Action):** Owner taps "Approve." Agent sends the message via WhatsApp and updates the CRM.

  ### AI Agent Integration Points
  - **Omnichannel Listener:** Subscribes to new message events.
  - **Identity Resolver:** Maps handles/numbers to OHC Customer IDs.
  - **Context Builder:** Gathers past orders, active tickets, and previous chats.
  - **Drafting Prompt:** Instructs the LLM to write a concise, brand-aligned response using the context.

  ## Implementation Prompt
  Implement the backend foundational services for the Omnichannel Unified Inbox. Specifically:
  1. Create a webhook ingress service capable of receiving normalized payloads from multiple channels (mock Instagram, WhatsApp, Email).
  2. Implement an Identity Resolution step that attempts to match incoming sender identifiers (phone, email, social handle) to an existing OHC Customer record.
  3. Create an Event Mesh publisher that emits an `InboundMessage` event upon successful ingestion and resolution.
  4. Ensure all new models support row-level tenant isolation (tenant_id).
  5. The acceptance criteria include 100% unit test coverage for the resolution logic and a Playwright E2E test simulating a message arriving and triggering the event. Do NOT prescribe the exact database schema or API shape.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
