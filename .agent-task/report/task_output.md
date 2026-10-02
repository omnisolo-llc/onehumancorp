outcome: no_work
issue_title: "[Native Chat] Custom Rust Omnichannel Chat System Replication"
issue_description: |
  # Problem Statement

  OmniSolo currently relies on an external integration for omnichannel customer support and inbox functionality. This violates our core tenet of a unified, self-contained, native platform, and limits our ability to seamlessly inject our AI agents (like "The Ambassador") directly into the core event stream.

  We need to replace the external dependency entirely by replicating its core omnichannel data models, real-time WebSocket messaging, and inbox architecture natively in Rust inside `omnisolo/mono`. This will enable true, invisible AI agent coordination for SMB owners, meeting our core value of "Radical Simplicity" where the system just works without complex third-party configurations.

  # Finding

  The core omnichannel data models have already been replicated natively in Rust inside the OHC stack. The `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables are defined in PostgreSQL with Row-Level Security in `src/server/db/migrations/1009_native_omnichannel_chat.sql`. These models are manipulated by a native Rust service, `src/server/services/chat/service.rs`, which implements multi-tenant DB isolation via `SET LOCAL app.current_tenant_id = '{}'`.

  Furthermore, incoming webhooks are natively ingested and dispatched without an external Chatwoot deployment. `src/server/api/omnichannel_webhook.rs` handles standard webhook ingestion for Facebook, Instagram, email, SMS, etc. It natively standardizes the payload into a standard `OmnichannelPayload` object. A new job, `message_triage`, is then spawned inside `ohc_job_queue`.

  Chatwoot removal has also been largely addressed, with legacy code paths being actively monitored for drift via `deploy/tests/no_chatwoot_residue_test.sh`. The `chatwoot` image and container configuration in Docker have been purged.

  Some acceptance criteria have not been explicitly observed in the repository:
  - Real-time websocket delivery mechanism to the mobile client
  - End to end coverage specifically validating `message_triage` job and a mock AI agent drafted reply response to the owner.

  Since the core implementation and migration paths have already been applied natively, we are returning a `no_work` finding and preserving the current system behavior.
