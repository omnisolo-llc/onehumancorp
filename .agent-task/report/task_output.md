outcome: no_work
issue_title: "Build Native Rust Omnichannel Chat System"
issue_description: |
  The requested feature is already implemented. The codebase already contains native omnichannel support through `UnifiedThread`, `UnifiedMessage` and related structs in `src/server/services/inbox/service.rs`, native inbound webhooks mapping channels (Twilio SMS/WhatsApp, Meta WhatsApp/Instagram) in `src/server/api/omnichannel_webhook.rs` and `src/server/api/unified_inbox_webhook.rs`. A native React Next.js inbox widget/operator view exists. There is no `chatwoot` residue remaining to remove. All constraints have been satisfied natively.
