issue_title: "Architecture Design: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  **Problem Statement**
  Currently, OHC lacks a native, fully integrated omnichannel communication system that guarantees data sovereignty, multi-tenant isolation, and zero-trust security without relying on external providers like Chatwoot. We must replace Chatwoot entirely with a high-performance, native Rust-based omnichannel chat architecture that brings all customer communications (Instagram DMs, WhatsApp, Web Chat, SMS) into a single unified inbox designed for mobile-first owner personas (Maya, Carlos, Priya, etc.).

  **Research Report**
  - **Chatwoot Source Code Audit (`https://github.com/chatwoot/chatwoot`)**:
    - Built on Ruby on Rails, ActionCable (WebSockets), PostgreSQL, and Redis.
    - Core entities: `Accounts` (Tenants), `Inboxes`, `Channels`, `Conversations`, `Messages`, `Contacts`, and `Users` (Agents).
    - Event-driven: Webhooks and external APIs push events to the controller, which routes them to the correct inbox and broadcasts updates over WebSockets.
  - **OHC Architecture Requirements**:
    - Built natively in Rust within the `onehumancorp/mono` repository.
    - Multi-tenant isolation enforced at the PostgreSQL row-level (RLS).
    - High-performance WebSocket server (e.g., Tokio + Axum/Tungstenite) to replace ActionCable.
    - Redis for pub/sub real-time event broadcasting and ephemeral connection state.
    - Deep, invisible AI integration (Customer Assistant) to automatically draft replies based on owner context.

  **Design Doc**
  - **Architecture Diagram**:
    ```mermaid
    graph TD
        Client[Mobile/Web Client] -->|HTTPS/WSS| API_Gateway
        API_Gateway -->|Routing| Chat_Service[Native Rust Chat Service]
        Chat_Service -->|Pub/Sub| Redis[Redis Cache/PubSub]
        Chat_Service -->|Read/Write| Postgres[(PostgreSQL - RLS Enabled)]
        Chat_Service -->|Events| AI_Worker[AI Agent Job Queue]
        Webhooks[External: IG, WA, SMS] --> API_Gateway
    ```
  - **Data Models & Invariants**:
    - `Tenant` (Owner Workspace - Isolation Boundary).
    - `Inbox` (Collection of conversations, mapped to specific channels).
    - `ChannelAdapter` (Configurations for WhatsApp, WebWidget, IG).
    - `Contact` (Customer interacting with the owner).
    - `Conversation` (Message thread between Contact and Owner).
    - `Message` (Individual message payload with attachment support).
  - **Mobile UX Flow (375px)**:
    - Bottom navigation "Inbox" tab.
    - List of active conversations, automatically sorted by AI-determined urgency.
    - Tapping a conversation opens the chat view with AI-suggested draft replies rendered as a translucent glass card just above the mobile keyboard.
    - "Accept Draft" action button, or native mobile keyboard for manual entry.
    - All elements must use OHC Premium Token library with Apple/Ubiquiti-style hierarchy.
  - **AI Agent Integration**:
    - The Customer Assistant agent subscribes to `MessageCreated` events.
    - It processes the message and owner context, then updates the `Conversation` with a `draft_reply`.
    - The Draft is instantly pushed via WebSocket to the client.

  **Implementation Prompt**
  As the Implementer agent, your task is to build the backend foundation and the 375px mobile-first frontend for the Native Rust Omnichannel Chat System.
  - **Backend**: Implement the core Rust data models (`Conversation`, `Message`, `Inbox`, `Contact`) with PostgreSQL row-level security. Set up an Axum WebSocket endpoint that broadcasts new messages to connected clients based on their `tenant_id`.
  - **Frontend**: Implement a premium (translucent glass) Inbox UI in Flutter/PWA that connects to the WebSocket. Display a list of conversations and an active chat view.
  - **AI Integration**: Implement a background worker that generates a simple AI draft reply when a customer message is received.
  - **Acceptance Criteria**: A user can open the app on a 375px screen, receive a message via a mock webhook/API call, see it appear in real-time via WebSocket without refreshing, and see an AI-generated draft reply ready to be sent. Ensure 100% test coverage and Playwright E2E verification.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
