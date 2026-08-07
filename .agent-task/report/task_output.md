issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  **Problem Statement**
  The current mission queue protocol explicitly requires the retirement of Chatwoot as an external third-party integration target. OHC aims to implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust directly inside `onehumancorp/mono`. Non-technical owner/operators (like Maya the home baker, Carlos the field service owner, etc.) need a unified inbox that brings together emails, web widget chats, WhatsApp, Instagram DMs, etc. Currently, relying on third-party SaaS for omnichannel chat violates the directive of building native Rust microservices for this feature.

  **Research Report**
  After reviewing the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), the primary domain model for an omnichannel chat system consists of the following key entities and features:
  - **Account/Tenant**: Multi-tenant isolation for different owners/operators.
  - **Inboxes and Channels**: Support for multiple channels such as Web Widget, API, Email, WhatsApp, SMS, Facebook, Instagram, etc.
  - **Contacts**: End-users reaching out to the business, managed via `Contact` models.
  - **Conversations & Messages**: Threading messages, handling attachments, assignment rules, routing, and bot integration.
  - **Webhooks & Integrations**: Receiving inbound messages securely (e.g., verifying HMAC/tokens) and dispatching updates.
  - **Automations & SLA**: Automated replies, routing, tagging based on rules, and measuring reply time.

  The native Rust microservice needs to achieve feature parity with Chatwoot, starting with the core chat infrastructure (Inboxes, Conversations, Messages, Contacts, and Web Widgets). This is crucial for OHC's product differentiation: delivering an assistant-first, integrated work command center rather than cobbling together multiple tools.

  **Design Doc**
  1. **Core Data Models (Rust/PostgreSQL)**: Define `tenant_id` isolated schemas for `inboxes`, `channels` (polymorphic or specific tables like `channel_web_widget`, `channel_whatsapp`, etc.), `contacts`, `conversations`, and `messages`.
  2. **API Layer (gRPC/REST)**: Implement endpoints to create inboxes, manage contacts, fetch conversation history, and send/receive messages. Ensure OpenAPI specs are provided for mobile/web clients.
  3. **Web Widget Integration**: Develop a lightweight embeddable script for the web widget that communicates with the Rust backend via WebSockets for real-time chat, supporting features like pre-chat forms and welcome messages.
  4. **Omnichannel Adapters**: Build modular channel adapters for Email (IMAP/SMTP or SendGrid/Mailgun webhooks), WhatsApp (Meta Cloud API), and SMS (Twilio/360dialog).
  5. **Agent/Assistant Integration**: Integrate the unified inbox with the OHC AI Assistant to allow it to draft replies, triage messages, and coordinate tasks based on chat context.

  **Implementation Prompt**
  Build a native Rust omnichannel chat system within `onehumancorp/mono` that achieves feature parity with the core functionality of Chatwoot.
  - **User-Facing Outcome**: The owner opens OHC and sees a unified "Work Triage" feed containing messages from their website widget, WhatsApp, and emails. They can read context, approve AI-drafted replies, or manually reply, all within a clean, native interface. Customers see a fast, responsive chat widget on the owner's website.
  - **Acceptance Criteria**:
    - Core database schema implemented with row-level tenant isolation.
    - REST/gRPC APIs built for managing inboxes, contacts, and conversations.
    - Web Widget channel supported with an embeddable JS script and WebSocket real-time delivery.
    - Webhook infrastructure in place to receive messages from external channels (e.g., WhatsApp).
    - AI Assistant can read conversation history and draft replies via API.
    - End-to-end (E2E) Playwright tests verifying the UI flow of creating an inbox, starting a chat from the widget, and replying as an owner.
    - 100% unit test coverage for the Rust microservices.

  **Priority**: P0 (Critical - required per engineering standards)
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
