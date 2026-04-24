1. **Prune Redundant Logs:**
   - Modified `srcs/server/db/database.go` to change `slog.Info` to `slog.Debug` for database connection messages, as well as migration execution messages. (Completed)
2. **Update `BUILD.bazel`:**
   - Updated `srcs/server/db/BUILD.bazel` to include missing migrations, removed non-existent migrations, and resolved test breakages due to incorrect syntax passed down to SQLite schemas. (Completed)
3. **Circular dependencies and bloated handlers:**
   - Extracted mesh-related handlers (`handleMeshBroadcast`, `handleMeshV2Broadcast`, `handleMeshDirect`, `handleMeshMailbox`) from `srcs/server/dashboard/server.go` to `srcs/server/dashboard/handlers_mesh_extra.go`. Refactored `srcs/server/dashboard/BUILD.bazel` to accommodate. (Completed)
4. **Pre-commit steps:**
   - Use `pre_commit_instructions` tool to run the necessary checks, including tests and verifications.
5. **Completion:**
   - Use `submit` tool to output a message with the `issue_id: 5733`.
