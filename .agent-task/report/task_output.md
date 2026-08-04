issue_title: "Architecture Design: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ### Problem Statement
  Currently, OneHumanCorp (OHC) requires a fully native, multi-tenant omnichannel chat engine that integrates seamlessly with our core Rust architecture. This lack causes friction for our key personas—like Maya (who relies on Instagram DMs) and Carlos (who needs SMS/Email follow-ups)—preventing them from managing all customer interactions in one unified inbox without depending on third-party SaaS tools like Chatwoot.

  ### Research Report
  - **Market Context**: Platforms like Shopify (Shopify Inbox), Wix (Wix Inbox), and Zendesk provide unified communication layers. Chatwoot provides a strong omnichannel model but requires external dependency management, higher latency, and complex multi-tenant isolation that breaks our Zero-Trust architecture.
  - **Audit Findings**: As mandated, Chatwoot as an external service is 100% retired. To match Chatwoot's capabilities natively, we inspected Chatwoot's source code (`https://github.com/chatwoot/chatwoot`):
    - `Inbox` model in Chatwoot (`app/models/inbox.rb`): Has attributes like `channel_type`, `auto_assignment_config`, `account_id` (which maps to our `tenant_id`), `channel_id`, `csat_config`, `enable_auto_assignment`.
    - `Conversation` model (`app/models/conversation.rb`): Belongs to `Inbox` and `Account`, tracks status, assignee, and custom attributes.
    - `Message` model (`app/models/message.rb`): Tracks sender, conversation, attachments, content, and message type.
  - **Scaling Needs**: The solution must handle high-concurrency WebSocket connections, support seamless integration with our `ohc-builtin-agent` harness for auto-replies, and provide strict row-level multitenancy in PostgreSQL (`tenant_id`).

  ### Design Doc
  - **Architecture Diagram**:
    ```mermaid
    graph TD;
      Client[Mobile/Web App] -->|WebSocket/REST| API[Rust API Server];
      API --> Auth[SPIFFE/OIDC Auth Layer];
      API --> ChatEngine[Native Chat Engine];
      ChatEngine --> DB[(PostgreSQL)];
      ChatEngine --> Agents[AI Agents for Auto-replies];
      Channels[External Channels: IG, WhatsApp, Email] --> Webhooks[Rust Webhook Handlers];
      Webhooks --> ChatEngine;
    ```
  - **Mobile UX Flow**: A unified inbox accessible on 375px screens. The inbox lists all active conversations with platform icons (e.g., IG, Email). Tapping a conversation opens a standard chat interface with AI suggested replies and actions.
  - **Data Model**: Follow strict multi-tenant isolation rules. You must design entities for Inbox, Conversation, and Message in PostgreSQL enforcing row-level security per tenant.
  - **AI Agent Integration Points**: The chat system will dispatch events to the `kairos` sub-agent queue. AI agents can pick up unassigned conversations, draft replies (e.g., answering "do you do vegan cakes?" for Maya), and seamlessly escalate to humans.

  ### Implementation Prompt
  **User Facing Outcome**: The owner can open the OHC app and see a unified inbox of all customer messages across multiple channels. They can read and reply, and see AI-drafted responses.

  **Implementation Tasks**:
  1. Create the `chat` module in `src/server/services/chat/`.
  2. Implement the core database schemas for Inboxes, Conversations, and Messages using strict `tenant_id` isolation, taking inspiration from Chatwoot's data models. Do not use external services.
  3. Expose gRPC/REST APIs and a WebSocket event handler for real-time message delivery.
  4. Ensure 100% unit test coverage for the new module and include a Playwright E2E test verifying a basic conversation flow.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
