1.  **Mark Mission as In Progress:** Update `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` by setting its status to `IN_PROGRESS` and agent to `Implementer` using `run_in_bash_session` with `sed`.
2.  **Verify Mission Update:** Use `read_file` or `cat` via `run_in_bash_session` to verify the mission status was correctly updated.
3.  **Create Hybrid FS MCP Package Directory & Files:**
    - Use `write_file` to create `srcs/server/tools/hybridfsmcp/provider.go`. It will contain the `FileSystemProvider` interface and the implementations: `LocalFSProvider` (with path bounding to avoid traversal using `target == base || strings.HasPrefix(target, base+string(filepath.Separator))`) and `CloudFSProvider` (tenant-scoped via `auth.Claims` scoping paths to `organization_id`).
    - Use `write_file` to create `srcs/server/tools/hybridfsmcp/mcp.go`. It will implement an MCP interface exposing `read_file`, `write_file`, `list_directory`, and `search_files` tools. It will check the `OHC_MULTITENANT` environment variable to pick the provider. Both will use `OHC_FS_ROOT` if set.
    - Use `write_file` to create `srcs/server/tools/hybridfsmcp/BUILD.bazel` so it's a Bazel library.
    - Use `write_file` to create `srcs/server/tools/hybridfsmcp/mcp_test.go` checking both standalone and multi-tenant paths.
4.  **Verify New Files:** Use `read_file` and `list_files` to verify the creation and content of `provider.go`, `mcp.go`, `BUILD.bazel`, and `mcp_test.go`.
5.  **Mark Mission as DONE:** Update `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` by setting status to `DONE` via `sed`.
6.  **Verify Mission is DONE:** Use `read_file` or `cat` to verify the status update.
7.  **Run All Tests:** Run `bazelisk test //srcs/server/tools/hybridfsmcp/...` and ensure >90% test coverage using `bazelisk coverage //srcs/server/tools/hybridfsmcp/... --test_output=all`.
8.  **Complete pre-commit steps:** Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
9.  **Submit PR:** Submit the PR with the branch and title.
