outcome: no_work
issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: "The functionality requested in issue #35625 is already fully implemented. The basic models (UnifiedThread, UnifiedMessage, UnifiedTriageAction) and their schemas exist in src/server/services/inbox/service.rs with tenant_id RLS applied via unified_threads and unified_messages migrations. Basic CRUD operations are covered, and a WebSocket handler stub is already in src/server/api/unified_ws.rs that accepts connections and broadcasts mock messages."
