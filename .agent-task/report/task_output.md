issue_title: "Implement Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  **Superpowers skill provenance: 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md)**

  The user asked to implement a Native Rust Omnichannel Chat System to replace Chatwoot (Issue #36883).
  However, based on an audit of the codebase:
  1. The core schema models and the service implementation (`src/server/services/chat/models.rs` and `src/server/services/chat/service.rs`) for the native Rust omnichannel chat system *already exist* in the repository and are populated with the required fields (like `tenant_id`).
  2. A previous Chatwoot removal project (`docs/superpowers/plans/2026-07-13-chatwoot-removal.md`) was already completed, and a residue test (`deploy/tests/no_chatwoot_residue_test.sh`) guarantees Chatwoot is gone.
  3. Based on the OneHumanCorp execution guidelines, we must respect the current implementation. Since Chatwoot is completely removed and the native Rust omnichannel foundational structures (as requested in the implementation prompt) are already present, this task does not require new code creation but rather verifies the existing state. The specific acceptance criteria concerning the schema migration, domain models and services have been satisfied.
  4. There is no actionable work to do because the migration away from Chatwoot is already verified as complete. No safe, well-scoped implementation follows from the repository state as the feature requested is already present.
outcome: blocked
