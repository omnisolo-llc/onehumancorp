issue_title: "Scout: Native Rust Omnichannel Chat System Architecture"
issue_description: |
  **Title**: Native Rust Omnichannel Inbox & Customer Identity Resolution

  **Problem Statement**:
  Currently, OneHumanCorp (OHC) operations are fragmented across channels. For a non-technical owner/operator like Maya (who gets Instagram DMs) or Carlos (who gets text messages and web leads), managing these scattered customer touchpoints is chaotic and unscalable. Relying on an external third-party chat service (like Chatwoot) violates our Zero Trust, multi-tenant architectural standards and limits our AI work assistant capabilities. The product must seamlessly ingest omnichannel messages (Instagram, WhatsApp, SMS, web) into a native Rust inbox, resolve customer identities contextually, and enqueue these interactions for our AI Triager without depending on third-party SaaS for the inbox layer.

  **Research Report**:
  - **Competitor Analysis**: Leading platforms like Shopify (Inbox), Square (Messages), and WeChat/WeCom natively own their messaging rails to ensure the AI assistant has immediate context. Third-party integrations break the "One Assistant" promise.
  - **Chatwoot Source Code Audit**: Investigated Chatwoot's `app/models`, `app/controllers/api`, webhooks processing, and WebSocket architecture. Chatwoot uses separate models for `Inboxes`, `Channels`, `Contacts`, `Conversations`, and `Messages`. It uses ActionCable (WebSockets) for real-time delivery and delayed_job for background routing/SLA processing.
  - **Proposed Paradigm Shift**: Complete retirement of Chatwoot integration. We will replicate Chatwoot’s robust multi-channel data schema and webhook ingestion pipelines in high-performance native Rust microservices within `onehumancorp/mono`.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    graph TD
      A[Omnichannel Webhooks: IG, WhatsApp, SMS] -->|Ingest| B(Rust: Webhook Controller)
      B --> C{Identity Resolution & Invariants}
      C -->|Resolved| D[PostgreSQL: chat_contacts]
      C -->|Create/Update| E[PostgreSQL: chat_conversations & messages]
      E --> F[Redis: AI Job Queue - message_triage]
      E --> G[WebSockets: Real-time UI Updates]
      F --> H[AI Work Assistant Triager]
    ```
  - **Mobile UX Flow**: On a 375px viewport, the owner taps "Inbox" and sees a unified timeline of messages. The AI Triager automatically drafts a suggested response for unread messages, surfaced directly above the keyboard. No configuration required.
  - **AI Agent Integration**: As messages are saved to `chat_messages`, the `message_triage` job triggers. The AI Triager evaluates customer intent (e.g., pricing question, booking request) and drafts a response or coordinates with the Operations Assistant to pull inventory data.
  - **Security & Tenancy**: Strict row-level tenancy (`tenant_id`) enforced on all chat tables.

  **Implementation Prompt**:
  Implement the Native Rust Omnichannel Chat subsystem:
  1. Define the SQL schema and RLS policies for `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` using PostgreSQL.
  2. Build the Rust structs, repository models, and service layer (`omnichannel_service.rs`) for managing chat entities and resolving identities.
  3. Refactor the existing webhook ingestion endpoints to directly interface with our native Rust service instead of pushing payloads to an external Chatwoot API.
  4. Integrate the message creation flow with the PostgreSQL `SKIP LOCKED` job queue to immediately trigger the `message_triage` AI background worker upon receiving a customer message.
  Ensure complete backend unit testing and Playwright E2E coverage for the unified inbox Critical User Journey (CUJ).

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
