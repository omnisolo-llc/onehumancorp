issue_title: 36756
issue_description: The issue requested replacing Chatwoot with a custom native Rust omnichannel chat system. However, based on the codebase audit, an omnichannel chat system is already natively implemented in Rust. We verified the database schema (migrations 233, 150, 1001, etc.), the webhook endpoints (src/server/api/omnichannel_webhook.rs), the UI flow (triage.html and its integration tests in src/e2e/omni_inbox_triage.mock-contract.ts), and the agent orchestration hooks. The Chatwoot dependency has already been removed (no mentions in the repository codebase). Therefore, the requested work is already completed.
issue_priority: P1
issue_category: implementation
issue_type: backend
issue_label: omnichannel
assignees: []
outcome: no_work
