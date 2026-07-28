issue_title: "Architecture Design: Native Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  Small business owners (Maya, Carlos, Priya, Leo, Fatima) need a unified way to manage communications across multiple channels (Instagram, WhatsApp, Email, Web Widget). The legacy architecture relied on Chatwoot, which has been removed as a dependency. OHC now needs a native Rust-based omnichannel chat system to replace Chatwoot, guaranteeing data privacy, multi-tenant isolation, and a seamless, high-performance experience without the operational overhead of third-party systems.

  ## Research Report
  Based on an audit of the `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md` and an inspection of the legacy Chatwoot Ruby on Rails codebase, several key concepts are required:
  - **Conversations & Messages**: The core domain model, mapping messages to a unified conversation timeline.
  - **Channels & Inboxes**: Abstractions for external services (Email, WhatsApp, Facebook, Twilio) routing messages to a tenant-specific inbox.
  - **Contacts**: Omnichannel identity resolution to track a single customer across multiple platforms.
  - **Agents & Automations**: AI/Human routing, SLA policies, macros, and canned responses.

  Unlike Chatwoot, the OHC system must natively integrate our AI agents (like "The Ambassador" and "The Manager") as first-class citizens in the conversation loop, drafting responses and automating operational tasks directly from the unified inbox, all with strict multi-tenant isolation at the row level via PostgreSQL RLS.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhooks: WhatsApp/Insta/Email] --> B[Ingress Verification & Gateway]
      B --> C{Channel Adapters}
      C --> D[Unified Event Mesh / Queue]
      D --> E[Identity Resolution Engine]
      E --> F[(PostgreSQL: RLS Isolated)]
      F --> G[Conversation State Machine]
      G --> H[AI Agents: The Ambassador]
      H --> I[Draft Action Required]
      G --> J[Realtime WebSocket / PowerSync]
      J --> K[Mobile/Web UI 375px]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Unified Inbox Feed (Mobile View)**:
    - Sticky glassmorphism header showing active/snoozed/resolved filters.
    - Conversation cards displaying customer name, last message preview, channel icon (e.g., WhatsApp), and time elapsed.
  - **Conversation Thread**:
    - Clean, bubble-style chat interface.
    - Integrated AI Draft area at the bottom: "The Ambassador suggests..." with "Approve", "Edit", "Discard" buttons.
    - Context drawer (swipe left) showing past orders, customer tags, and notes.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success)**: Listens to new inbound messages on the Event Mesh. Uses RAG on customer history and product catalog to draft intelligent replies.
  - **The Manager (Operations)**: Analyzes conversation intent for actionable items (booking, order change, cancellation) and prompts the owner with operational tasks alongside the reply draft.

  ### Key Design Decisions
  - **Rust Microservices**: High performance, memory safety, and precise control over multi-tenant data access (RLS).
  - **Unified Timeline**: All interactions (messages, notes, AI drafts, status changes) are events on a single immutable conversation timeline.
  - **Offline & Realtime Parity**: Deep integration with PowerSync for local-first SQLite replication, ensuring the inbox works flawlessly in low-connectivity environments (like Fatima's food cart).
  - **Strict RLS**: Every database table related to the chat system MUST include `tenant_id` and enforce PostgreSQL Row Level Security.

  ## Implementation Prompt
  **User-Facing Outcome**: When a business owner opens the OHC app, they see a single, unified list of all customer messages from Instagram, WhatsApp, and their website. Clicking a message shows the full history and an AI-drafted reply ready for 1-tap approval.

  **Acceptance Criteria**:
  1. Define the PostgreSQL schema and Rust domain models for `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message` with strict `tenant_id` RLS.
  2. Implement the `ChannelAdapter` trait/interface in Rust for standardized webhook ingestion (starting with a Mock/Local channel for testing).
  3. Create the `ConversationStateMachine` to handle status transitions (Open, Snoozed, Resolved, Bot-Handled).
  4. Ensure 100% unit test coverage for the Rust domain models and state transitions.
  5. Provide Playwright E2E tests validating the creation of a conversation, routing a message, and updating the unified inbox UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
