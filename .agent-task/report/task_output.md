issue_title: "Native Rust Omnichannel Chat Integration - Core Engine & Widget API"
issue_description: |
  # Native Rust Omnichannel Chat Integration - Core Engine & Widget API

  ## Problem Statement
  We have fully retired Chatwoot as an external service to reduce operational overhead and embrace a unified backend. We need to replace Chatwoot's core messaging engine and widget APIs with a native Rust implementation in `onehumancorp/mono`. Currently, the `omnichannel_repo.rs` only has a basic implementation for conversational concepts, but it doesn't support the full breadth of multi-channel data models (Inboxes, Contacts, Conversations, Messages, Webhooks) required to serve our owner personas (Maya, Carlos, Priya, Leo).

  ## Research Report
  - **Competitor/Legacy Audit**: Chatwoot’s core data model revolves around `Account` (Tenant), `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message`. Their API exposes endpoints for Web Widgets (`/api/v1/widget/messages`, `/api/v1/widget/conversations`) and internal operator endpoints.
  - **OHC Gap**: OHC's current `omnichannel_repo.rs` has basic `CustomerProfile`, `WorkItem`, `Conversation`, and `Message` entities, but lacks `Inbox`, robust `Contact` identities, and widget-specific API endpoints to process real-time incoming messages.
  - **Proposed Capability**: A native Rust Chat Engine with strict multi-tenant isolation, serving as the foundational API for the web widget and future integrations (WhatsApp, IG).

  ## Design Doc
  ### Architecture
  - **Data Model (PostgreSQL / SeaORM or SQLx)**:
    - `Tenant` -> 1:N -> `Inbox`
    - `Inbox` -> 1:N -> `Conversation`
    - `Contact` -> 1:N -> `Conversation`
    - `Conversation` -> 1:N -> `Message`
    - Multi-tenant isolation using `tenant_id` on all tables.
  - **API Layer**:
    - Build Axum routes under `src/server/api/chat/` for the web widget: `POST /api/chat/messages`, `GET /api/chat/conversations`.
  - **Real-time**:
    - Placeholder for WebSocket/SSE using `axum::extract::ws` or NATS pub/sub integration.
  - **Mobile UX Flow**: The core backend changes will power the unified inbox on 375px mobile screens, allowing owners to view threads across channels.
  - **AI Agent Integration**: `OmniChannelService` will trigger AI drafts via `AgentDraft` on new incoming messages in `Conversation`.

  ## Implementation Prompt
  Implementer Agent:
  1. Expand the data models in `src/server/domain/repository/omnichannel_repo.rs` to include `Inbox` and `Contact` with full `tenant_id` isolation.
  2. Create Axum API endpoints for the Chat Widget: `POST /api/widget/conversations`, `POST /api/widget/messages`, and `GET /api/widget/messages`.
  3. Wire up the API to use the expanded repository. Ensure all queries filter by `tenant_id`.
  4. Write comprehensive unit tests for the repository and API layer, adhering to the 100% test coverage rule.
  5. The target CUJ: A user on an external website (via Widget) starts a conversation and sends a message, which is persisted and visible to the OHC owner.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
