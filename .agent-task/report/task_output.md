issue_title: "Architectural Design: Native Rust Omnichannel Chat System"
issue_description: |
  # Mission Queue Protocol: Native Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) owners need a single, unified inbox to coordinate messages from various channels (Instagram DMs, WhatsApp, SMS, web chat) seamlessly. Currently, OHC relied on a legacy external integration (Chatwoot), which has been retired. For personas like Maya (baker managing Instagram DMs) and Carlos (handyman fielding SMS requests), the assistant must have direct, native access to these conversations to instantly triage requests, coordinate bookings, and propose draft replies without routing through third-party services. The lack of a native system breaks the "single pane of glass" and "AI Work Assistant" vision.

  ## Research Report
  - **Codebase Audit:** The repository previously integrated Chatwoot, which has now been fully removed from the active application and deployment graph (`deploy/tests/no_chatwoot_residue_test.sh` enforces this).
  - **Source Material:** The `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md` specifies a consolidated inbox domain.
  - **Competitor Analysis:** Shopify Inbox and WeCom handle omnichannel natively by pushing real-time events via WebSockets to mobile apps and using a single canonical data store for robust analytics. Relying on an external chat platform introduces latency, security risks, and high operational overhead.
  - **Finding:** A native Rust implementation using an explicit Delivery Outbox pattern, canonical `Inbox`, `Conversation`, and `Message` entities, backed by PostgreSQL RLS for multi-tenancy, and WebSockets/PowerSync for local-first mobile sync is critical.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Inbox ||--o{ ChannelConnection : has
      Inbox ||--o{ Conversation : routes
      Tenant ||--o{ Contact : manages
      Contact ||--o{ ContactIdentity : identified-by
      Conversation ||--o{ Message : contains
      Conversation ||--o{ Participant : includes
      Message ||--o| Attachment : includes
      Message ||--o{ Receipt : tracked-by

      Inbox {
          uuid id
          uuid tenant_id
          string name
      }
      ChannelConnection {
          uuid id
          uuid inbox_id
          string provider
          jsonb capabilities
      }
      Conversation {
          uuid id
          uuid inbox_id
          string status
          int priority
      }
      Message {
          uuid id
          uuid conversation_id
          uuid sender_id
          text content
          string delivery_state
      }
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Unified Inbox List (375px):** A clean, un-cluttered list view. Each row shows the customer avatar, channel icon (e.g., Instagram, SMS), a bold snippet of the latest message, and a subtle unread indicator.
  - **Conversation Thread (375px):** Translucent glass topbar with the customer's name and channel icon. The main canvas contains chat bubbles. An input area at the bottom stays fixed above the native keyboard, featuring a "+ Attachment" button and an "AI Draft" spark icon.
  - **Mobile UX Flow:** Maya receives an Instagram DM. A push notification arrives. Tapping it opens the OHC app directly to the conversation. An AI-generated draft response is already suggested in the input field. Maya taps "Send". The message state transitions from `Queued` -> `Sent` -> `Delivered` with checkmarks (similar to WhatsApp).

  ### AI Agent Integration Points
  - **Work Triage Agent:** Hooks into the incoming message pipeline. Evaluates intent. If it's a booking, extracts dates; if a question, queries Knowledge base.
  - **Customer & Relationship Assistant:** Drafts suggested replies based on context and past interactions. The draft is stored ephemerally and pushed to the UI for owner approval.
  - **Security & Authorization:** The AI agents operate strictly within the bounds of the `tenant_id` context. They cannot access cross-tenant data.

  ### Key Design Decisions
  - **Native Rust Implementation:** Guarantees memory safety, high concurrency for WebSockets, and deep integration with the core OHC GraphQL/REST API.
  - **Multi-Tenant Isolation (PostgreSQL RLS):** Every query is scoped by `tenant_id` natively at the database level.
  - **PowerSync for Offline-First:** Ensures the app remains responsive on flaky mobile connections (crucial for Carlos on the road or Fatima in a busy food cart).
  - **Delivery Outbox Pattern:** Ensures messages are never lost during transient network failures or provider outages.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the Core Domain Model and PostgreSQL RLS Schema for the Native Omnichannel Chat System in the Rust backend.

  **Acceptance Criteria:**
  1. Define the Rust structs and traits for `Inbox`, `Conversation`, `Message`, `Contact`, and `ChannelConnection`.
  2. Implement the PostgreSQL migrations with `ENABLE ROW LEVEL SECURITY` and strict `tenant_id` checks for all new tables.
  3. Ensure that all database queries automatically apply the `TenantContext`.
  4. Write comprehensive unit tests verifying that cross-tenant access is blocked at the database level (Cross-Tenant Denial Tests).
  5. The API surface must not expose the `tenant_id` parameter to the client; it must be derived securely from the authenticated session.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
