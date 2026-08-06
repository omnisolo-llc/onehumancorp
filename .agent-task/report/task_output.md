issue_title: "[Research] Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Problem Statement

  Small business owners (Maya the baker, Carlos the handyman) operate across multiple fragmented messaging platforms (WhatsApp, Instagram, web widgets). Currently, OHC relies on a legacy external dependency (Chatwoot) for omnichannel capabilities, which introduces architectural complexity, latency, and challenges in enforcing strict multi-tenant isolation (Zero Trust/SPIFFE/SPIRE) and seamless integration with OHC's native AI agents. We need to retire this external dependency and build a native, high-performance Rust omnichannel chat engine directly into the OHC monolith.

  # Research Report

  **Findings & Feature Benchmarking (Chatwoot Audit):**
  - **Data Models:** Chatwoot's core entities include Account (Tenant), User, Inbox, Channel (WhatsApp, WebWidget), Contact, Conversation, and Message.
  - **Channel Adapters:** E.g., `Channel::Whatsapp` handles provider configs and webhooks; `Channel::WebWidget` manages website tokens and hmac validation.
  - **Real-time:** Depends heavily on WebSockets for web widget and internal dashboard updates.
  - **Automation:** Supports Macros, Canned Responses, and Agent Bot assignment, but these are rigidly programmatic.

  **OHC Architecture Gap:**
  - OHC needs to replicate the core data structures (Conversation, Message, Inbox, Contact, Channel Adapters) but natively in Rust.
  - Instead of legacy "Agent Bots" or static "Canned Responses," OHC's native chat engine must inherently integrate with our AI Event Mesh, specifically triggering `The Ambassador Agent` (Customer Success) for context-aware proactive drafting.
  - **Multi-Tenancy:** We must enforce strict Row Level Security (RLS) via `tenant_id` at the database level and Zero Trust isolation at the application layer, avoiding the complexities of mapping an external tool's tenancy model to ours.

  # Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      subgraph External Channels
          A[Meta WhatsApp Webhook]
          B[Instagram Webhook]
          C[Web Widget WebSocket]
      end

      subgraph Native Rust Chat Engine (OHC Mono)
          D[Omnichannel Gateway / Ingress]
          E[Identity Resolution]
          F[Conversation Manager]
          G[(Unified Customer Graph DB)]
      end

      subgraph OHC AI Mesh
          H[Event Bus]
          I[The Ambassador Agent]
      end

      A --> D
      B --> D
      C --> D
      D --> E
      E -->|Lookup / Create Contact| G
      E --> F
      F -->|Persist Message| G
      F -->|Publish Event| H
      H --> I
      I -->|Draft Reply| F
  ```

  ### Data Model (Core Entities)
  - `Tenant` (`Account`)
  - `Inbox`: Aggregates channels for a business context.
  - `ChannelAdapter`: Configs for WhatsApp, Web Widget, etc.
  - `Contact`: Unified customer identity across channels.
  - `Conversation`: Thread between Contact and Inbox.
  - `Message`: Individual message unit with `sender_type` (Contact, Agent, Bot).

  ### Mobile UX Flow (375px First)
  - **Work Triage Feed:** The home feed acts as the unified inbox. New conversations appear as priority cards.
  - **Agent Handoff:** When `The Ambassador Agent` drafts a reply, the card shows the message context and the AI draft.
  - **One-Tap Action:** The owner (e.g., Carlos) taps a single button to "Approve & Send" the draft.
  - **Mobile Constraints:** No horizontal scrolling; large tap targets for the "Send" and "Edit" buttons. The UI must elegantly handle offline states and optimistic UI updates for message sending.

  ### AI Agent Integration Points
  - **Event Trigger:** Every incoming `Message` creation publishes a `message.created` event to the Event Mesh.
  - **The Ambassador:** Subscribes to `message.created`. Queries the `Unified Customer Graph DB` for the contact's purchase history and active orders.
  - **Drafting:** The Ambassador inserts a draft message into the `Conversation` or `ActionRequiredQueue` for owner approval, rather than auto-replying in most SMB scenarios.

  ### Key Design Decisions
  - **Rust Native:** Eliminates external network hops and dependency management for the core chat loop.
  - **Tenant Isolation:** Every new table (`conversations`, `messages`, `inboxes`) MUST have a `tenant_id` column and enforce RLS in PostgreSQL.
  - **Event-Driven AI:** The chat engine itself is dumb; it just routes messages and fires events. The intelligence lives in the OHC AI Mesh, maintaining separation of concerns.

  # Implementation Prompt

  **User-Facing Outcome:** As an owner like Maya, I want to receive a WhatsApp message from a customer and see it immediately in my OHC app, along with a perfectly drafted AI response based on that customer's past cake orders, all without needing to configure or log into a separate chat tool.

  **CUJ & Acceptance Criteria:**
  1. Define and implement the PostgreSQL database schemas for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`, strictly enforcing multi-tenant isolation (`tenant_id`).
  2. Implement a Rust service layer for the `Omnichannel Gateway` capable of receiving a simulated webhook payload (e.g., from WhatsApp).
  3. The gateway must resolve the contact (create or find) and persist the incoming message to the database.
  4. The gateway must emit an event to the internal event bus signaling a new message.
  5. **Automated Verification:** Write unit tests for the Rust service layer with 100% coverage. Write Playwright E2E tests simulating an incoming webhook, logging in as a user, and verifying the new message appears in the UI.

  # Scope & Priority
  - **Priority:** P0 (Critical for retiring external dependencies and unblocking native AI features).
  - **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []