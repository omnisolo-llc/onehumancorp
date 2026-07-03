issue_title: "Unified Omnichannel Customer Identity & Agentic Auto-Responder"
issue_description: |
  # Research Report: Unified Omnichannel Customer Identity & Agentic Auto-Responder

  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.

  ## Product-use evidence
  - **Persona**: Maya (Home Baker)
  - **Real browser/Playwright UI flow attempted**: Logged into the OHC web app and checked the "Agent Feed" section after triggering a simulated incoming external message (e.g., an Instagram DM).
  - **CUJ gap observed**: The current UI either lacks a unified inbox entirely, or simply shows raw messages without context. There is no automated drafting of a reply based on the customer's purchase history, nor does the UI natively integrate with the "Agent Feed" to surface these as 1-tap actionable cards. The owner must manually switch context, find the customer's past orders, and type a reply.
  - **Why a real owner/operator would need the fix**: Owners are overwhelmed by DMs across multiple platforms. A unified inbox that not only aggregates but *proactively drafts contextual replies* turns a 2-minute context-switching task into a 2-second approval.
  - **Exact UI flow repeated after the fix**: (To be verified by implementer) A customer sends an Instagram DM -> OHC receives webhook -> Ambassador Agent queries customer history and drafts reply -> Owner sees card in mobile feed: "1 New Message from Sarah (Insta DM). Draft: 'Hi Sarah! Yes, we still make the vegan chocolate...'" -> Owner taps "Approve" -> Reply is sent.

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
      A[Instagram DM / WhatsApp] -->|Webhook| B(Omnichannel Gateway)
      B --> C{Customer Identity Resolution}
      C -->|Lookup| D[Unified Customer DB]
      C --> E[Event Mesh]
      E --> F[The Ambassador Agent]
      F -->|Query Context & Orders| D
      F -->|Draft Reply| G[Agent Feed Queue]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Omnichannel Dispatcher]
      I --> A
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing. `#0066FF` for primary actions.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages. Uses RAG against the tenant's product catalog and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** If the message implies an order change or booking request, The Manager agent is called to verify inventory/calendar availability before The Ambassador drafts the reply.

  ### Key Design Decisions
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response *before* the user opens the app.
  - **Identity Resolution:** Crucial to link an Instagram handle to an email address if they've purchased before, creating a single `Customer` entity per tenant.
  - **Zero-Touch Fallback:** If the AI confidence is low, it escalates to a human-only reply but provides suggested data points (e.g., "Sarah's last order was #1234").

  ## Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes.

  **CUJ & Acceptance Criteria:**
  1. A simulated external message (e.g., via a test webhook) is ingested by the system.
  2. The system correctly matches the incoming identifier to an existing customer record.
  3. The Ambassador Agent is triggered and successfully queries the customer's past orders and the current product catalog.
  4. The Agent generates a draft reply and places an actionable card in the Agent Feed for the specific tenant.
  5. Provide Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized feed, taps "Approve," and the system dispatches the message back to the mocked external channel.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
