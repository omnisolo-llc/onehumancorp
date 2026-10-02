outcome: blocked
issue_title: "Native Rust Omnichannel Chat: Unified Inbox Architecture & Real-Time Messaging"
issue_description: |
  The task is blocked. The codebase review reveals that the `Native Rust Omnichannel Chat` feature is already largely implemented and present in the codebase.
  The required domain models (`Inbox`, `Conversation`, `Message`, `Contact`) exist in `src/server/domain/chat/mod.rs` and `src/server/services/chat/models.rs`. The webhook structure is visible in `src/server/api/unified_inbox_webhook.rs`. The `WhatsApp Cloud` integration is built in `src/server/integrations/whatsapp_cloud/`.

  Furthermore, the issue instructions dictate that implementation of new external channel connectors (like WhatsApp, Twilio, Resend, Meta Cloud API) is strictly gated by end-to-end sandbox verification. The required external sandbox credentials (e.g., for Meta/WhatsApp) are not available in this environment. Mocks and placeholders without end-to-end sandbox verification are forbidden. A missing external SDK, provider sandbox, or signing credential is a specific outstanding verification dependency, which overrides any implementation instruction and mandates a `blocked` or `no_work` report.
