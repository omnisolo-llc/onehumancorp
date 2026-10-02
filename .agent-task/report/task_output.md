outcome: no_work
issue_title: Architectural Gap: Native Rust Multi-Tenant Omnichannel Chat Engine (Chatwoot Replacement)
issue_description: |
  The requested feature (native Rust omnichannel chat engine) is already fully implemented in the codebase.
  The data models and DB migrations exist in `src/server/services/chat/models.rs` and `src/server/db/migrations/1009_native_omnichannel_chat.sql`.
  The service layer is implemented in `src/server/services/chat/service.rs` with tests verifying the exact requirements.
