issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  **Title**: Implement Custom Rust Omnichannel Chat System (Chatwoot Replacement)

  **Problem Statement**:
  Currently, OHC lacks a native omnichannel chat system and relied on Chatwoot as an external service. Chatwoot has been fully retired as an external service requirement. Non-technical owners (like Maya the baker and Carlos the handyman) need a unified inbox where they can handle Instagram DMs, WhatsApp, SMS, and web widget chats seamlessly within the OHC platform without needing to configure or manage external tools. We need a native Rust implementation integrated directly into OHC.

  **Research Report**:
  - Chatwoot was providing web widget, conversation routing, macro responses, assignment policies, and webhook integrations for Meta/WhatsApp.
  - OHC's backend currently has stubbed models in `src/server/services/chat/` and migration files like `20260701_omnichannel_tables.sql` mapping basic customer profiles and work items, but lacks the deep real-time messaging, websocket presence, unified conversations, channels, and UI to surface this.
  - Reviewing Chatwoot's source code at `/tmp/chatwoot/app/models/`, core models include `Account`, `Inbox`, `Conversation`, `Message`, `Contact`, `Channel::*`, `AgentBot`, and `Webhook`.
  - To implement in OHC, we need to map these to OHC's multi-tenant architecture (`tenant_id` isolated RLS tables), building upon the `work_item` concept to include threaded `conversations` and `messages`. We will need real-time update capability (already possible via gRPC/Tauri/Next frontend integrations or websockets).

  **Design Doc**:
  - **Architecture Diagram (Mental Model)**:
    - `Web Widget / Instagram DM` -> `OHC Webhook/API Route` -> `Inbox Service (Rust)` -> `DB (Conversations/Messages)` -> `Tauri UI (Owner Workstation)`
    - Background AI (Operations/CS) can intercept messages and draft `agent_draft` records.
  - **Data Models (Multi-tenant via RLS)**:
    - `omni_inboxes`: config for channels (web widget, IG, etc.)
    - `omni_conversations`: links `customer_profile`, `omni_inboxes`, and `status`.
    - `omni_messages`: individual messages in a thread, `direction` (in/out).
  - **Mobile UX Flow**:
    - Inbox view at 375px: list of unread conversations.
    - Tap conversation: Thread view with input box.
    - Glassmorphism design and UniFi clean layouts.
  - **AI Agent Integration Points**:
    - AI automatically analyzes incoming `omni_messages` in background job.
    - Creates `agent_draft` for the owner to 1-tap send, or auto-responds if confident (like Maya sleeping).

  **Implementation Prompt**:
  "As an Implementer agent, build the native Rust unified omnichannel chat backend and the corresponding Flutter/Tauri UI. Use the existing multi-tenant architecture. Add DB migrations for `omni_inboxes`, `omni_conversations`, and `omni_messages` with strict RLS policies. Implement the gRPC/REST endpoints for receiving webhooks from platforms and fetching threads for the UI. Create the 'Inbox' screen in the Tauri/Flutter app, ensuring 375px mobile responsiveness, translucent glass styling, and seamless interaction for reading and replying. Provide 100% unit and Playwright E2E coverage for a user replying to a simulated DM."

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
