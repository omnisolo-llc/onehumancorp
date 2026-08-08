issue_title: "Implement Custom Rust Omnichannel Chat System based on Chatwoot Architecture"
issue_description: |
  **Problem Statement**
  OneHumanCorp (OHC) is replacing the third-party Chatwoot dependency with a native, high-performance, multi-tenant Rust backend implementation within the repository.

  **Research Report**
  We examined Chatwoot's Ruby-on-Rails architecture and data models (`app/models/*`). The core models revolve around Conversations, Contacts, Messages, Agents (Users), and Inboxes (with Channel integrations). This matches our requirements exactly, but needs to be adapted for Rust/PostgreSQL using strict tenant row-level security (RLS).

  **Design Doc**
  - **Architecture:** Native Rust microservices inside `src/server/services/chat` and `src/server/domain/`.
  - **Data Models:** Inboxes, Channels, Conversations, Messages, Contacts. Every table must include `tenant_id` and have Postgres RLS policies enabled.
  - **Mobile UX:** 375px-first. A combined chat inbox feed. Unread badges. Quick reply overlays.
  - **AI Agents:** Customer assistant bots automatically draft replies and use tenant context for responses.

  **Implementation Prompt**
  Implement the core data model and Rust API for the Omnichannel Chat system. Focus on creating the DB migrations (SQL) for `inboxes`, `conversations`, and `messages` (all using `tenant_id` and RLS). Then, implement the core Rust API service layer to create/list conversations and send messages.
  Ensure it follows the `OHC_MULTITENANT` isolation patterns. Add robust unit tests (100% coverage).

  **Priority:** P0
  **Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
