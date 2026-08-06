issue_title: "[Architecture] Native Rust Omnichannel Chat (legacy chat engine Replacement)"
issue_description: |
  ## Problem Statement
  OHC is retiring the external legacy chat engine dependency to build a native Rust omnichannel customer support and chat engine. Our target personas (Maya, Carlos, Priya, Leo, Fatima) need unified communication across web, email, SMS, and Instagram DMs directly within OHC's high-performance native architecture, without relying on third-party SaaS tools that complicate deployment, scale poorly, or lack deep platform integration.

  ## Research Report
  - The `legacy-chat-engine/legacy-chat-engine` repository (Ruby on Rails) was analyzed to understand the domain model of an omnichannel inbox.
  - **Key Entities Identified**: `Account` (Tenant), `Inbox`, `Channel` (WebWidget, API, Email, Twilio, Facebook), `Conversation`, `Message`, `Contact`, `User` (Agent).
  - **Key Features**: Real-time WebSocket messaging, omnichannel routing (WhatsApp, Instagram, Web), agent assignment, canned responses, SLA policies, and webhook integrations.
  - **OHC Implementation Goal**: Replicate this domain model natively in Rust, ensuring strict multi-tenant isolation (Zero-Trust via `tenant_id` RLS) and leveraging high-performance asynchronous Rust (Tokio/Axum) for WebSocket handling.

  ## Design Doc
  - **Architecture Diagram**:
    ```mermaid
    erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ CONVERSATION : has
      INBOX ||--o{ CONVERSATION : tracks
      CONVERSATION ||--o{ MESSAGE : contains
      CONTACT ||--o{ CONVERSATION : initiates
      INBOX ||--o| CHANNEL_ADAPTER : uses
      CHANNEL_ADAPTER {
          string type "WEB_WIDGET, EMAIL, SMS, INSTAGRAM"
          json credentials
      }
    ```
  - **Mobile UX Flow**: The chat interface must be optimized for 375px screens. A unified "Inbox" tab displays all active conversations. Tapping a conversation opens a full-screen chat view with quick actions for canned responses and AI-assisted replies.
  - **AI Agent Integration**: The Customer Service AI agent will listen to new `Message` events on the internal queue and automatically draft replies for unassigned conversations, proposing them to the owner for approval.
  - **Key Design Decisions**: Use Axum for HTTP and WebSocket APIs. Leverage PostgreSQL with RLS for multi-tenancy. Use Redis (Valkey) pub/sub for cross-node WebSocket event broadcasting.

  ## Implementation Prompt
  Implement the core database schema (PostgreSQL with RLS) and basic Rust CRUD APIs for the native OHC chat system. Start with the `Inbox`, `Conversation`, and `Message` entities. The user-facing outcome is that a team member can create an inbox and send/receive messages via a basic API. Include E2E tests verifying tenant isolation. Do not implement specific channel adapters (like Twilio) yet; focus on the core chat engine.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
