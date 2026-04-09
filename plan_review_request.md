The mission is to implement a foundational schema change and a Go synchronization service interface for the Hybrid MCP RAG Protocol.

I will write a plan to accomplish the following:
1. Use `run_in_bash_session` with `sed -i` to change the status to `IN_PROGRESS` and prepend `agent: Jules` in the frontmatter of `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`.
2. Use `run_in_bash_session` with `cat << 'EOF' > srcs/server/db/migrations/032_hybrid_mcp_rag_sync.sql` to create the migration file containing `ALTER TABLE ADD COLUMN` for `sync_status` and `last_sync_at` on the `agent_memories` table. (As instructed in the mission file prompt, step 1)
3. Use `ls` and `cat` to verify the migration file was created and written correctly.
4. Use `run_in_bash_session` with `cat << 'EOF' > srcs/server/hub/rag_sync.go` to write the interface and OpenTelemetry setup for RAGSyncService. (As instructed in the mission file prompt, step 2 and 3)
5. Verify the Go file creation using `ls` and `cat`.
6. Use `run_in_bash_session` with `cat << 'EOF' > srcs/server/hub/rag_sync_test.go` to implement the unit test. (As instructed in the mission file prompt, Verification)
7. Verify creation using `ls srcs/server/hub/rag_sync_test.go`.
8. Verify changes by running `bazelisk run //:gazelle`. Then use `git status` and `cat srcs/server/hub/BUILD.bazel` to verify the generated build file.
9. Verify changes by running `bazelisk test //srcs/server/hub/...`.
10. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
11. Use `run_in_bash_session` with `sed -i` to update the mission file's status to `DONE`. Then use the `submit` tool to create the PR.
