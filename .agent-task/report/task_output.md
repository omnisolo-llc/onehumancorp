issue_title: "[Architect] Implement Native Rust Omnichannel Chat System"
issue_description: |
  # Problem Statement

  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for an owner-operator.
  Chatwoot as an external third-party service, dependency, or integration is 100% RETIRED. OHC does NOT rely on external Chatwoot services. We need to build our own.

  # Research Report

  **Findings & Competitive Analysis:**

  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history across all channels.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **Zendesk/Intercom:** Enterprise-grade and far too complex/expensive for a single-person SMB.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.
  - **Native Implementation:** Building our own native Rust Chat system inspired by Chatwoot ensures performance, data privacy (multi-tenant RLS), and seamless integration with our Ambassador AI without third-party bottlenecks.

  # Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway - Axum)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup| F[Unified Customer Graph DB - Postgres RLS]
      E --> G[Event Mesh / MsgBus]
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue - Postgres SKIP LOCKED]
      I --> J[Mobile App Feed 375px - Flutter/Tauri]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
      K --> A/C/D
  ```

  ### Data Model (Native Chatwoot Replacements)
  - `channels`: Configurations for Instagram, WhatsApp, Email, Web Widget.
  - `inboxes`: Connects a channel to a tenant.
  - `contacts`: The resolved omnichannel customer identity.
  - `conversations`: A thread of messages between a contact and an inbox.
  - `messages`: Individual messages within a conversation.
  - *All tables must have `tenant_id` and Row Level Security enabled.*

  ### Mobile UX Flow (375px First)

  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing. (macOS Translucent Glass styling, UniFi modular dashboard layout).

  ### AI Agent Integration Points

  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh. Uses RAG against the tenant's product catalog and the customer's specific history to draft highly personalized replies.

  ### Key Design Decisions

  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response _before_ the user opens the app.
  - **Identity Resolution:** Crucial to link an Instagram handle to an email address if they've purchased before, creating a single `Customer` entity per tenant.
  - **Native Rust:** Replace all Chatwoot dependencies with native high-performance Rust axum handlers, leveraging our existing Postgres/MsgBus infrastructure.

  # Implementation Prompt

  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes.

  **CUJ & Acceptance Criteria:**
  1. Implement the database schema for the native omnichannel chat (inboxes, conversations, messages, contacts) with RLS in PostgreSQL.
  2. Implement the API endpoints (Axum) to receive webhooks from channels (mocked for this task) and store messages.
  3. Implement the backend logic to trigger The Ambassador agent on new messages, query context, and draft a reply into the Action Required queue.
  4. Build the mobile-first UI components (Tauri/React) for the Home Feed card and the message approval view using the Premium Token design system.
  5. Provide Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized feed, taps "Approve," and the system dispatches the message back to the mocked external channel.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
