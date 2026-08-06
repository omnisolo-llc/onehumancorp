issue_title: "[Research] Architect OHC Custom Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC currently lacks a unified, native, high-performance, and multi-tenant omnichannel customer support & chat engine. The instructions mandate the **COMPLETE RETIREMENT** of Chatwoot as an external third-party service and require replacing it with a native Rust implementation directly inside the `onehumancorp/mono` repository, achieving 100% feature parity. For a small business owner (like Maya the Baker or Carlos the Handyman), fragmented communication across Instagram, WhatsApp, email, and a website widget creates operational friction, missed leads, and dropped context. They need a single, unified "inbox" that coordinates all customer interactions seamlessly, without relying on or managing a heavy external dependency like Chatwoot.

  ## Research Report & Gap Analysis
  - **Chatwoot Architecture Audit:** I have cloned and analyzed the Chatwoot source code (`https://github.com/chatwoot/chatwoot`). Chatwoot's core architecture relies heavily on a complex Ruby on Rails monolith managing `accounts`, `inboxes`, `conversations`, `messages`, `contacts`, and `channel` adapters.
  - **The Missing Piece in OHC:** OHC needs to replicate this functionality using Rust to ensure high performance, tight integration with our existing multi-tenant architecture (PostgreSQL row-level security), and AI agent orchestration (Work Triage, Customer Assistant).
  - **Key Required Components:**
    - **Data Model:** Core entities mapping to `Tenant`, `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message`.
    - **Channel Adapters:** Interfaces for Web Widget, Email, SMS (Twilio), WhatsApp, and Instagram/Messenger.
    - **Real-time Engine:** WebSocket or Server-Sent Events (SSE) for instant message delivery and read receipts.
    - **AI Integration:** Hooks for the Customer Assistant to automatically draft replies, categorize intent, and perform actions based on conversation context.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      TENANT ||--o{ CONTACT : manages
      CONTACT ||--o{ CONVERSATION : participates
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE }|--|| CONTACT : sender
      MESSAGE }|--|| TENANT : sender (agent)
  ```

  ### Component Flow
  1. **Ingress:** External webhooks (WhatsApp, IG) or the Web Widget hit the OHC Rust API (`/api/v1/webhooks/chat`, `/api/v1/widget/messages`).
  2. **Processing:** The message is authenticated, associated with a `Tenant` and `Contact`, and saved to the `messages` table via `sqlx`.
  3. **Event Dispatch:** A message creation event is published to the internal message bus (e.g., Redis Pub/Sub or similar OHC mechanism).
  4. **AI Interception:** The Work Triage and Customer Assistant agents subscribe to the message bus, analyze the message, update context, and potentially generate an automated response (draft or direct send).
  5. **Real-time Push:** Connected frontend clients (Flutter/Tauri) receive the new message via WebSockets.

  ### Mobile UX Flow (375px)
  - **Inbox View:** A clean list of active conversations, prioritized by the AI Work Triage. Each row shows the customer name, channel icon (e.g., IG, Web), snippet, and an AI-generated intent tag.
  - **Conversation View:** Standard chat interface. Messages are bubbled. A prominent "AI Assistant" button above the keyboard allows the owner to request a drafted reply or action (e.g., "Send Payment Link", "Book Appointment").

  ### AI Agent Integration
  - **Context Provider:** The conversation history acts as the primary context for the Customer Assistant.
  - **Action Triggers:** Messages can trigger agent actions (e.g., a message containing "book a time" triggers the Operations Assistant to surface calendar slots).

  ## Implementation Prompt
  **Goal:** Implement the core backend data models and REST APIs for the native Rust omnichannel inbox, replacing the need for external Chatwoot.

  **Persona:** Maya (Baker) needs all her Instagram DMs and website inquiries to appear in one place on her phone, with AI suggesting replies.

  **Requirements:**
  1.  **Database Schema:** Design and create migrations for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`. Ensure strict multi-tenant isolation using `tenant_id` and RLS where applicable.
  2.  **API Endpoints:** Implement CRUD endpoints for these entities under a new `src/server/services/chat/` or `src/server/domain/chat/` module.
  3.  **Real-time Foundation:** Stub out the WebSocket integration for message delivery (if not already existing in the platform) or integrate with the existing event bus.
  4.  **Testing:** Write comprehensive unit and integration tests for the new models and APIs. Ensure 100% code coverage for new files.
  5.  **Documentation:** Update relevant API docs.

  **Acceptance Criteria:**
  - A client can create an inbox, associate a contact, start a conversation, and send/receive messages via the REST API.
  - All operations are strictly tenant-isolated.
  - The implementation is purely native Rust and integrates with the existing OHC stack (no external Chatwoot dependencies).
  - All tests (`bazel test //...`) pass.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
