outcome: blocked
issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  The issue requested the implementation of a custom native Rust omnichannel chat system to replace Chatwoot, including database migrations with RLS and CRUD services. Upon investigation, these requirements have already been implemented:
  - Database schema and RLS policies are complete in src/server/migrations/1009_native_omnichannel_chat.sql.
  - The core Rust service and models exist in src/server/services/chat/service.rs and src/server/services/chat/models.rs.
  - Further models are also defined in src/server/domain/repository/omnichannel_repo.rs and src/server/services/omnichannel_service.rs.
  - Websocket support for omnichannel is implemented (e.g., src/server/api/unified_ws.rs).

  Since the codebase already satisfies the issue's requirements, no further implementation is necessary and this issue is blocked.

  Superpowers skill provenance: 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md)
