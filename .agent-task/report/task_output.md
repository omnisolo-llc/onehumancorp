issue_title: "Architect & Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Mission Queue Protocol: Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) is transitioning away from Chatwoot as an external third-party service to a native Rust, high-performance omnichannel chat system built directly into the `onehumancorp/mono` platform. The owner/operator personas (Maya, Carlos, Priya) currently face fragmented communications across Instagram, WhatsApp, email, and web widgets. They need a unified inbox that brings all customer communications into one place where AI agents (Customer Assistant, Operations Assistant) can read context, draft replies, and take action without the owner navigating between different apps. The gap holds back our core promise of a "unified work assistant."

  ## Research Report & Gap Analysis
  Based on a source code audit of Chatwoot (`https://github.com/chatwoot/chatwoot`), the core architecture necessary for an omnichannel support system involves:
  - **Inboxes & Channels:** A polymorphic relationship where an `Inbox` belongs to a specific `Channel` (e.g., WhatsApp, Email, Web Widget, API).
  - **Conversations & Messages:** Conversations group messages between a `Contact` and an `Inbox`. Messages support rich payloads (text, attachments, interactive buttons).
  - **WebSockets & Real-time:** Real-time dispatch of events to connected clients (owners/staff) and web widget users.
  - **Agent Routing & AI:** The ability for AI agents to participate as "bot agents" in the conversation flow, handing off to humans when necessary.

  **Competitor Insights (Shopify Inbox, Wix Inbox, Chatwoot):**
  These platforms succeed because they abstract the complexity of underlying networks (Meta APIs, email protocols) into a single, predictable UI for the business owner. OHC's differentiator is that our AI agents actively read this stream and propose drafts or actions (e.g., booking a service from an Instagram DM) rather than just waiting for the owner to reply.

  ## Design Doc (Architecture & AI Coordination)
  ### High-Level Architecture
  The system will be implemented as native Rust modules within the `src/server` and a Flutter frontend (`src/ui`).

  **Mermaid Diagram:**
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : contains
      INBOX ||--o{ CHANNEL : configures
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      TENANT ||--o{ CONTACT : manages

      MESSAGE {
          uuid id
          uuid conversation_id
          uuid sender_id
          string content
          string content_type
          jsonb metadata
          datetime created_at
      }

      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status
          uuid assignee_id
      }

      INBOX {
          uuid id
          string name
          string channel_type
      }
  ```

  ### Multi-Tenant Data Model & Invariants
  - **Row-Level Security (RLS):** Every table (`inboxes`, `conversations`, `messages`, `contacts`, `channel_*`) MUST have a `tenant_id` UUID column. PostgreSQL RLS policies will enforce strict isolation so no tenant can ever read another's messages.
  - **Distributed Locks:** Redis Redlock (`ohc:lock:{tenant_id}:conversation:{conversation_id}`) will be used for concurrent AI agent drafts to prevent race conditions.
  - **Background Queues:** A PostgreSQL `SKIP LOCKED` job queue will handle incoming webhooks (e.g., from WhatsApp/Instagram) to ensure reliable message processing.

  ### AI Department Coordination
  - **Customer Assistant Agent:** Listens to new `messages` in `conversations` where `status = unassigned`. Uses tenant memory to draft replies.
  - **Operations Assistant Agent:** Scans messages for booking/service intent. If found, proposes a calendar slot and drafts the reply for the Customer Assistant.
  - **Workflow:** Agents do NOT send messages automatically by default. They append a `draft` message to the conversation. The owner reviews the draft in the UI and taps "Send".

  ### Mobile-First UX Flow (375px)
  - **Unified Feed (Home Screen):** The owner opens the app and sees "Action Items". New unread conversations or AI-drafted replies appear as cards.
  - **Conversation View:** A familiar chat interface. Clean Apple-style translucent top nav. Message bubbles. At the bottom, instead of just a keyboard, the AI draft is presented as a floating card above the input field with a 1-tap "Send" or "Edit" button.
  - **Performance Targets:** The initial screen must render from a local SQLite cache (via PowerSync) within 200ms. Offline creation of drafts must queue locally and sync when the network returns.

  ## Implementation Prompt (For Implementer Agents)
  **Objective:** Implement the core database schema, Rust service layer, and basic unified inbox UI for the native OHC chat system.

  **CUJ (Critical User Journey):**
  1. The owner (Maya) logs into the OHC mobile app.
  2. She navigates to the "Inbox" tab.
  3. She sees a list of conversations (from mocked web widget or API channel).
  4. She opens a conversation, views a message from a contact, and types a reply.
  5. The reply is saved and broadcasted via WebSocket.

  **Acceptance Criteria:**
  - Create the PostgreSQL schema migrations for `inboxes`, `channels`, `contacts`, `conversations`, and `messages` with `tenant_id` and RLS enabled.
  - Implement the Rust gRPC/REST API endpoints for fetching conversations and sending messages.
  - Implement a mobile-first (375px) Flutter UI for the unified inbox list and conversation detail view following the OHC translucent glass design system.
  - Ensure 100% unit test coverage for the Rust service logic.
  - Write at least 5 Playwright E2E tests validating the conversation list and message sending flows (no mocked backend calls).
  - DO NOT implement third-party vendor integrations (Meta, WhatsApp) in this PR; use a generic "API/Custom" channel adapter for now.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []