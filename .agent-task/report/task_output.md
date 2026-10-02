outcome: no_work
issue_title: "🛠️ Taskmaster: [Native Rust Omnichannel Inbox]"
issue_description: |
  The Native Rust Omnichannel Inbox is already implemented.

  Verified criteria:
  1. SQL schema with RLS on tenant_id is in src/server/db/migrations/1009_native_omnichannel_chat.sql
  2. CRUD backend is in src/server/services/chat/service.rs
  3. API endpoint for unified feed is in src/server/api/unified_inbox_webhook.rs

  Unverified criteria:
  - End-to-end integration with the explicitly deprecated Flutter frontend.
