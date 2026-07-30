issue_title: "Native Rust Chatwoot Replacement System Architecture"
issue_description: |
  # Omni-channel Native Chatwoot Replacement System Design

  ## Problem Statement
  OHC currently relies on external Chatwoot services for omnichannel customer support and chat, which violates the architectural constraint of relying on third-party services for core capabilities and the "Native Rust Chatwoot Replacement" mandate. We need a native Rust implementation of Chatwoot's core features (inboxes, channels, contacts, conversations, messages, agent routing, WebSocket real-time messaging) that guarantees multi-tenant isolation, high performance, and deep integration with OHC's AI agents.

  ## Design

  ### Architecture Diagram
  ```mermaid
  graph TD;
      Client[Web/Mobile App] -->|HTTPS/WSS| API_Gateway[API Gateway];
      API_Gateway --> Auth[Auth Service];
      API_Gateway --> WSS[WebSocket Server];
      API_Gateway --> InboxService[Native Inbox/Chat Service];

      WSS --> PubSub[Redis PubSub];
      InboxService --> PubSub;

      InboxService --> DB[(PostgreSQL + pgvector)];

      InboxService --> Channels[Channel Adapters];
      Channels -->|Webhooks| Email[Email API];
      Channels -->|Webhooks| FB[Facebook/IG API];
      Channels -->|Webhooks| WA[WhatsApp API];
      Channels -->|Webhooks| SMS[Twilio SMS];

      InboxService --> AgentQueue[Agent AI Queue];
      AgentQueue --> OHC_Agents[OHC AI Agents];
  ```

  ### Data Model (Chatwoot Parity)

  - **chat_inboxes**: Groups channels together (e.g., "Support", "Sales").
  - **chat_channels**: The specific integration (e.g., `channel_email`, `channel_whatsapp`, `channel_web_widget`). Polymorphic config storage.
  - **chat_contacts**: The end user communicating with the business.
  - **chat_contact_inboxes**: Links a contact to a specific inbox with a channel-specific `source_id`.
  - **chat_conversations**: A thread of messages between a contact and the business in a specific inbox.
  - **chat_messages**: Individual messages within a conversation.
  - **chat_canned_responses**: Pre-defined responses for agents.

  *Strict Multi-tenant RLS applies to all tables using `tenant_id`.*

  ### Mobile UX Flow (375px)
  1. **Inbox List**: Bottom tab "Inbox". Shows unified list of conversations across all channels. Unread indicators.
  2. **Conversation View**: Clean chat interface. Messages grouped by date. Clear distinction between customer messages (left) and agent/bot messages (right).
  3. **Action Bar**: Text input, attachment button, and a "magic spark" button to trigger AI suggested replies.
  4. **Contact Info Sheet**: Tapping the customer name opens a bottom sheet with contact details, past orders, and AI-summarized sentiment.

  ### AI Agent Integration
  - **Triage Agent**: Listens to new `chat_messages` via job queue. Analyzes intent, tags the conversation, and assigns priority.
  - **Reply Drafter Agent**: Can be invoked manually by the owner or automatically for common questions. Drafts a reply in `chat_messages` as `sender_type: 'bot'` with a pending status for owner review, or auto-sends if confidence is high.
  - **Knowledge Agent**: Uses RAG against help articles to provide the Reply Drafter with accurate information.

  ## Implementation Prompt
  Implement the backend foundation for the Native Chatwoot Replacement.
  1. Define the SQL schema migrations for `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_contact_inboxes`, `chat_conversations`, and `chat_messages`, including strict RLS policies.
  2. Create the Rust data models in `src/server/services/chat/models.rs`.
  3. Implement the CRUD service layer in `src/server/services/chat/service.rs` to handle these entities.
  4. Ensure 100% unit test coverage for the service layer.
  5. Create Playwright E2E tests for the basic Inbox UI flow to ensure the API and UI are wired correctly.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
