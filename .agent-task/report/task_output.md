issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.
  We must retire the external Chatwoot dependency entirely and build our own multi-tenant omnichannel chat system natively in Rust, achieving 100% feature parity with Chatwoot's omnichannel models (Inbox, Channel, Contact, Conversation, Message) and WebSocket event streams.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history across all channels.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies.
  - **Chatwoot (Current External Dep, to be retired):** Excellent omnichannel abstractions (Inbox, Channel, Contact, Conversation, Message), robust webhooks, and WebSocket integration, but relying on an external service limits our tight AI integrations, Zero-Trust multi-tenancy, and UI consistency. It adds operational overhead and breaks our single-binary/mono-repo architecture.
  - **OHC Opportunity:** Implement the Chatwoot architecture internally in Rust. Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Rust Omnichannel Gateway)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup| F[(PostgreSQL: Unified Customer Graph)]
      E --> G[NATS Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Rust Omnichannel Dispatcher]
      K --> A/C/D
      B --> L[Rust WebSocket Server]
      L --> J
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Premium macOS-style Translucent Glass materials, blurred background to maintain focus, native keyboard integration if editing.

  ### Data Model (Rust/PostgreSQL) - based on Chatwoot concepts
  Strict multi-tenant isolation via `tenant_id` on every table and PostgreSQL Row Level Security (RLS).
  - `ohc_inbox`: Represents a unified inbox for a tenant.
  - `ohc_channel`: Adapters for specific platforms (Instagram, WhatsApp, Email, Web Widget).
  - `ohc_contact`: A unified customer identity across channels.
  - `ohc_conversation`: A thread between a contact and the tenant via a specific channel/inbox.
  - `ohc_message`: Individual messages within a conversation (including AI drafts vs. sent messages).

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh. Uses RAG (Retrieval-Augmented Generation) against the tenant's product catalog and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** If the message implies an order change or booking request, The Manager agent is called to verify inventory/calendar availability before The Ambassador drafts the reply.

  ### Key Design Decisions
  - **Native Rust Implementation:** Retire the Chatwoot external dependency completely. Implement the core models (Inbox, Contact, Conversation, Message) in Rust using SQLx and PostgreSQL.
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response *before* the user opens the app.
  - **Identity Resolution:** Crucial to link an Instagram handle to an email address if they've purchased before, creating a single `Contact` entity per tenant.
  - **Zero-Touch Fallback:** If the AI confidence is low, it escalates to a human-only reply but provides suggested data points.

  # Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes. The system is entirely native, blazing fast, and requires no third-party inbox setup.
  **CUJ & Acceptance Criteria:**
  1. Implement the foundational Rust data models and database migrations for `ohc_inbox`, `ohc_channel`, `ohc_contact`, `ohc_conversation`, and `ohc_message` ensuring strict multi-tenant isolation (`tenant_id`).
  2. Implement the Omnichannel Gateway to ingest a simulated external message (e.g., via a test webhook).
  3. The Customer Identity Resolution Engine correctly matches the incoming identifier to an existing `ohc_contact` record.
  4. The Ambassador Agent is triggered and successfully queries the customer's past orders and the current product catalog.
  5. The Agent generates a draft reply (a `ohc_message` with a draft status) and places it in the Action Required Queue.
  6. Provide Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized feed, taps "Approve," and the system dispatches the message back to the mocked external channel.
  7. All tests (`bazel test //...`) MUST pass. Test coverage MUST be 100%.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, omnichannel, chatwoot-migration]
assignees: []
