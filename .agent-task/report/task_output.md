issue_title: "Native Rust Omnichannel Chat System Replication (Chatwoot Replacement)"
issue_description: |
  **Problem Statement**:
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context and require manual responses. OHC needs to build a native omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono` to replace the retired external Chatwoot dependency.

  **Research Report**:
  - Chatwoot as an external third-party service, dependency, or integration is 100% RETIRED.
  - Native Rust Implementation: OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`.
  - Swarm agents MUST checkout / clone the Chatwoot source repository (https://github.com/chatwoot/chatwoot) to audit its source code (data models, channel adapters, web chat widget, WebSocket events, APIs, webhooks, SLA policies, macros, canned responses, agent routing) and build matching native Rust microservices, crates, and frontend UI components in OHC to achieve 100% feature parity.
  - OHC differentiation: The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response.

  **Design Doc**:
  ### Architecture
  ```mermaid
  graph TD
      A[Instagram DM / WhatsApp / Email] -->|Webhook| B(Omnichannel Gateway - Rust)
      B --> C{Customer Identity Resolution Engine}
      C -->|Lookup| D[Unified Customer Graph DB]
      C --> E[Event Mesh]
      E --> F[The Ambassador Agent]
      F -->|Query Context| D
      F -->|Draft Reply| G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Omnichannel Dispatcher - Rust]
      I --> A
  ```

  ### Data Model & Invariants
  - `Conversation`: Links to a specific `Customer` and `Inbox`.
  - `Inbox`: Represents a specific channel (e.g., an Instagram account).
  - `Message`: Individual messages within a `Conversation`.
  - `ChannelAdapter`: Rust traits for handling different integrations (e.g., WhatsApp, Instagram).
  - Strict multi-tenant isolation using `tenant_id` on all tables with Row Level Security (RLS) in PostgreSQL.

  ### Mobile UX Flow (375px First)
  - **Unified Inbox View**: A list of conversations across all channels, clearly badged with the source channel icon.
  - **Conversation View**: Clean, translucent glass UI showing the message history.
  - **AI Drafting**: The Ambassador agent proactively drafts replies. The owner sees the drafted reply at the bottom of the screen with a single "Approve & Send" button, or an "Edit" button.

  ### AI Agent Integration
  - **The Ambassador**: Listens to the `MessageReceived` event on the Event Mesh. Uses the customer's history and the business's data to draft a contextual reply.

  **Implementation Prompt**:
  As an implementer, build the core Rust backend architecture for the native Omnichannel Inbox.
  1. Define the PostgreSQL schema for `inboxes`, `conversations`, and `messages`, ensuring `tenant_id` RLS is applied.
  2. Implement the core Rust structs and repositories for these entities.
  3. Create the foundational `ChannelAdapter` trait that future specific channel integrations (like Instagram or WhatsApp) will implement.
  4. Build the core API endpoints to fetch inboxes and conversations for a tenant.
  5. Ensure all new code has 100% test coverage and verify the API through E2E Playwright tests using a mocked frontend component or direct API calls within the test environment.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
