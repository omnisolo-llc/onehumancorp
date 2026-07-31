issue_title: "Implement Native Rust Omnichannel Chat System for OHC"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context and require the owner to manually type responses. We need an integrated, native omnichannel unified inbox within OHC.

  As mandated by OHC Engineering Standards, we are **100% RETIRING** external legacy external dependency dependencies. We need a native Rust implementation of legacy external dependency's core omnichannel architecture within `onehumancorp/mono`.

  # Research Report
  **Findings & Competitive Analysis:**
  - **legacy external dependency Source Code Audit:** legacy external dependency uses a robust schema with entities like `Contact`, `Inbox`, `Conversation`, and `Message` built on Ruby on Rails. Our implementation will replicate this conceptual model in Rust, leveraging high-performance, multi-tenant row-level security (RLS).
  - **Shopify/Wix Inbox:** Aggregates chat but lacks deep proactive AI contextual replies.
  - **OHC Opportunity:** A native Rust implementation integrated with OHC's "Teammate" AI (The Ambassador) allows seamless real-time WebSocket communication and background AI processing.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[WhatsApp Webhook] -->|Ingest| B(Rust API Server / Omnichannel Gateway)
      C[Instagram Webhook] -->|Ingest| B
      D[Email Webhook] -->|Ingest| B
      E[Web Widget] <-->|WebSocket| B
      B --> F{Identity Resolution}
      F --> G[(PostgreSQL with RLS)]
      G -->|Conversation/Message| H[Action Required Queue]
      H --> I[OHC AI Agents - The Ambassador]
      I -->|Draft Reply| G
      G -->|Publish| E
      G --> J[Mobile App UI - Unified Inbox]
  ```

  ### Core Data Model (Rust/PostgreSQL)
  Replicating legacy external dependency's core concepts with OHC's multi-tenant architecture:
  - `Contact`: Represents the end-customer.
  - `Inbox`: Represents a specific channel connection (e.g., WhatsApp Business Account, Instagram Page).
  - `Conversation`: A thread linking a `Contact` to an `Inbox`.
  - `Message`: Individual messages within a `Conversation`.

  All tables MUST include `tenant_id` and have Row Level Security (RLS) enabled.

  ### Mobile UX Flow (375px First)
  - **Home Feed:** Top card shows aggregated new messages across all channels.
  - **Unified Inbox View:** A streamlined chat interface showing the conversation history, regardless of which channel the customer used.
  - **Agent Interaction:** AI-drafted responses appear as "Approve Draft" blocks inline, allowing the owner to 1-tap send or edit.

  ### AI Agent Integration Points
  - **The Ambassador:** Triggered via database events or application-level hooks upon new incoming messages. Uses the unified `Conversation` and `Contact` context to draft intelligent replies.

  ### Key Design Decisions
  - **Native Rust:** High performance, memory safety, and seamless integration with the existing OHC Go/Rust monorepo structure.
  - **WebSocket Real-time:** Essential for web widget and mobile app responsiveness.
  - **Multi-Tenant First:** Built natively on our RLS PostgreSQL setup.

  # Implementation Prompt
  **User-Facing Outcome:** As an OHC owner, I have a single "Inbox" tab in my app. When a customer messages me on WhatsApp, Instagram, or my website widget, it appears instantly in this one view. The AI Ambassador automatically drafts contextual replies based on the customer's history.

  **CUJ & Acceptance Criteria:**
  1. Define the PostgreSQL schema (using SQL or Diesel/SQLx migrations depending on OHC's Rust stack) for `contacts`, `inboxes`, `conversations`, and `messages` with `tenant_id` and RLS.
  2. Implement the core Rust data models and CRUD operations.
  3. Implement a webhook ingestion endpoint in Rust capable of handling incoming messages and storing them in the new schema.
  4. Implement a basic REST or WebSocket API for the frontend to fetch unified conversations and messages.
  5. Provide unit tests ensuring tenant isolation (user A cannot see user B's conversations).

  **Priority:** P0 (Critical Infrastructure & legacy external dependency Replacement)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, rust, architecture, omnichannel]
assignees: []
