issue_title: "[Architecture] Native Rust Omnichannel Chat & Inbox System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC requires a high-performance, multi-tenant omnichannel customer support & chat engine. Currently, there is an instruction to completely RETIRE Chatwoot as an external third-party service and replace it with a native Rust implementation within the `onehumancorp/mono` codebase. The external service introduces unnecessary dependencies, scaling complexities, and breaks the Zero Trust multi-tenant data isolation required by OHC's architecture.

  Owners (like Maya the baker and Carlos the handyman) need a unified inbox to manage DMs, emails, and web chats from a single, mobile-friendly interface without knowing what a channel adapter or webhook is. The unified inbox must support real-time messaging, AI agent auto-responses, and strict multi-tenant data isolation.

  ## Research Report
  - **Chatwoot Audit**: The Chatwoot Ruby on Rails codebase uses complex relational models for `Account`, `Inbox`, `Conversation`, and `Message` with polymorphic `Channel` adapters and real-time WebSocket publishing.
  - **Data Model Insights (Chatwoot)**:
    - `conversations` link `account_id`, `inbox_id`, `contact_id`, and `assignee_id` with statuses.
    - `messages` link `conversation_id`, `account_id`, `inbox_id`, and `sender_id` with `content_type` (text, attachment, etc.) and `message_type` (incoming, outgoing, template).
    - `inboxes` act as channel endpoints with rules like `working_hours_enabled` and `csat_survey_enabled`.
  - **Competitor Insights**: Systems like Shopify Inbox, WeCom, and Stripe Customer portals consolidate all customer interactions into a single stream. The architectural pattern is a unified "Ledger" of interactions, with specialized adapters for Instagram, WhatsApp, Email, and Web Widget.
  - **OHC Technical Gap**: OHC lacks this native Rust implementation for Inboxes, Conversations, and Messages. We need a robust, multi-tenant data model and service layer in Rust to replicate Chatwoot's functionality with superior performance and deeper integration into the OHC AI Agent ecosystem.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONVERSATION : owns
      TENANT ||--o{ CONTACT : owns

      INBOX ||--o{ CONVERSATION : routes
      INBOX ||--o| CHANNEL_ADAPTER : configured_with

      CONTACT ||--o{ CONVERSATION : participates_in

      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION ||--o{ CONVERSATION_PARTICIPANT : has

      MESSAGE ||--o{ ATTACHMENT : includes
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox View**: A simple list view showing all active conversations, badged by unread status. Avatars indicate the channel (e.g., WhatsApp icon, Email icon).
  2. **Conversation Thread**: Tapping a conversation opens a chat thread. The UI uses macOS Translucent Glass styling. AI-drafted replies appear as "Ghost text" above the keyboard for one-tap sending.
  3. **Contact Context Sheet**: Swiping left or tapping the contact header pulls up a half-sheet with the customer's lifetime value, past orders, and AI-summarized sentiment.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Automatically assigns tags and priority to incoming messages based on sentiment and intent analysis.
  - **Customer Assistant Agent**: Subscribes to conversation creation events. If configured, it automatically drafts and sends replies for common queries (e.g., "Do you do vegan cakes?") while the owner sleeps.
  - **Operations Agent**: Parses messages for actionable intents (e.g., booking a service, requesting a quote) and proposes structured actions in the chat UI.

  ### Key Design Decisions
  - **Strict Multi-Tenancy**: Every table must have a `tenant_id` with Row-Level Security (RLS) enforced at the Postgres level.
  - **Rust Microservice/Crate**: The chat system will be built as a native Rust crate within `src/server/integrations/chat/` or a dedicated `src/server/chat/` module, leveraging gRPC for internal communication and WebSockets for real-time client updates.
  - **Unified Event Bus**: Leverage Redis/NATS for pub-sub to dispatch new messages to connected clients and AI Agents simultaneously.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Build the foundational data models and service layer for the native Rust Omnichannel Chat System in OHC, replacing Chatwoot.

  1. **Target Persona**: Maya (Baker) who needs to see Instagram DMs, Web Chats, and Emails in one single list on her iPhone.
  2. **Core Task**: Implement the Rust data structures and repository layer for `Inbox`, `Conversation`, and `Message` entities. Ensure strict multi-tenant isolation (`tenant_id`) is baked into every struct and query.
  3. **Acceptance Criteria**:
     - Rust structs for `Inbox`, `Conversation`, and `Message` are defined with proper fields (aligning with the audited Chatwoot models but modernized for Rust/OHC).
     - Repository implementation for CRUD operations with built-in multi-tenant filtering.
     - 100% Unit test coverage for the new repository layer.
     - Provide a clear API boundary (e.g., gRPC service definition or Rust traits) for creating conversations and sending messages.
     - (No need to implement the full WebSocket layer or specific channel adapters in this first PR, focus on the core data engine).
     - Must compile successfully with `bazel test //...`.

  ## Priority
  P0 (Critical path for Chatwoot retirement)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chatwoot-replacement]
assignees: []
