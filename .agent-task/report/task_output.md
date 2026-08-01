issue_title: "Native Rust Chat Engine: Chatwoot Replacement"
issue_description: |
  # Native Rust Chat Engine (Chatwoot Replacement)

  ## Problem Statement
  OneHumanCorp (OHC) mandates the complete retirement of Chatwoot as an external service. Small business owners like Maya (baker) and Carlos (handyman) need a unified inbox that brings together SMS, Email, Instagram DMs, and Web Chat. They expect this unified inbox to be robust, fast, and native to OHC without relying on third-party SaaS for the core messaging infrastructure. The current architectural gap is the lack of a high-performance, multi-tenant native Rust omnichannel chat system within `onehumancorp/mono`.

  ## Research Report
  - **The Gap**: External dependencies for core customer interactions (like Chatwoot) introduce latency, potential security risks, and break the seamless multi-tenant architecture OHC requires.
  - **Competitive Audit (Chatwoot)**: Chatwoot provides an excellent model for an omnichannel inbox, including Conversation, Message, Contact, and Inbox models, plus channel adapters. However, building this natively in Rust allows us to leverage OHC's existing multi-tenant PostgreSQL (Row Level Security), Redis PubSub for real-time WebSockets, and AI agent integrations (Customer Assistant) much more tightly and securely.
  - **Persona Need**: Maya needs to see Instagram DMs alongside website inquiries in one place, instantly. Carlos needs SMS quotes to appear in the same thread as his email follow-ups.

  ## Design Doc
  ### Architecture Diagram

  ```mermaid
  graph TD
      subgraph Frontend "Flutter App (Mobile-First)"
          UI[Unified Inbox UI]
          WSClient[WebSocket Client]
      end

      subgraph Backend "Rust + Bazel Backend"
          API[Chat API gRPC/REST]
          WSServer[WebSocket Server]
          Router[Message Router]
          Adapters[Channel Adapters: SMS, Email, IG, Web]
      end

      subgraph AI "AI Agent Departments"
          CustomerAsst[Customer Assistant Agent]
      end

      subgraph Data
          DB[(PostgreSQL - Tenant DB RLS)]
          Redis[(Redis PubSub)]
      end

      UI --> API
      WSClient <--> WSServer

      API --> DB
      WSServer <--> Redis
      Router <--> Redis
      Router --> DB

      Adapters --> Router
      Router --> Adapters

      Router <--> CustomerAsst
  ```

  ### Mobile UX Flow (375px First)
  1.  **Unified Feed**: The main screen is a vertical list of conversations across all channels. Each item shows an avatar, the channel icon (SMS, Web, IG), the contact name, and a snippet of the latest message.
  2.  **Conversation View**: Tapping a conversation opens a standard chat interface. A prominent text input area at the bottom uses the native keyboard.
  3.  **Agent Integration**: Within the chat view, an "AI Draft" button allows the user to immediately summon the Customer Assistant to draft a reply based on context.
  4.  **Channel Indication**: Clear visual cues indicate which channel the current reply will be sent via.

  ### AI Agent Integration
  -   The `Customer Assistant` agent listens to the Message Router (via Redis or NATS) for new incoming messages.
  -   It can automatically draft replies (placed in a pending state for owner approval) or auto-respond if configured by the owner for simple queries (e.g., "What are your hours?").

  ### Key Design Decisions
  -   **Data Models**:
      -   `Inbox`: Represents a collection of channels (e.g., "Support Inbox", "Sales Inbox").
      -   `Channel`: Represents a specific integration (e.g., a specific Twilio number, a specific IG account).
      -   `Contact`: The external user.
      -   `Conversation`: A thread of messages between a Contact and an Inbox, potentially spanning multiple Channels over time.
      -   `Message`: The individual communication unit.
  -   **Multi-Tenancy**: All tables must have `tenant_id` and RLS policies enforced.
  -   **Real-time**: Redis PubSub handles distributing WebSocket events to the correct connected client instances.

  ## Implementation Prompt
  **Objective**: Implement the core data models, REST/gRPC API, and real-time WebSocket infrastructure for the Native Rust Chat Engine, replacing Chatwoot.

  **Critical User Journey (CUJ)**:
  1.  As a business owner, I receive a new message via a mocked channel adapter (e.g., a simple HTTP POST simulating an incoming SMS webhook).
  2.  The system creates or updates a Contact and a Conversation.
  3.  The message is saved to the database with the correct tenant context.
  4.  A WebSocket event is broadcasted.
  5.  The owner's mobile app (or a simulated test client) receives the real-time event and updates the UI to show the new message.

  **Acceptance Criteria**:
  -   Define `Conversation`, `Message`, `Contact`, `Inbox`, and `Channel` models in Rust (using SeaORM or sqlx) with strict `tenant_id` isolation.
  -   Implement API endpoints to fetch inboxes, conversations, and messages.
  -   Implement a WebSocket server capable of authenticating a tenant and subscribing to new message events via Redis PubSub.
  -   Provide a basic "dummy" channel adapter to inject test messages.
  -   100% unit test coverage for the new API and routing logic.

  ## Priority & Scope
  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
