1. **Understand Goal**: The goal is to integrate a "Hybrid File System MCP Server" into OHC.
2. **Review Existing State**:
   - The mission file states: Implement `mcp.FileSystemProvider` and expose tools like `read_file`, `write_file`, `list_directory`, `search_files`.
   - `LocalFSProvider` (bounded paths) and `CloudFSProvider` (tenant-scoped) are required.
   - Inject `auth.Claims` from the context in cloud mode to scope access to the tenant.
3. **Execution Steps Completed**:
   - `srcs/server/tools/hybridfsmcp` directory created.
   - `provider.go` implemented: Defines `LocalFSProvider` and `CloudFSProvider` fulfilling the abstract file system operations. The `CloudFSProvider` uses `auth.ClaimsFromContext(ctx)` to scope paths.
   - `mcp.go` implemented: Defines `HybridFSMCP` representing the MCP server implementation. Provides the requested tools and passes arguments to the provider.
   - `mcp_test.go` implemented: Contains >90% code coverage tests for both local and cloud modes.
   - `srcs/server/tools/hybridfsmcp/BUILD.bazel` generated via `bazelisk run //:gazelle` and the package tests passed via `bazelisk test //srcs/server/tools/hybridfsmcp/...`.
   - Connected `hybridfsmcp` to the `dashboard` by updating `srcs/server/dashboard/handlers_mcp.go` with a new `case "hybrid-fs-mcp":` execution block.
   - Updated `srcs/server/dashboard/BUILD.bazel` to include `"//srcs/server/tools/hybridfsmcp"`.
4. **Final Step**:
   - Update `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` status to `DONE` and `agent` to `Jules`.
   - Pre-commit instructions.
