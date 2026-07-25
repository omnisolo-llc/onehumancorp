issue_title: "Architecture Design: Omnichannel Native Chat & Conversational Inbox System (Chatwoot Replacement)"
issue_description: |
  **Title**: Complete Architecture & Implementation Plan for OHC Native Rust Omnichannel Chat System

  **Problem Statement**:
  OneHumanCorp (OHC) historically relied on an external third-party dependency (Chatwoot) to provide customer communication, inbox management, and multi-channel messaging capabilities. This violates the core design requirements of a secure, multi-tenant, locally runnable AI work assistant for owners and operators. It creates a disjointed operational experience, preventing AI agents (e.g., Maya's Instagram DM bot or Carlos's automated quote generator) from natively reading, coordinating, and interacting within a unified conversation context without external network hops and external data synchronization. We need a native Rust implementation of these core features built directly into `onehumancorp/mono`.

  **Research Report**:
  - **Goal**: Retire Chatwoot 100% and build a native, high-performance omnichannel inbox system in Rust.
  - **Source Code Audit (chatwoot/chatwoot)**: Analyzed Chatwoot's Postgres schema models:
    - **Inboxes**: Defines the channel type, greeting messages, working hours, and auto-assignment configurations.
    - **Conversations**: Tracks conversation states, contacts, assigned agents (or AI bots), SLA policies, and priority.
    - **Messages**: Stores content (with attributes like sender type, attachments, private notes).
    - **Contacts**: Represents end users communicating across channels.
  - **Competitor Insights**: Like Shopify Inbox, Zendesk, or Zendesk Support, OHC needs robust support for multi-channel message aggregation (Web Widget, Email, Instagram DMs, WhatsApp). For OHC, integrating our SPIFFE/SPIRE agent models as "first-class assignees" inside conversations is crucial.

  **Design Doc (Architecture Proposal)**:
  - **Data Model (Rust/Postgres)**:
    - `ohc_inbox`: Maps to tenant configurations for different channels.
    - `ohc_conversation`: Tenant-isolated session representing a thread of messages with a specific contact.
    - `ohc_message`: Individual communication blobs (text/media) associated with a conversation.
    - `ohc_contact`: End-user identity across channels.
  - **Architecture Diagram (Mental Model)**:
    - [Channel Webhooks] -> [Rust API Gateway (gRPC/REST)] -> [Message Queues (Redis)] -> [Conversation Service (Rust)] -> [Postgres DB (Tenant Isolated RLS)].
    - AI Agents listen to `ohc_conversation` events and can post `ohc_message` drafts directly.
  - **Mobile UX Flow (375px First)**:
    - App opens to a unified "Inbox" tab.
    - List of unread conversations (cards showing sender, channel icon, preview text, time).
    - Tapping a conversation opens a mobile-optimized chat view with bottom input (native keyboard support) and a "Drafted by AI" translucent overlay when the agent pre-fills a response.
  - **AI Agent Integration Points**:
    - AI Agents (via `ohc-builtin-agent`) subscribe to new `ohc_message` events in Redis for their tenant.
    - Agents generate replies and post them back via API as `message_type: 'bot_draft'` or direct replies based on tenant settings.

  **Implementation Prompt**:
  Implement the Core Database Schema (Rust/Postgres migrations) and gRPC/REST service boundaries for the new OHC Native Inbox system.
  1. Define and execute SQL migrations for `inboxes`, `conversations`, `messages`, and `contacts` with strict `tenant_id` Row-Level Security (RLS).
  2. Implement the basic CRUD API endpoints in Rust for these entities.
  3. Ensure a UI skeleton using Flutter (or Playwright-testable frontend) exists to list unread conversations on a 375px mobile view.
  4. Integrate the message creation API with the existing Redis pub/sub mechanism to emit events for AI agent consumption.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
