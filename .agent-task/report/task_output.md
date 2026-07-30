issue_title: "🔍 Scout: Tool Integration Research - Native Omnichannel Chat (Chatwoot Parity)"
issue_description: |
  **Title**: Native Omnichannel Chat System in Rust (Chatwoot Parity)

  **Problem Statement**:
  Non-technical owner/operators like Maya (Home Baker) and Carlos (Field Service Owner) receive inquiries across multiple channels—WhatsApp, Instagram DMs, website chat, and email. Managing these fragmented channels leads to dropped leads and delayed responses. They need a unified inbox where all messages flow into one place, enabling them to read and reply from one cohesive interface without needing to understand the underlying APIs or channel integrations.

  **Research Report**:
  Following the Chatwoot Retirement standard, we evaluated the open-source Chatwoot repository (`https://github.com/chatwoot/chatwoot`) to determine the core features required for a native Rust-based omnichannel chat system within OHC.
  Key findings from Chatwoot's source code (`app/models` and `app/controllers`):
  - **Inbox & Channel Architecture**: Chatwoot maps generic 'Inboxes' to specific 'Channels' (e.g., `Channel::Whatsapp`, `Channel::WebWidget`). This abstracts channel specifics away from the unified conversation UI.
  - **Contact & Conversation Model**: A 'Contact' has many 'Conversations', which contain 'Messages'. Messages can be incoming, outgoing, or internal notes (templates/macros).
  - **Webhooks & Events**: Real-time updates rely heavily on WebSocket events and webhooks to synchronize the frontend with incoming messages.
  - **SLAs & Automation**: They provide SLA policies and automated assignments (Round Robin), which are critical for larger teams but for small owners, simple triage and smart AI replies take precedence.
  - **Pricing/Viability**: Third-party solutions are expensive and limit control over our own AI triage flows. Building natively in Rust gives us sub-millisecond response times, absolute data privacy (essential for AI processing), and tight integration with OHC's internal event loop.

  **Design Doc**:
  To build a 100% native Rust omnichannel chat system inside OHC:
  1. **Core Domain Models**: Implement `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message` entities in PostgreSQL, scoped by `tenant_id` for row-level security.
  2. **Channel Connectors**: Start with `WebWidget` and `WhatsApp` (via Cloud API) channels. Implement webhook receivers in Rust to ingest messages and map them to standard `Message` entities.
  3. **Event System**: Use Redis Pub/Sub to dispatch real-time events to connected clients via WebSockets when new messages arrive.
  4. **AI Triage Integration**: Wire incoming messages into the existing AI Job Queue to allow the Customer & Relationship Assistant to draft replies or extract tasks seamlessly.
  5. **User Experience**: Present a unified feed where Maya sees all incoming chats regardless of origin, integrated directly into her OHC workspace.

  **Implementation Prompt**:
  Build the foundational Native Omnichannel Chat system in Rust to replace third-party dependencies, adhering to the Chatwoot Parity standard.
  - **Acceptance Criteria**:
    - Define and migrate database schemas for Contacts, Conversations, Messages, Inboxes, and Channels.
    - Create a Rust microservice/crate that handles incoming webhook payloads for WhatsApp and normalizes them into standard Message entities.
    - Implement a WebSocket server to push real-time message updates to the Flutter frontend.
    - Develop the core UI in Flutter for the unified inbox: a list of conversations and a threaded message view.
    - Ensure the UI allows replying, which then routes the outgoing message through the appropriate channel adapter.
    - Verify end-to-end flow using the real local stack and write Playwright E2E tests for the new UI components. No mocked APIs in E2E tests.

  **Priority**: P0

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
