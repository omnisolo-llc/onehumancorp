issue_title: "Retire the legacy third-party chat dependency: Native Rust Omnichannel Chat System Architecture"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is transitioning away from the legacy third-party chat dependency as a third-party omnichannel customer support dependency to build a fully native, high-performance omnichannel chat system in Rust within our monorepo. This allows deep integration with our existing tenant model, AI agents, billing, and order management, while avoiding the operational overhead and feature limitations of external dependencies. Non-technical owners need an integrated inbox where all communications (Instagram DMs, Web Chat, Email, WhatsApp) are triaged by the AI agent before escalating to the human owner.

  ## Research Report
  Based on an audit of the current the legacy third-party chat dependency architecture (via their GitHub repository), the system relies heavily on several key entities:
  - **Conversations**: The central aggregation unit tracking thread status, assignees (Agent/Bot), priority, SLA, and timestamps.
  - **Messages**: Individual communication units within a conversation, containing text, attachments, and metadata, typed as `incoming`, `outgoing`, or `template`.
  - **Contacts & Inboxes**: Modeling the customer identity and the channel entry point (e.g., a specific WhatsApp number or web widget).
  - **Channel Adapters**: Provider-specific integrations that normalize external provider webhooks into unified `Message` records.

  Leading platforms like Shopify Ping and Stripe Dashboard demonstrate the value of keeping these communications natively integrated with the primary transactional database, reducing latency and allowing unified search/context for the LLM agents.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
    TENANT ||--o{ INBOX : owns
    TENANT ||--o{ CONTACT : has
    INBOX ||--o{ CHANNEL_ADAPTER : configured_via
    INBOX ||--o{ CONVERSATION : receives
    CONTACT ||--o{ CONVERSATION : initiates
    CONVERSATION ||--o{ MESSAGE : contains
    MESSAGE }|--|| MESSAGE_TYPE : is_of_type
    CONVERSATION }|--|| CONVERSATION_STATUS : has_state
  ```

  ### Core Entities & Data Model
  We will implement the following core entities in PostgreSQL (via SQLx in our Rust backend), strictly adhering to our multi-tenant row-level security model (`tenant_id` on all tables):

  - **`inboxes`**: Represents a channel endpoint (e.g., "Main Web Widget", "Support Email").
  - **`contacts`**: Customer records (merged across channels where identity is known).
  - **`conversations`**:
    - `id`, `tenant_id`, `inbox_id`, `contact_id`
    - `status` (open, snoozed, resolved, bot_assigned)
    - `assignee_id` (human or AI agent)
    - `custom_attributes` (JSONB)
  - **`messages`**:
    - `id`, `tenant_id`, `conversation_id`
    - `content`, `content_type` (text, image, template)
    - `message_type` (incoming, outgoing, internal_note)
    - `sender_type` (contact, user, agent_bot)

  ### Mobile UX Flow (375px First)
  1. **Work Triage Feed**: The owner opens the app. The primary tab shows "Requires Attention".
  2. **Unified Inbox**: Instead of separate channel tabs, the owner sees a single feed of conversations. Each card shows the contact name, the latest message snippet, a channel icon (Web/IG/WA), and an AI-generated suggested reply if the agent hasn't fully handled it.
  3. **Conversation View**: Tapping a conversation opens a standard chat view. The AI's drafted response sits in the text input box, ready for the owner to tap "Approve & Send" or edit.
  4. **Context Drawer**: Swiping left or tapping a header info icon reveals the contact's previous orders/appointments (pulled from OHC native state), replacing the need for external integrations.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Triggers on every new `Message` creation. Analyzes intent. If it's a simple query (e.g., "Are you open today?"), the agent drafts a reply and optionally auto-sends based on owner preferences.
  - **Operations Agent**: If the message implies a booking or order change, the agent extracts the intent and attaches an actionable "Suggested Action" card to the conversation (e.g., "Move appointment to 3 PM").

  ## Implementation Prompt
  **To the Implementer:**
  Implement the backend data model and basic CRUD API for the native OHC Omnichannel Chat System.
  1. Create database migrations for the tables: `inboxes`, `contacts`, `conversations`, and `messages`. Ensure every table includes a `tenant_id` and has Row-Level Security enabled.
  2. Implement the Rust service layer (e.g., `src/server/services/chat/`) with methods to create an inbox, create a conversation, and add messages to a conversation.
  3. Create the corresponding gRPC/REST API endpoints to expose this functionality to the Flutter/Tauri frontend.
  4. Implement a Critical User Journey (CUJ) Playwright E2E test that simulates a customer sending a message through a mock channel, and an owner logging into the OHC dashboard to view the message in the unified inbox.

  Do not worry about the specific channel adapter implementations (WhatsApp, IG) in this phase; focus on the core data structures and the unified internal API.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
