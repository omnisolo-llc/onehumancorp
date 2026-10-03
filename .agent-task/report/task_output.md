outcome: no_work
issue_title: "Native Rust Omnichannel Chat Inbox System Implementation"
issue_description: |
  The Native Rust Omnichannel Chat System issue (#35944) is already complete in the current repository state and there is no remaining work.

  **Verification Evidence:**
  1. The Rust structs for the native omnichannel system exist in `src/server/integrations/omnichannel/src/models.rs`.
  2. The unified persistence models (`inbox`, `conversation`, `message`, `contact`) are implemented using `sea_orm` with tenant isolation.
  3. The core `ChatRepository` and `ChannelAdapter` interfaces and their implementations are present in `src/server/integrations/omnichannel/src/repository.rs` and `traits.rs`.
  4. The ingestion webhook router logic resides in `src/server/integrations/omnichannel/src/router.rs`.
  5. The migrations `233_chat_omnichannel.sql` and `1009_native_omnichannel_chat.sql` define the required Postgres tables (`chat_inboxes`, `chat_conversations`, `chat_messages`, etc.) with row-level security (`ENABLE ROW LEVEL SECURITY`) and tenant isolation policies.
  6. The `server_integrations_omnichannel` test suite passes successfully.
  7. A residue guard test `deploy/tests/no_chatwoot_residue_test.sh` confirms that the legacy Chatwoot integration codebase has already been fully removed.

  Because the native components are already built and tested, and Chatwoot has been fully extracted, I am closing this with a `blocked` (no_work) outcome per the OHC contract guidelines.
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
