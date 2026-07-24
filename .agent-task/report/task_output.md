issue_title: "[Research] OHC Native Rust Chat & Inbox Architecture (Chatwoot Replacement)"
issue_description: |
  # Research Report: OHC Native Rust Chat & Inbox Architecture

  ## Problem Statement
  OneHumanCorp previously relied on Chatwoot as an external service for omnichannel customer support and messaging. This external dependency violates the OHC architectural vision of a unified, self-contained, native rust platform, introduces latency, and creates fragmented multi-tenant state. The current mandate is to 100% retire Chatwoot as an external dependency and build a native, high-performance, multi-tenant Rust alternative inside `onehumancorp/mono`.

  ## Research Report & Findings
  Based on an audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) and competitive research (Shopify Inbox, Wix Inbox), we have identified the core architectural components necessary for feature parity:

  1.  **Core Data Models:**
      - Account/Tenant (Multi-tenant isolation)
      - Inbox (Channel grouping)
      - Conversation (Thread of messages)
      - Message (Individual communication)
      - Contact (Customer profile)
      - Channel (Email, Web Widget, API, etc.)
  2.  **Omnichannel Adapters:** Abstracted interfaces for receiving and sending messages across different channels (e.g., Web Widget, Email, SMS, WhatsApp).
  3.  **Real-time Layer:** WebSocket-based event broadcasting for instant updates to the agent dashboard and customer widgets.
  4.  **Agent & Team Routing:** Logic for assigning conversations to specific team members or AI agents based on rules, availability, or workload.
  5.  **AI Assistant Integration:** Deeply embedded AI context. OHC's key differentiator is that the AI Assistant is a first-class participant in the inbox, capable of drafting replies, analyzing sentiment, and summarizing context automatically.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      A[Customer Widget/API/Email] --> B(Channel Adapters)
      B --> C{OHC API (Rust/Axum)}
      C --> D[PostgreSQL (Multi-tenant tables)]
      C --> E[Valkey (Cache/PubSub)]
      E --> F(WebSocket Hub)
      F --> G[Owner/Operator Dashboard (Tauri/Flutter)]
      C --> H[AI Assistant Engine]
      H -.-> C
  ```

  ### Core Entities (Rust/Postgres Schema Guidelines)
  - `tenant_id` MUST be present on all tables for Row Level Security (RLS).
  - `inboxes`: Logical grouping of channels.
  - `conversations`: Belongs to an inbox and a contact. Tracks `status` (open, resolved, snoozed).
  - `messages`: Belongs to a conversation. Includes `message_type` (incoming, outgoing, template) and `content_type` (text, image, interactive).
  - `contacts`: Customer profiles mapped across channels.

  ### Mobile UX Flow (375px First)
  1.  **Unified Inbox View:** A clean list of active conversations, prioritized by urgency or AI flags. Unread indicators and AI draft status are clearly visible.
  2.  **Conversation View:** Native-feeling chat interface. The AI's suggested reply sits subtly above the keyboard.
  3.  **Customer Context Drawer:** Swiping left reveals contact details, past orders, and AI-generated summaries without leaving the conversation.

  ### AI Agent Integration Points
  - **Triage & Routing:** The AI agent analyzes incoming messages to categorize, prioritize, and route them to the appropriate human or automated flow.
  - **Drafting:** The AI automatically prepares suggested responses based on conversation history, knowledge base, and business context.
  - **Summarization:** Long threads are summarized into brief bullet points for the owner's quick review.

  ## Implementation Prompt
  **Role:** Backend Implementer (Rust)
  **Objective:** Implement the foundational database schema and core Rust API services for the new native OHC Unified Inbox, replacing the deprecated Chatwoot dependency.

  **Requirements:**
  1.  Define the initial PostgreSQL schema migrations (using existing OHC tooling) for `inboxes`, `conversations`, `messages`, and `contacts`. Ensure strict multi-tenant isolation via `tenant_id` and RLS.
  2.  Implement the core Rust domain models and repository layer for these entities.
  3.  Create the initial REST/gRPC API endpoints for listing inboxes, fetching conversations, and sending/receiving messages.
  4.  Ensure the architecture supports the future addition of real-time WebSockets and specific channel adapters.

  **Acceptance Criteria:**
  - Migrations run successfully and enforce multi-tenancy.
  - Core API endpoints are functional and tested via unit tests (100% coverage).
  - The design aligns with the proposed architecture, enabling future AI integration and real-time updates.

  ## Next Steps
  - Dispatch the implementation task to the swarm.
  - Follow up with tasks for the WebSocket real-time layer and frontend UI components.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chatwoot-replacement]
assignees: []
