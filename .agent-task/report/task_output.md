issue_title: "[Omnichannel] Implement Native Rust Chatwoot Replacement"
issue_description: |
  ## Problem Statement
  OHC currently relies on external Chatwoot for customer chat integration, which breaks our Zero-Trust architecture and tight multi-tenant data model. Small business owners (like Maya the Baker or Carlos the Handyman) need a seamless, invisible omnichannel chat experience (Instagram DMs, Web Chat, SMS) that lives natively within the OHC ecosystem. We need to retire external Chatwoot dependencies and build a high-performance native Rust replacement for OHC's backend and a seamless Flutter integration on the frontend.

  ## Research Report
  Based on an audit of the `chatwoot/chatwoot` source repository:
  1. **Data Model Insights**:
     - `Conversation` model uses `account_id` for multi-tenancy. Key fields: `status`, `assignee_id`, `contact_id`, `inbox_id`.
     - `Message` model handles the actual content (`content`, `content_type`, `message_type`, `private`).
     - `Inbox` represents the channel (e.g., Web, Instagram, SMS).
     - `Contact` represents the customer entity.
  2. **Architecture**: Chatwoot relies heavily on background jobs and webhooks to process messages from external platforms (Meta API, Twilio).
  3. **Gap Analysis for OHC**: We need a Rust-based, Bazel-built microservice (or mono-repo module) that replicates this `Conversation -> Message -> Inbox` structure with strict row-level security (RLS) tied to OHC's `tenant_id`.

  ## Design Doc
  ### Architectural Flow
  - External platforms (Meta, Twilio) hit the OHC Ingress Webhook endpoint.
  - Rust API (gRPC/REST) parses the payload, identifies the `tenant_id` and `inbox_id`.
  - Creates/updates `contacts`, `conversations`, and `messages` in PostgreSQL (with RLS).
  - Publishes events to a Redis pub/sub or PostgreSQL `SKIP LOCKED` queue for real-time WebSocket delivery to the owner's active Flutter client and AI Agent processing.

  ### Data Model Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ CONVERSATION : owns
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string channel_type
          string name
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string phone_or_email
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string content
          string sender_type
      }
  ```

  ### Mobile-First UX Flow (375px)
  1. **Work Triage Feed**: Owner opens the app and sees "3 New Inquiries" in the unified feed.
  2. **Conversation View**: Tapping an inquiry opens a chat interface. The UI uses macOS Translucent Glass materials.
  3. **Smart Actions**: Above the keyboard, AI suggests "Draft Quote" or "Send Payment Link".
  4. **Send**: Owner taps 'Send'. The Rust backend routes the message back through the correct channel adapter (e.g., Meta API) based on the `inbox_id`.

  ### AI Agent Integration
  - **Customer & Relationship Assistant**: Listens to the message queue. If a new message arrives and the owner is offline, the AI drafts a response or replies automatically if confident (e.g., "Yes, we do vegan cakes. Here is the link...").

  ## Implementation Prompt
  **Goal**: Implement the core data layer and basic REST/gRPC API for the native Rust Omnichannel Chat system, fully retiring any Chatwoot dependencies.

  **Tasks for Implementer**:
  1. Design PostgreSQL schemas for `inboxes`, `contacts`, `conversations`, and `messages`. All tables MUST include `tenant_id` and enforce RLS.
  2. Implement Rust models and data access layers in `src/server/services/chat/`.
  3. Create gRPC/REST endpoints to create and list conversations and messages.
  4. Ensure 100% unit test coverage for the new code.
  5. Add Playwright E2E tests demonstrating a simulated incoming webhook creating a message, and the OHC UI displaying it. (Use local mock adapters for external webhooks).

  **Priority**: P0
  **Estimated Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
