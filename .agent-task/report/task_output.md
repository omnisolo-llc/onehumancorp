issue_title: "Architect & Implement Native Rust Omnichannel Chat Data Models"
issue_description: |
  ## Problem Statement
  Small business owners using OneHumanCorp (OHC) need a centralized place to view and respond to messages from multiple sources (WhatsApp, Website Live Chat, Instagram, etc.). Previously, OHC relied on an external third-party service (Chatwoot). To reduce latency, ensure strict tenant isolation (RLS), and tightly integrate the "Customer Success Agent" (The Ambassador), OHC is retiring the external Chatwoot dependency and building a native Rust-based omnichannel chat system.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Audit:** I have audited the open-source Chatwoot repository to understand its core architecture. Chatwoot relies on the following key entities:
    - `Inbox`: Represents a collection point for messages (e.g., a specific WhatsApp number or Website widget).
    - `Channel`: The specific type of integration for an inbox.
    - `Conversation`: A thread of messages between a contact and a team (or agent) within an Inbox.
    - `Message`: The individual message payloads, including text, attachments, and metadata.
    - `Contact` and `ContactInbox`: Represents the customer and their link to a specific inbox identifier (e.g., their WhatsApp number).
  - **OHC Gap:** OHC currently lacks these robust, multi-tenant conversational data models natively in PostgreSQL and Rust. The legacy integration must be replaced with native models that enforce `tenant_id` Row-Level Security (RLS) on every table, seamlessly integrating with OHC's AI Work Triage.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : contains
      INBOX ||--o| CHANNEL_ADAPTER : configured_via
      CONTACT ||--o{ CONTACT_INBOX : has
      INBOX ||--o{ CONTACT_INBOX : has
      CONTACT_INBOX ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE }|--|| TENANT : belongs_to
      CONVERSATION }|--|| TENANT : belongs_to
      INBOX }|--|| TENANT : belongs_to
      CONTACT }|--|| TENANT : belongs_to
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Inbox List View:** A clean, 375px-friendly list of open conversations. Unread messages are bolded. Small channel icons (e.g., WhatsApp, Web) indicate the source.
  - **Conversation View:** Standard chat interface. Left-aligned bubbles for the contact, right-aligned for the agent/owner. A sticky input area at the bottom with native keyboard support. Minimum 44x44px touch targets for "Send" and "Attach" buttons.
  - **AI Agent Integration:** When opening a conversation, a "Draft Reply" button is prominently displayed. Tapping it invokes "The Ambassador" agent to generate a contextual response based on the contact's history.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Hooks into the `Conversation` creation and `Message` creation events. If a message is from a customer, The Ambassador can automatically evaluate it and draft a response or resolve it if it's a simple FAQ.

  ### Key Design Decisions
  - **Strict Multi-Tenancy:** Every table (`inboxes`, `conversations`, `messages`, `contacts`) MUST have a `tenant_id` column and PostgreSQL Row-Level Security (RLS) enabled.
  - **Native Rust Implementation:** We will implement the backend using Axum/Tonic in Rust (`src/server/services/chat/`).
  - **Channel Adapters:** Instead of rigid polymorphic associations like Rails, we will use a JSONB `channel_config` on the `inboxes` table or a separate `channel_adapters` table to store channel-specific credentials securely.

  ## Implementation Prompt
  **User-Facing Outcome:** The user can navigate to the "Chat" section in the OHC mobile or desktop app and see a native, lightning-fast inbox containing messages from all configured channels, without relying on an external iframe or service.

  **CUJ & Acceptance Criteria:**
  1. Create PostgreSQL database migrations to create the core tables: `inboxes`, `contacts`, `contact_inboxes`, `conversations`, and `messages`, all with `tenant_id` and RLS.
  2. Implement Rust struct models (in `src/server/services/chat/models.rs`) mapped to these new tables.
  3. Expose Axum HTTP or gRPC endpoints to list inboxes, list conversations for an inbox, and fetch messages for a conversation.
  4. Ensure 100% unit test coverage for the new Rust service layer.
  5. Create a Playwright E2E test where an Admin logs in, creates a new native Inbox, creates a mock Contact, and sends a test Message, verifying it appears in the UI (mocking the UI if necessary, but testing the real backend API).

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
