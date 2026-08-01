issue_title: "Architecture & Native Rust Implementation Design: OHC Omnichannel Chat Engine"
issue_description: |
  # Native Rust Omnichannel Chat Engine

  ## Problem Statement
  OneHumanCorp (OHC) is designed to be the ultimate assistant for operators (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun), coordinating their messages, customers, and operations. Currently, we lack a unified, native, high-performance inbox system. We are mandated to **retire Chatwoot completely** and build a native omnichannel customer support and chat engine directly in Rust within `onehumancorp/mono`. A disconnected third-party tool violates our core value of "Radical Simplicity" and forces operators to juggle contexts.

  ## Research Report
  ### Discovery & Benchmarking
  - **Source Code Audit of Chatwoot**: Cloned and examined `https://github.com/chatwoot/chatwoot`. Chatwoot uses a monolithic Rails architecture heavily reliant on PostgreSQL (for core models like `Account`, `Inbox`, `Conversation`, `Message`, `Contact`), Redis (for Sidekiq job queues and WebSocket pub/sub via ActionCable), and various channel adapters (Email, WhatsApp, Facebook, Twilio).
  - **Identified Gaps**:
    - The current OHC setup lacks a robust, natively integrated data model to represent multi-channel conversations.
    - Real-time WebSocket infrastructure for immediate message delivery is missing natively in the Rust backend.
    - AI Agent hook-ins into the message lifecycle (e.g., auto-replying to Maya's DMs) need a structured event bus.

  ## Design Doc
  ### Data Model & Invariants (Multi-Tenant)
  We will map Chatwoot's core entities into our Rust/SeaORM schema with strict row-level security (`tenant_id`):
  - **Inbox**: Represents a channel endpoint (e.g., "Carlos's WhatsApp", "Maya's IG").
  - **Contact**: The customer communicating with the business.
  - **Conversation**: A thread of messages between a Contact and an Inbox.
  - **Message**: Individual payloads (text, images, templates) within a Conversation.
  - **ChannelAdapter**: Configuration for connecting to external APIs (Twilio, Meta, Stripe for payments in chat).

  ### AI Department Coordination
  - **Work Triage / Customer Assistant**: Listens to new `Message` events via PostgreSQL `SKIP LOCKED` job queue. Drafts replies based on memory and writes back pending `Message` objects for operator approval or auto-sends based on confidence.

  ### Mobile-First UX Flow
  - **375px First**: The UI will feature a consolidated "Inbox" tab. Cards representing `Conversations` will show the channel icon (e.g., IG, Email), unread status, and AI draft previews.
  - Tapping a card opens the chat thread. The input area uses native mobile keyboards.
  - Translucent glass materials and UniFi-style status tokens will indicate message state (Sent, Delivered, Read, AI Draft).

  ## Implementation Prompt
  **Goal:** Implement the core database schemas and Rust entity models for the native Omnichannel Chat Engine, replacing Chatwoot dependencies.

  **Tasks:**
  1. Create SeaORM migration scripts and entity definitions in `src/server/entities/` (or the appropriate domain folder) for `Inbox`, `Contact`, `Conversation`, and `Message`.
  2. Ensure every table includes a `tenant_id` column and respects multi-tenant invariants.
  3. Implement a basic REST/gRPC API service layer to list and create these entities.
  4. Build a robust test suite covering multi-tenant isolation.

  **Acceptance Criteria:**
  - `bazel test //...` passes 100%.
  - No dependencies on Chatwoot exist.
  - A real operator persona (e.g., Maya) can (via API testing) create a contact and receive a simulated conversation thread tied only to her tenant.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
