1. **Design and Implement `FileSystemProvider` Interface**
   - Create `srcs/server/tools/hybridfsmcp/provider.go`.
   - Define `FileSystemProvider` interface with `ReadFile`, `WriteFile`, and `ListDir` methods.

2. **Implement `LocalFSProvider` (Standalone Mode)**
   - Create `srcs/server/tools/hybridfsmcp/local.go`.
   - Implement path bounding using `filepath.Clean` to ensure we don't traverse outside of the allowed workspace directory.

3. **Implement `CloudFSProvider` (Cloud Mode)**
   - Create `srcs/server/tools/hybridfsmcp/cloud.go`.
   - Implement tenant-scoping. Rely on `auth.Claims.OrganizationID`. Validate string equality + prefix matching for secure path scoping.

4. **Implement MCP Server / Tools Factory**
   - Create `srcs/server/tools/hybridfsmcp/server.go`.
   - Check `OHC_STANDALONE` or `OHC_MULTITENANT` to instantiate `LocalFSProvider` or `CloudFSProvider`.
   - Expose MCP tools (`read_file`, `write_file`, `list_directory`, `search_files`).

5. **Update Mission File**
   - Update `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` status to `IN_PROGRESS` and claim the task as the `Implementer` agent.

6. **Implement Tests**
   - Create `srcs/server/tools/hybridfsmcp/provider_test.go` and achieve >90% coverage for local bounds and cloud tenant-scoping.
   - Run `go test` and `bazelisk test`. Use Gazelle to auto-generate `BUILD.bazel`.

7. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
   - Run Gazelle.
   - Run Go formatting.
   - Run test suite again.

8. **Submit Changes**
   - Submit the PR, marking mission as `DONE`.
