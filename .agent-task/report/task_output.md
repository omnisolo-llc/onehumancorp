issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot with a high-performance, native Rust omnichannel chat system within `onehumancorp/mono`. Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels (Instagram DMs, WhatsApp, SMS, email). We need a unified inbox that aggregates these channels and supports AI agent workflows (like "The Ambassador" acting as a Customer Success Agent).

  Currently, our system lacks the native omnichannel data models, controllers, and messaging architectures required to handle this at scale securely. The owner needs an interface that works seamlessly on their mobile device (375px wide) without understanding the technical complexity.

  ## Research Report
  **Chatwoot Source Code Audit:**
  - **Models:** Uses extensive inheritance (e.g., `Channel::Email`, `Channel::Whatsapp`, `Channel::WebWidget`) feeding into an `Inbox` model, linked to `Conversation`, `Message`, and `Contact`.
  - **Real-Time:** WebSockets using ActionCable for pushing updates.
  - **Multi-Tenancy:** Handled at the account level (`account_id` on most models).

  **OHC Market Gaps & Opportunities:**
  - Shopify and Wix lack unified AI proactive drafting (The Ambassador).
  - OHC can build a zero-trust multi-tenant architecture natively in Rust for superior performance and safety.
  - We can integrate our own AI context right into the message processing pipeline.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
    TENANT ||--o{ INBOX : has
    TENANT ||--o{ CONTACT : has
    INBOX ||--o{ CONVERSATION : tracks
    INBOX ||--o| CHANNEL_ADAPTER : configured_with
    CONTACT ||--o{ CONVERSATION : participates_in
    CONVERSATION ||--o{ MESSAGE : contains
    MESSAGE }|--|| CONTACT : sent_by
    MESSAGE }|--|| AGENT : drafted_by
  ```

  **Key Entities (Rust / PostgreSQL):**
  - `Tenant`: Core row-level security boundary.
  - `Inbox`: Aggregates messages for a specific channel/purpose.
  - `ChannelAdapter`: Configs for WhatsApp, Instagram, Email, WebWidget.
  - `Contact`: Unified customer identity across channels.
  - `Conversation`: A thread of messages between a Contact and the Tenant.
  - `Message`: Individual message payloads (text, media).

  ### Mobile UX Flow (375px First)
  - **Unified Feed:** Cards showing unread messages or AI drafts awaiting approval.
  - **Interaction:** Tap a message card -> opens Conversation View.
  - **Conversation View:** Full history, unified across channels for that contact.
  - **AI Drafting:** "The Ambassador" draft sits at the bottom, above the keyboard, with "Approve" or "Edit" actions.
  - **Design Language:** Translucent Glass materials, native mobile keyboards, touch targets > 44x44px.

  ### AI Agent Integration Points
  - **Event Mesh:** Incoming messages via Webhook -> Event Mesh -> triggers `The Ambassador` agent.
  - **Proactive Drafting:** Agent queries `Contact` history and `Product Catalog`, drafts a `Message` (status: `pending_approval`), and pushes to the Mobile App Feed.

  ### Key Design Decisions
  - **Native Rust:** Replace Ruby on Rails (Chatwoot) with Rust for high concurrency, low latency, and memory safety.
  - **PostgreSQL RLS:** Row-level security on `tenant_id` enforced in the database layer.
  - **Proactive AI:** Shift from "read and type" to "read and approve".

  ## Implementation Prompt
  **User-Facing Outcome:** The owner opens the OHC mobile app and sees unified conversations from Instagram, WhatsApp, and Email. The AI Ambassador has pre-drafted highly contextual responses based on past purchases. The owner can tap "Approve" to send the draft immediately.

  **CUJ & Acceptance Criteria:**
  1. Define the PostgreSQL schema and Rust domain models for `Inbox`, `ChannelAdapter`, `Contact`, `Conversation`, and `Message` in `src/server/domain`.
  2. Ensure strict `tenant_id` integration and Row-Level Security (RLS) on all new tables.
  3. Implement the internal API (gRPC/REST) to create an Inbox, link a ChannelAdapter, and ingest a simulated message.
  4. Write comprehensive Unit Tests (100% coverage) for the Rust models and API handlers.
  5. Provide Playwright E2E tests verifying the creation of an Inbox and viewing a simulated conversation thread in the mobile-first UI.

  **Estimated Scope:** Large

  **Priority:** P0

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
