issue_title: "Native Rust Omnichannel Chat System Replication (Chatwoot Replacement)"
issue_description: |
  # Problem Statement

  OneHumanCorp (OHC) currently relies on an external integration with Chatwoot for omnichannel customer support and inbox functionality. This violates our core tenet of a unified, self-contained, native platform, and limits our ability to seamlessly inject our AI agents (like "The Ambassador") directly into the core event stream.

  We need to replace the external Chatwoot dependency entirely by replicating its core omnichannel data models, real-time WebSocket messaging, and inbox architecture natively in Rust inside `onehumancorp/mono`. This will enable true, invisible AI agent coordination for SMB owners, meeting our core value of "Radical Simplicity" where the system just works without complex third-party configurations.

  # Research Report

  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Audit**: A clone and audit of `https://github.com/chatwoot/chatwoot` reveals a robust Ruby on Rails architecture that we must adapt to Rust. Key components to replicate:
    - Multi-tenant data models: `Account` (Tenant), `Inbox`, `Channel`, `Conversation`, `Message`, `Contact`.
    - Real-time WebSocket infrastructure for live updates (ActionCable in Rails, we need a native Rust solution like `tokio` + `axum-ws` or `tungstenite`).
    - Channel Adapters: The abstraction layer for integrating with external providers (Instagram DMs, WhatsApp Cloud API, Email, Web Widget).
    - Event Pub/Sub: For triggering AI agent workflows (e.g., when a new message arrives).

  - **Why Native Rust?**: Performance, memory safety, and tight integration with our existing (hypothetical, based on prompt) Go/Rust backend and AI agent orchestration layer. We need to own the entire data lifecycle to run our AI agents efficiently against the omnichannel streams.

  # Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[External Channels: IG, WhatsApp, Email, Web] -->|Webhooks/APIs| B(Channel Adapters - Rust)
      B --> C{Omnichannel Gateway}
      C -->|Pub/Sub Event| D(Event Mesh)
      C --> E[(Unified Customer Graph DB - PostgreSQL)]
      D --> F[AI Agents: The Ambassador, etc.]
      F -->|Read Context| E
      F -->|Draft Reply| G(Action Required Queue)
      G --> H[Mobile Client - Flutter UI]
      H -->|Approve/Send| C
      C -->|Dispatch| B
      B --> A
      C <-->|WebSocket Stream| H
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)

  - **Mobile Inbox View:** A clean, single-column feed of conversations across all channels.
  - **Conversation Detail:**
    - Standard chat UI (bubbles) but integrated with OHC Premium Tokens (Glassmorphism, clear typography).
    - Top area shows AI-summarized customer context (past orders, tags).
    - "Smart Reply" area at the bottom: AI pre-drafts the response based on the conversation history and business knowledge base.
    - Large, accessible "Approve & Send" button.

  ### AI Agent Integration Points

  - The new native chat system must emit strongly typed events (e.g., `MessageReceived`, `ConversationCreated`) to a central event bus (like Redis or Kafka, as per our architecture).
  - "The Ambassador" agent listens to these events, accesses the tenant's data, and generates draft replies, pushing them back into the native system's `Drafts` or `PendingApproval` state.

  ### Key Design Decisions

  - **Strict Multi-Tenancy**: The database schema must enforce row-level security (RLS) on `tenant_id` for all chat-related tables (`inboxes`, `conversations`, `messages`, `contacts`).
  - **Stateless WebSocket Servers**: The WebSocket implementation must be designed to scale horizontally, utilizing Redis for pub/sub across server nodes to ensure messages reach the right connected clients regardless of which node they hit.
  - **Extensible Channel Trait**: Design a core Rust `Channel` trait/interface that allows easily adding new external integrations (e.g., SMS via Twilio) later without modifying the core conversation logic.

  # Implementation Prompt

  **User-Facing Outcome**: The SMB owner can manage all customer conversations (Instagram, WhatsApp, Web) directly within the OHC mobile app. The AI automatically drafts highly accurate replies based on the customer's history. There is no external "Chatwoot" setup required; it's seamlessly integrated.

  **CUJ & Acceptance Criteria**:
  1. A new `Conversation` is created via a mock incoming webhook simulating an Instagram DM.
  2. The system correctly identifies or creates a `Contact` associated with the tenant.
  3. The `Message` is persisted in the database with the correct `tenant_id`.
  4. An event is emitted that triggers the AI agent (mocked for this specific task if needed) to generate a draft reply.
  5. The mobile client (via Playwright E2E test) receives real-time updates of the new message and the AI draft via WebSocket.
  6. The user clicks "Approve" on the AI draft, and the system correctly dispatches the final message back out through the channel adapter.
  7. **Crucial**: The entire backend implementation must be in Rust, replacing the conceptual need for Chatwoot.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat, rust]
assignees: []
