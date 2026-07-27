issue_title: "Architecture Design: Native Rust Omnichannel Inbox & Agent Routing (Chatwoot Parity)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) has mandated the 100% retirement of Chatwoot as a third-party dependency. OHC must natively implement its own high-performance, multi-tenant omnichannel customer support & chat engine in Rust. Currently, the OHC `ChatInbox` and related data models (in `src/server/services/chat/models.rs` and migration `217_native_omnichannel_chat.sql`) are rudimentary, lacking the advanced features found in Chatwoot such as working hours, out-of-office messages, auto-assignment, round-robin routing, CSAT surveys, and detailed channel configurations. Non-technical owners (like Maya the baker and Carlos the handyman) need an intelligent unified inbox that automatically routes messages, handles off-hours responses via AI, and tracks customer satisfaction, without requiring manual intervention.

  ## Research Report
  **Chatwoot Source Code Audit Findings:**
  An audit of the cloned Chatwoot repository (`https://github.com/chatwoot/chatwoot`) reveals a sophisticated inbox and routing architecture:
  - **Inbox Model (`app/models/inbox.rb`)**: Supports `working_hours_enabled`, `out_of_office_message`, `greeting_enabled`, `csat_survey_enabled`, `enable_auto_assignment`, and `auto_assignment_config`. It ties to polymorphic channels (`Channel::Whatsapp`, `Channel::Email`, `Channel::WebWidget`, etc.).
  - **Conversation Model (`app/models/conversation.rb`)**: Handles SLA policies, assignment (`assignee_id`), bot handoffs (`assignee_agent_bot_id`), status (`open`, `resolved`, `pending`, `snoozed`), and detailed timestamps for first reply and last activity to calculate metrics.
  - **Agent Routing**: Implements Round-Robin assignment policies (`AutoAssignment::InboxRoundRobinService`) and load balancing based on agent capacity.
  - **Competitor Landscape**: Platforms like Shopify Inbox, Zendesk, and Intercom provide similar omnichannel capabilities but often require complex configurations. OHC's differentiation is the "AI Assistant-first" approach where agents configure routing and draft replies automatically for the owner.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ CHAT_INBOX : manages
      CHAT_INBOX ||--o{ CHAT_CHANNEL : contains
      CHAT_INBOX ||--o{ CHAT_CONVERSATION : hosts
      CHAT_CONVERSATION ||--o{ CHAT_MESSAGE : has

      CHAT_INBOX {
          UUID id PK
          UUID tenant_id FK
          String name
          Boolean working_hours_enabled
          String out_of_office_message
          Boolean greeting_enabled
          String greeting_message
          Boolean csat_survey_enabled
          JSONB auto_assignment_config
      }

      CHAT_CHANNEL {
          UUID id PK
          UUID tenant_id FK
          UUID inbox_id FK
          String channel_type "e.g., whatsapp, web_widget, instagram"
          JSONB credentials
      }

      CHAT_CONVERSATION {
          UUID id PK
          UUID tenant_id FK
          UUID inbox_id FK
          UUID contact_id FK
          UUID assignee_id "Agent or None"
          UUID bot_assignee_id "AI Agent"
          String status "open, resolved, pending, snoozed"
          DateTime waiting_since
      }
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  **Screen 1: Unified Inbox View (375px)**
  - Top Nav: "Inbox" title, Dropdown to filter by "Open", "Resolved", "Snoozed".
  - Main Body: Vertical list of conversation cards.
    - Card: Avatar (channel icon + contact initials), Contact Name, Last Message Snippet (truncated to 1 line), Timestamp. Translucent Glass styling, 12px spacing.
  - Bottom Nav: Standard OHC mobile tab bar.

  **Screen 2: Conversation Thread & AI Drafts (375px)**
  - Top Nav: Back Button, Contact Name, Channel Badge (e.g., "WhatsApp").
  - Thread: Chat bubbles. Incoming (gray glass), Outgoing (OHC brand color).
  - Floating AI Action: "AI Assistant drafted a reply: 'Yes, we do vegan cakes!'" with "Approve & Send" / "Edit" buttons.
  - Composer: Input field, attachment icon, native mobile keyboard integration.

  ### AI Agent Integration Points
  - **Operations Assistant**: Triggers upon new conversation creation. If `working_hours_enabled` is true and it's off-hours, the Operations Assistant automatically sends the `out_of_office_message`.
  - **Customer Assistant (Bot Handoff)**: Evaluates incoming messages for intent. If the AI is confident (e.g., FAQ), it drafts a reply and assigns the conversation to `bot_assignee_id`. If human approval is needed, it drafts the reply and leaves the conversation assigned to the human owner.
  - **Finance/Sales Assistant**: Can interject in the conversation to attach a Stripe payment link if the customer asks for a quote.

  ### Key Design Decisions
  - **Tenant Isolation**: Row-Level Security (RLS) via `tenant_id` on all new tables/columns.
  - **Zero Trust**: Channel credentials (e.g., WhatsApp API tokens) must be stored securely (encrypted at rest) in the `CHAT_CHANNEL` table's JSONB.
  - **Bot vs. Human Assignment**: Explicit separation of `assignee_id` (human) and `bot_assignee_id` (AI), mirroring Chatwoot's architecture to prevent routing conflicts and enable clean handoffs.

  ## Implementation Prompt
  **Goal**: Upgrade the OHC native Rust chat system to support Chatwoot-parity Inbox features (Working Hours, Greetings, Agent Routing, and AI Bot Handoff).
  **Persona**: Maya (Baker) wants her Instagram and WhatsApp messages funneled into one OHC inbox. When she's sleeping, she wants her custom out-of-office message sent automatically, and she wants the AI Assistant to draft replies for her to review in the morning.

  **Tasks for Implementer**:
  1. Add missing schema columns to `chat_inboxes` and `chat_conversations` (via a new SQL migration in `src/server/migrations`) to support `working_hours_enabled`, `out_of_office_message`, `greeting_enabled`, `greeting_message`, `status`, `assignee_id`, and `bot_assignee_id`.
  2. Update the Rust models in `src/server/services/chat/models.rs` to match the new schema.
  3. Implement the CRUD API endpoints for updating Inbox configurations.
  4. Build the mobile-first (375px) Unified Inbox UI in Flutter/Next.js using OHC Translucent Glass tokens.
  5. Add robust E2E Playwright tests covering the "Create Inbox -> Configure Working Hours -> Receive Message -> Verify Auto-Reply" Critical User Journey (CUJ).
  6. Ensure 100% unit test coverage for the new Rust service logic and strict RLS tenant isolation.

  **Acceptance Criteria**:
  - The database migration applies cleanly with RLS enforced.
  - The UI is fully usable on a 375px width without horizontal scrolling.
  - Playwright E2E tests pass for the off-hours auto-reply scenario.
  - ZERO mock data is used in the UI; all configurations flow end-to-end.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
