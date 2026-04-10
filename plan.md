1. **Explore context & requirements**
   - Read the `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to understand the goal.
   - We need an interface `mcp.FileSystemProvider` with `ReadFile`, `WriteFile`, `ListDir`.
   - We need a `LocalFSProvider` (bounded to a path) and `CloudFSProvider` (tenant-scoped).
   - We need an MCP server wrapping this provider.
   - We need factory logic to determine cloud vs local via `OHC_MULTITENANT`.

2. **Implement the FileSystemProvider logic**
   - Create a module under `srcs/server/tools/hybridfsmcp`.
   - Add `FileSystemProvider` interface in `fs_provider.go`.
   - Add `LocalFSProvider` in `local_fs.go`.
   - Add `CloudFSProvider` in `cloud_fs.go` that accesses tenant id via `auth.ClaimsFromContext()`.
   - Add factory function `NewFileSystemProvider` in `factory.go`.
   - Add MCP server `FileSystemMCPServer` in `mcp_server.go` wrapping the tools.

3. **Verify the implementation**
   - Run tests to achieve > 90% test coverage using `mcp_server_test.go`.
   - Generate BUILD file using Gazelle: `bazelisk run //:gazelle -- update srcs/server/tools/hybridfsmcp`.
   - Run Bazel tests with coverage: `bazelisk coverage //srcs/server/tools/hybridfsmcp/...`.
   - Update `2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` status to DONE.

4. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
   - Run the pre commit instructions and check for any remaining actions.

5. **Submit the PR**
   - Submit the PR using the `submit` tool.
