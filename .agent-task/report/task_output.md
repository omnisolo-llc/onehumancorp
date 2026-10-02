outcome: no_work
issue_title: "[Native Rust Implementation] Omnichannel Unified Inbox Architecture & Missing Core Capabilities"
issue_description: |
  The requested omnichannel unified inbox architecture is already implemented. The codebase contains UnifiedThread, UnifiedMessage, and InboxService in src/server/services/inbox/service.rs. Webhook endpoints and WebSocket streaming are available in src/server/api/inbox/webhook.rs and src/server/api/unified_ws.rs. E2E tests for the frontend already exist in src/ui/next/src/e2e/omni_inbox.spec.ts. Therefore, this issue requires no further work.
