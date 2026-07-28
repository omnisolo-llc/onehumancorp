issue_title: "[Architecture] Native Rust Omnichannel Inbox & Customer Success Engine"
issue_description: |
  # Problem Statement
  Small business owners (like Maya the baker and Carlos the handyman) receive customer inquiries across multiple fragmented channels: Instagram DMs, WhatsApp, SMS, web chat, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox, or integrating external tools like Chatwoot) simply aggregate messages without deep contextual awareness. They require the owner to manually type responses, often lacking the customer's purchase history, active bookings, or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur. Furthermore, relying on an external system like Chatwoot introduces multi-tenancy risk, latency, and fragments our "Teammate" AI context.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Audit:** Chatwoot relies on Ruby on Rails models for `Account`, `Inbox`, `Conversation`, and `Message`, with heavy reliance on Postgres, Redis, and WebSockets. While robust, it is an external dependency that duplicates our multi-tenancy and lacks native integration with OHC's Customer Identity Graph and AI Agent ecosystem. OHC needs a native Rust replacement that mimics the `Inbox` -> `Conversation` -> `Message` hierarchy but does so seamlessly within our single-tenant-isolated Postgres design.
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **OHC Opportunity:** Complete Chatwoot retirement. Build a native Rust omnichannel engine that feeds directly into OHC's "The Ambassador" AI. The engine doesn't just aggregate messages; it routes them to our Event Mesh where the AI queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs) and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Rust Gateway)
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
  - **Interaction:** Tapping the card opens a unified translucent glass view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing. No developer terms visible.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh. Uses RAG (Retrieval-Augmented Generation) against the tenant's product catalog and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** If the message implies an order change or booking request, The Manager agent is called to verify inventory/calendar availability before The Ambassador drafts the reply.

  ### Key Design Decisions
  - **100% Native Rust:** Retire Chatwoot entirely. The `ohc-mono` backend will natively handle channel integrations (WhatsApp, Instagram, Email) via Rust adapters, mapping into `ChatInbox`, `ChatConversation`, and `ChatMessage` models with strict tenant isolation.
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response *before* the user opens the app.
  - **Identity Resolution:** Link social handles to email addresses to create a single Customer entity per tenant.
  - **Zero-Touch Fallback:** If AI confidence is low, escalate to human-only reply but provide suggested data points (e.g., "Sarah's last order was #1234").

  # Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app on my 375px phone to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes. I do not have to log into a separate chat system.
  **CUJ & Acceptance Criteria:**
  1. Define the SQL schema and Rust data models (using `sqlx` and `uuid`) for `ChatInbox`, `ChatChannel`, `ChatConversation`, `ChatMessage`, and `ChatContact` that replicate essential omnichannel functionality with strict `tenant_id` isolation.
  2. A simulated external message (e.g., via a test webhook endpoint) is ingested by the new native Rust Omnichannel Gateway.
  3. The Customer Identity Resolution Engine correctly matches the incoming identifier to an existing customer record in the database.
  4. The Ambassador Agent is triggered via Event Mesh, queries the customer's past orders and the current product catalog, generates a draft reply, and places it in the `ActionRequiredQueue` for the specific tenant.
  5. Provide Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized feed, taps "Approve," and the system dispatches the message back to the mocked external channel.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []