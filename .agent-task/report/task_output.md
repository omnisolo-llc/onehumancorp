issue_title: "Implement Native Rust Omnichannel Chat Engine"
issue_description: |
  **Problem Statement**:
  As per the OHC Engineering Standards, the legacy third-party chat dependency is 100% RETIRED. We must implement a high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`. Currently, the directory `src/server/integrations/chat` is mostly empty, and `src/server/services/chat` contains only a skeleton. We need to architect our custom open-source capabilities and implement them natively in our Rust backend. This enables our persona (e.g., Maya the baker or Carlos the handyman) to handle DMs, SMS, WhatsApp, and Web Chat in a unified, AI-triaged inbox, without managing third-party tools.

  **Research Report**:
  - The legacy chat integration must be fully removed and replaced with a native Rust implementation.
  - The new architecture centers around models like `Account`, `User`, `Inbox`, `Channel`, `Conversation`, `Message`, `Contact`.
  - OHC's backend requires a similar structure mapped to our multi-tenant PostgreSQL schema (`tenant_id` RLS).
  - WebSockets (via `gorilla/websocket` or Rust equivalent) are required for real-time web widget and mobile app synchronization.
  - Integration with the OHC Agent Triage queue is crucial for auto-drafting replies and suggesting next actions.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    graph TD
      A[Mobile App / Web Widget] <-->|WebSocket/REST| B(API Gateway / Rust Backend)
      C[WhatsApp / Instagram Webhooks] -->|HTTPS POST| B
      B --> D[(PostgreSQL - RLS Tenant Isolation)]
      B --> E[Redis / PubSub - Realtime Events]
      B <--> F[AI Agent Triage / Work Queue]
      D --> G(Conversations, Messages, Inboxes, Contacts)
    ```
  - **Mobile UX Flow**:
    - The first screen on 375px mobile shows the unified "Work Triage" feed. New messages from all channels appear here.
    - Tapping a message opens the Conversation View (macOS Translucent Glass style).
    - AI-drafted replies appear in a distinct visually clear state with a "Send Draft" button.
    - Offline support allows the owner to read cached messages and queue replies for when connectivity restores.
  - **AI Agent Integration Points**:
    - New incoming messages immediately trigger the `Work Triage` agent (via PostgreSQL `SKIP LOCKED` or Redis background queue).
    - The `Customer & Relationship Assistant` drafts replies based on tenant context (business info, past conversations, catalog).
    - Drafts are persisted and pushed to the client via WebSockets.
  - **Data Model (Rust Structs / DB Tables)**:
    - `Inbox`: Aggregates channels.
    - `ChannelAdapter`: Polymorphic configuration for WhatsApp, Instagram, Web.
    - `Conversation`: Thread between a Contact and the Inbox.
    - `Message`: Individual messages, including system events and AI drafts.
    - `Contact`: Customer profile linked across channels.

  **Implementation Prompt**:
  - Implement the core Rust data models and database migrations for the native omnichannel chat engine (`Inbox`, `Conversation`, `Message`, `Contact`, `ChannelAdapter`).
  - Ensure strict multi-tenant isolation using `tenant_id` on all tables.
  - Implement REST API endpoints for fetching inboxes, conversations, and sending messages.
  - Implement a WebSocket handler (using a Rust async framework like `axum` or `tungstenite` based on our stack) to push real-time message updates to clients.
  - Add integration tests (Playwright E2E and Rust unit tests) to verify a complete message flow from external webhook -> database -> websocket push -> AI draft generation.
  - Maintain the existing `src/server/services/chat/models.rs` and `service.rs` as the starting point.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
