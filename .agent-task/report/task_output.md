issue_title: "Architect Native Rust Omnichannel Chat to Replace Chatwoot"
issue_description: |
  **Verified trace limitations**:
  - Codebase was thoroughly checked for "chatwoot" but found no current dependencies or occurrences that needed replacing.
  - The `src/server/integrations/omnichannel` package, `src/server/domain/chat/mod.rs` definitions, `src/server/services/chat/service.rs`, and PostgreSQL migration schemas (233 and 1009) already implement the native rust omnichannel chat domain with `Inbox`, `Conversation`, `Message`, and `Contact` mapped to PostgreSQL, with RLS applied to `tenant_id`.
  - The `MessageRouter` logic (`route_incoming_message`) is already in place, establishing `Conversation`s and routing them correctly.
  - Test suites (`cargo test -p server_integrations_omnichannel`) run and pass successfully.

  The current state of the repository has already implemented the required architectural requirements for replacing the legacy Chatwoot integration. Thus, this issue corresponds to a no_work finding because the described omnichannel chat features are already present and fully functional in the system as of this execution.
outcome: no_work
