outcome: no_work
issue_title: Implement Custom Rust Omnichannel Chat System to Replace Chatwoot
issue_description: "The functionality requested in issue #35553 is already fully implemented. The basic models (UnifiedThread, UnifiedMessage, UnifiedTriageAction) and their schemas exist in src/server/services/inbox/service.rs with tenant_id isolation applied via SQLx. Basic CRUD operations are covered, and a WebSocket handler is already implemented in src/server/api/unified_ws.rs."
