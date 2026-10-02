outcome: no_work
issue_title: "Product Research: OHC Omnichannel Messaging & Agentic Assistance Gap Analysis"
issue_description: |
  # Gap Analysis Report

  The requested feature, a Native Rust Omnichannel Messaging System to replace external dependencies, has already been implemented natively in the codebase.

  ### Verified Implementation Evidence
  - **Database Schema**:
      - The `1009_native_omnichannel_chat.sql` and `233_chat_omnichannel.sql` migrations correctly define the expected core multi-tenant models with Row-Level Security explicitly enforced.
      - Migrations establish `chat_inboxes`, `chat_channels`, and `chat_contacts` tables scoped to `tenant_id`.
  - **Rust Backend Core Services**:
      - `src/server/services/chat/models.rs` implements the native data models mapped to the PostgreSQL tables.
      - `src/server/services/chat/service.rs` provides the `ChatService` implementation to handle operations like `create_inbox`, applying SQL `SET LOCAL app.current_tenant_id` context for tenant isolation.

  ### Unverified Criteria
  The issue descriptions mention AI triage auto-parsing to generate draft replies and connecting conversations to business action entities (like quotes or bookings), which are missing in the currently provided code context:
  - Missing complete implementations tying `src/server/services/chat` native components directly with the AI Agent generation pipelines specifically for incoming `ChatMessage` intent parsing.
  - Missing real-time WebSocket infrastructures directly integrated into the chat UI for dynamic event feeds natively (outside of the specific help widget implementations).
  - Missing a mobile-first `375px` conversation view in Flutter implementing translucent AI draft panels and actionable intent chips for a unified inbox view.

  ### Superpowers Workflow Provenance
  - Upstream Commit Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Loaded Skills Paths: skills/using-superpowers/SKILL.md
