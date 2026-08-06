issue_title: "[Architecture] Native Rust Omnichannel Inbox to Replace Chatwoot"
issue_description: |
  # Problem Statement
  Small business owners need to manage inquiries across multiple channels (Instagram, WhatsApp, SMS, Email). The previous approach relied on an external third-party service (Chatwoot). This external dependency breaks the "Zero Trust & Security" and multi-tenant isolation guarantees, adds latency, and forces context-switching or complex sync mechanisms between OHC and Chatwoot.

  As mandated by OHC Engineering Standards, Chatwoot must be fully retired and replaced with a native Rust omnichannel customer support and chat engine inside the OHC monolithic repository (`onehumancorp/mono`).

  # Research Report
  Based on auditing the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) and the OHC product vision:
  - **Data Models:** Chatwoot relies on `Account` (Tenant), `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message` models. We need strict multi-tenant Row-Level Security (RLS) equivalents in our PostgreSQL schema and native models.
  - **Omnichannel Support:** Chatwoot uses different "Channel Adapters" for Twitter, Facebook, Email, API, etc. Our Native Rust implementation needs a modular approach to handle different channel types.
  - **Real-time Messaging:** Chatwoot uses ActionCable (WebSockets). We need a high-performance native WebSocket implementation combined with a distributed event backplane for real-time events.
  - **AI Integration (The Ambassador):** Unlike Chatwoot's basic macro/automation rules, OHC's native inbox must deeply integrate with our AI Event Mesh, allowing The Ambassador Agent to proactively draft replies by querying the Unified Customer Graph DB.

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      Client[Customer Channels: Insta, Web, SMS] -->|Webhooks/API| Gateway[OHC Rust API Gateway]
      Gateway --> Auth[SPIFFE/SPIRE Identity & Auth]
      Auth --> ChannelAdapter[Native Channel Adapters]
      ChannelAdapter --> InboxService[Omnichannel Inbox Service]

      InboxService -->|Database| DB[(PostgreSQL with RLS)]
      InboxService -->|Publish| EventMesh[Distributed Event Mesh]

      EventMesh -->|Subscribe| WSServer[Real-time Event Server]
      WSServer -->|Real-time| OwnerApp[OHC Mobile PWA/Flutter 375px]

      EventMesh -->|Trigger| AmbassadorAgent[The Ambassador AI Agent]
      AmbassadorAgent -->|Query| Graph[Unified Customer Graph]
      AmbassadorAgent -->|Draft Reply| InboxService
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Triage Feed:** The main screen is a prioritized feed of action items.
  - **Drafted Reply Card:** An incoming message from WhatsApp shows up as a card: "New Message from Carlos (WhatsApp)". Below it, the AI draft is already visible: "Suggested Reply: Hi Carlos...".
  - **Actions:** Prominent "Send Draft" button. A "Modify" button opens a native keyboard view.
  - **Chat Thread View:** Tapping the card (not the action button) opens the full conversational history, blending past messages, system notes (e.g., "Invoice #123 Paid"), and channel indicators (Instagram icon, SMS icon).

  ### Mobile UX Parity
  - Translucent glass UI components for the message bubbles.
  - Fast, optimistic UI updates for sending messages (displaying a muted/sending state immediately).
  - Real-time events must gracefully handle reconnection and offline caching for the owner's mobile device.

  ### AI Agent Integration
  - The core messaging service emits an event on every incoming customer message.
  - The Ambassador Agent listens to these events, retrieves the customer's full context, and creates a drafted message.

  # Implementation Prompt
  **User-Facing Outcome:** The owner opens the OHC app and sees a unified inbox handling Instagram, Web Chat, and Email without any external third-party branding or latency. AI drafts are instantly available for new messages.

  **CUJ & Acceptance Criteria:**
  1. **Schema Migration:** Implement PostgreSQL database migrations for required conversational models with strict Row-Level Security isolation.
  2. **Core Service Implementation:** Implement the omnichannel backend service for CRUD operations on the above entities.
  3. **Channel Adapter Integration:** Implement a mechanism to support a "Web Widget" channel adapter as the first proof-of-concept.
  4. **Real-time Event Integration:** Implement a persistent connection endpoint in the backend to stream new messages to the frontend.
  5. **Agent Trigger:** Ensure that when a new message arrives via a channel, it is published to the event mesh so the AI agent can draft a reply.
  6. **E2E Test:** Playwright E2E test where an owner creates an inbox, a simulated customer sends a message via the API, the owner sees the message appear in the UI via the real-time stream, and the owner replies.

  **Priority:** P0
  **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
