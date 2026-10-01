issue_title: "#36926: Implement Native Rust Omnichannel Chat System"
issue_description: |
  The codebase already contains `src/server/migrations/1009_native_omnichannel_chat.sql` for the database schema, as well as `src/server/services/chat/service.rs` providing `ChatService` repository functions. However, the project context (`docs/research/native_migration_and_remediation.md` and related documents) specifies an ongoing migration and remediation phase, with numerous capabilities (like additional chat channels, advanced integrations, and generic UI restyling) explicitly deferred until the core migration gates and safety metrics are met. Implementing the Axum endpoints, integrating AI triage into the chat service, and completing the Flutter/Tauri mobile frontend UI for this feature are deferred.
  Superpowers skill provenance: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
outcome: blocked
