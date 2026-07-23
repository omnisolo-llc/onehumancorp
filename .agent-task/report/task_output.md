issue_title: "[Research] OHC Native Rust Chat System & Chatwoot Replacement"
issue_description: |
  # Research Report: Native Rust Chat System for OHC

  ## 1. Problem Statement
  The mandate is to retire Chatwoot completely as an external dependency and implement its multi-tenant, omnichannel chat features natively in Rust within `onehumancorp/mono`. Currently, the system lacks a native implementation for critical Chatwoot features such as unified inbox, multi-channel adaptors (Email, Facebook Page, Instagram, WhatsApp, SMS, Web Widget), and advanced chat routing, SLA policies, macros, and AI agent integration. A first-class, high-performance native Rust implementation is needed to give the owner/operator a seamless, zero-config unified communication experience.

  ## 2. Research Report
  - **Chatwoot Source Audit:** The Chatwoot codebase relies heavily on Ruby on Rails patterns. Key entities identified are `Account`, `User`, `Inbox`, `Channel::*` (many adapters), `Conversation`, `Message`, and `Contact`.
  - **Data Model Translation:** These entities map to a multi-tenant PostgreSQL design. The `Account` maps to OHC's `tenant`. `Inbox` groups conversations from multiple `Channel` configurations.
  - **Real-time Needs:** Chatwoot relies on ActionCable/WebSockets. OHC will use its Rust API stack (tokio-tungstenite, axum ws, async-nats, redis) for real-time delivery and sync.
  - **Agent Routing & SLAs:** Rules engines and automated assignments are critical features to port, requiring background worker integration.
  - **Extensibility:** The Rust channel layer must be extensible to support arbitrary external providers (WhatsApp, Twilio, IG) safely in a multi-tenant context without cross-talk.

  ## 3. Design Doc (Architecture)

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client/Widget/Browser] -->|WebSocket/REST| B(OHC Rust Gateway API)
      B --> C{Channel Router}

      C -->|Email| D[Email Adapter]
      C -->|WhatsApp| E[WhatsApp Adapter]
      C -->|Web| F[Web Widget Adapter]
      C -->|Social| G[IG/FB Adapter]

      D --> H(Unified Inbox Service)
      E --> H
      F --> H
      G --> H

      H --> I[(PostgreSQL: Conversations/Messages/Contacts)]
      H --> J(Redis/NATS: Real-time PubSub)

      J --> K[AI Agent Assistant]
      K -->|Drafts/Replies| H
  ```

  ### Core Entities & Invariants (Multi-Tenant)
  - `tenant_id` mandatory on all tables (`conversations`, `messages`, `inboxes`, `contacts`). RLS enforced.
  - **Inbox:** Represents a unified view that groups multiple Channel instances.
  - **Channel:** Configuration specific to a provider (e.g., credentials, webhook tokens).
  - **Conversation:** Links a `Contact` to an `Inbox`.
  - **Message:** Belongs to a `Conversation`, sent by either `Contact` or `Agent/AI`.

  ### Mobile UX Flow (375px First)
  - **Work Triage Dashboard:** Shows pending conversations prioritized by SLA and VIP status.
  - **Unified Chat Thread:** Fluid scrollable view with AI drafts clearly delineated. Actions to approve/edit/send AI replies.
  - **Zero Config:** Channels are auto-provisioned or activated via 1-tap OAuth. No webhook manual configuration exposed to the user.

  ### AI Agent Integration
  - Agents subscribe to the NATS event bus for `message.created` and `conversation.updated`.
  - AI Assistant capability drafted messages appear in the timeline as "Agent Draft" waiting for owner approval, or auto-sent based on rules.

  ## 4. Implementation Prompt
  **Goal:** Implement the core database schema, gRPC/REST API definitions, and Rust service layer for the Unified Inbox system to replace Chatwoot, starting with the Web Widget channel.

  **Critical User Journey (CUJ):**
  1. Maya (Baker) logs into OHC on her phone.
  2. She navigates to the Inbox.
  3. A new message arrives via the Web Widget on her storefront ("Do you have vegan cakes?").
  4. The AI Assistant instantly drafts a reply based on her knowledge base.
  5. Maya reviews the draft, taps "Send", and the message is delivered via WebSocket back to the customer's widget.

  **Acceptance Criteria:**
  - Database schema definitions for `inboxes`, `channels`, `conversations`, `messages`, and `contacts` with full RLS multi-tenant isolation.
  - Rust API endpoints (or gRPC services) for creating conversations and sending/receiving messages.
  - Basic WebSocket structure for real-time delivery to the frontend.
  - Playwright E2E test verifying a message sent from the Web Widget appears in the Owner's Inbox and can be replied to.
  - All unit tests pass at 100% coverage. E2E tests pass reliably. No UI mock data.

  ## 5. Priority & Scope
  - **Priority:** P0 (Critical foundational architecture for Work Triage)
  - **Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
