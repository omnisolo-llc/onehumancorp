issue_title: "[Platform] Native Rust Omnichannel Chat & Inbox System (Chatwoot Replacement)"
issue_description: |
  **Problem Statement**
  OneHumanCorp (OHC) is currently lacking a native, high-performance omnichannel chat system. Relying on external systems like Chatwoot creates architectural fragmentation, increases operational complexity, and prevents the tight integration required for our AI agents to seamlessly participate in conversations, observe context, and assist owners in real-time. We need to implement a native Rust-based omnichannel chat architecture directly within the OHC platform to provide unified messaging, inbox management, and agent routing for our users (Maya, Carlos, Priya, Leo, Fatima).

  **Research Report**
  We have benchmarked the open-source Chatwoot data model (conversations, messages, inboxes, contacts, channel adapters) as a baseline. The Chatwoot system relies heavily on a standard conversational data model:
  - `Conversation`: The central entity grouping messages between a contact and a business/team.
  - `Message`: Individual units of communication within a conversation.
  - `Inbox`: The queue or channel entry point where conversations arrive.
  - `Contact`: The end-customer initiating or participating in the conversation.
  - `Channel`: The specific medium (e.g., Web Widget, WhatsApp, Instagram DM, SMS).

  Competitor Analysis:
  - **Shopify Inbox**: Deeply integrated with store data (products, orders), allowing seamless sharing of context during chats. Our native solution needs similar deep integration with OHC's operational data.
  - **Intercom / Zendesk**: Highly flexible routing and bot automation. Our system must be designed from day one to allow our AI agents (Customer & Relationship Assistant, Operations Assistant) to seamlessly join, observe, and draft replies within conversations.

  By building this natively in Rust within `onehumancorp/mono`, we gain:
  - Strict multi-tenant isolation via OHC's existing tenant boundaries.
  - High performance and lower memory footprint compared to Rails-based alternatives.
  - Zero-trust security model integration (SPIFFE/SPIRE).
  - Deep, unified integration with our AI Job Queue (PostgreSQL SKIP LOCKED) for agent actions.

  **Design Doc**
  *Architecture Overview*
  The new Native Rust Omnichannel Chat system will consist of several core components:
  1.  **Core Chat Service (Rust)**: Manages the lifecycle of Inboxes, Conversations, Messages, and Contacts. It exposes gRPC endpoints for internal services (e.g., AI agents) and REST/WebSocket APIs for the client facing apps.
  2.  **WebSocket Gateway (Rust)**: A scalable, real-time gateway handling persistent connections from the Flutter frontends and external web widgets. It will broadcast events (new messages, typing indicators, presence) to connected clients.
  3.  **Channel Adapters (Rust)**: Modular components responsible for translating external platform webhooks/APIs (WhatsApp, Instagram, Email, Web Widget) into the unified internal `Message` format, and vice versa.
  4.  **AI Integration Layer**: A specialized pub/sub mechanism (leveraging PostgreSQL LISTEN/NOTIFY or Redis) allowing OHC's AI agents to observe incoming messages and inject drafted replies directly into the conversation stream.

  *Mermaid.js Diagram*
  ```mermaid
  graph TD
      Client[Flutter App / Web Widget] <-->|WebSocket| WSG[WebSocket Gateway]
      Client <-->|REST| API[Core Chat API]
      Ext[WhatsApp / IG / Email] -->|Webhooks| Adapters[Channel Adapters]
      Adapters <--> API
      WSG <--> API
      API <--> DB[(PostgreSQL)]
      API <--> Redis[(Redis)]
      API -->|Events| AILayer[AI Integration Layer]
      AILayer <-->|Drafts/Actions| AIAgents[OHC AI Agents]
  ```

  *Mobile UX Flow (375px First)*
  1.  **Inbox List (Work Triage)**: A unified list of all active conversations, prioritized. Unread indicators and agent-drafted reply indicators are clearly visible. Touch targets are large (44x44px).
  2.  **Conversation View**: A standard chat interface. Messages are bubbled. The input area supports typing, but more importantly, it prominently displays agent-suggested replies or actions (e.g., "Send Quote", "Request Deposit") based on the context.
  3.  **Context Panel (Slide-over)**: Swiping left reveals the Contact's history, previous orders/bookings, and preferences, allowing the owner to make informed decisions without leaving the chat.
  4.  **Design Language**: OHC Premium Token library (macOS-style Translucent Glass, clear typography).

  *AI Agent Integration Points*
  - **Work Triage Agent**: Monitors new conversations and categorizes/prioritizes them in the Inbox List.
  - **Customer Assistant**: Subscribes to conversation events. Upon a new message, it analyzes intent and optionally drafts a reply, inserting it into the conversation as a "draft" state pending owner approval.
  - **Operations Assistant**: Scans messages for intent related to bookings or services and surfaces actionable UI elements (e.g., a "Create Booking" button) directly in the conversation view.

  **Implementation Prompt**
  *Target*: Implementer Agent (Backend & DB focus)
  *User-Facing Outcome*: As a business owner (like Maya or Carlos), I need a unified inbox where I can see and respond to messages from all my channels (Web, IG, WhatsApp) in one place, with my AI assistant seamlessly suggesting replies and actions.
  *Acceptance Criteria*:
  1. Define the PostgreSQL schema for the core entities (`inboxes`, `conversations`, `messages`, `contacts`, `channel_adapters`). Ensure strict multi-tenant isolation (`tenant_id` on all tables, Row Level Security enabled).
  2. Implement the core CRUD logic and gRPC service definitions for these entities in Rust within the OHC mono-repo structure.
  3. Design and implement the WebSocket Gateway foundation for real-time event distribution (new messages).
  4. Create a foundational generic `ChannelAdapter` trait/interface that specific channel integrations (e.g., Web Widget) can implement.
  5. Provide comprehensive unit tests (100% coverage) for the new Rust modules.
  6. Ensure all schema definitions and API endpoints align with the goal of complete Chatwoot feature replication, but optimized for the OHC architecture.

  **Priority**: P0 (Critical - Foundational Architecture)
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
