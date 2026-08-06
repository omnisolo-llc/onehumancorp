issue_title: "Build Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  **Problem Statement**
  OHC currently lacks a native omnichannel chat and customer support system, and the external third-party Chatwoot integration is being completely retired. A non-technical owner/operator needs a unified, simple interface to manage messages from various sources (Web, WhatsApp, Instagram, Email, SMS) directly within OHC, without relying on fragmented third-party software or needing technical setup to route messages.

  **Research Report**
  As mandated, Chatwoot has been evaluated from its source code (`https://github.com/chatwoot/chatwoot`). Chatwoot's core entities include:
  - `Account` (tenant equivalent).
  - `Inbox` and `Channel` adapters (`WebWidget`, `WhatsApp`, `Instagram`, `Email`, `SMS`, `API`, `Telegram`, `FacebookPage`, `Line`, `TwitterProfile`).
  - `Conversation` (messages grouped by thread, assigned to agents or bots, tracking status, priority, and snoozed times).
  - `Message` (individual communication entries, with attachments, macros, and canned responses).
  - Webhooks and automation rules.

  Relying on external Chatwoot violates our goal of "Radical Simplicity" where setup and integrations are hidden, and it adds an extra layer of latency, failure points, and data privacy concerns. A native implementation provides better multi-tenant (tenant_id) row-level security and seamless OHC-native UI experience using our design system.

  **Design Doc**
  - **Triggers**: Messages arrive via webhooks from configured channels (WhatsApp, Instagram, etc.) or native web chat widgets.
  - **Actions**: The system normalizes these incoming messages into a unified `Conversation` and `Message` model in PostgreSQL. It then pushes events via WebSockets to connected OHC clients.
  - **User-Facing Outcome**: The owner sees a single "Unified Inbox" in the OHC assistant interface. They can view the conversation history, see the source channel icon, and draft replies. The replies are routed back through the appropriate native channel adapter. AI agents can monitor these conversations and draft suggested replies (Work Triage & Customer Assistant capabilities).

  **Implementation Prompt**
  Build a native, multi-tenant omnichannel chat engine in Rust within the `onehumancorp/mono` repository.
  - Create the unified data models for Inboxes, Conversations, Messages, and Contacts, ensuring they use OHC's standard `tenant_id` row-level security.
  - Implement a web chat widget adapter and at least two external channel adapters (e.g., WhatsApp and Email) based on Chatwoot's adapter logic, but written in Rust.
  - Build the backend WebSocket service to push real-time message updates to the OHC Flutter/PWA client.
  - The UI should present a clean, Apple/Ubiquiti-style unified inbox that hides the technical details of the channels from the owner.
  - Ensure 100% unit test coverage for the new Rust crates and end-to-end Playwright tests verifying the UI inbox flow.

  **Priority**: P0 (critical)
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
