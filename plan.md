1. **Define `FileSystemProvider` interface**
   - Create `srcs/server/tools/hybridfsmcp/provider.go`.
   - Add `package hybridfsmcp`.
   - Define `FileSystemProvider` interface with `ReadFile`, `WriteFile`, and `ListDir` methods taking `context.Context` and path.

2. **Implement `LocalFSProvider` and `CloudFSProvider`**
   - Create `srcs/server/tools/hybridfsmcp/local_provider.go`
     - Struct `LocalFSProvider` with `basePath string`.
     - Implement `ReadFile`, `WriteFile`, `ListDir`.
     - Ensure path bounding (prevent directory traversal outside `basePath`).
   - Create `srcs/server/tools/hybridfsmcp/cloud_provider.go`
     - Struct `CloudFSProvider` with `basePath string`.
     - Implement methods. It should extract `auth.Claims` from the `context.Context` and scope the file operations to `filepath.Join(basePath, claims.OrganizationID)`. If there are no claims, it should return an error.

3. **Implement MCP Server**
   - Create `srcs/server/tools/hybridfsmcp/mcp.go`.
   - The MCP server needs a factory method `NewHybridFSMCPServer(ctx context.Context, provider FileSystemProvider) *Server` (or similar depending on MCP interfaces in `srcs/server/agents/mcp`).
   - The server must expose standard tools: `read_file`, `write_file`, `list_directory`.
   - Create a factory `NewFileSystemProvider(ctx context.Context, basePath string) FileSystemProvider` that returns `LocalFSProvider` if `OHC_STANDALONE` is true, otherwise `CloudFSProvider`.

4. **Implement Tests**
   - Create `srcs/server/tools/hybridfsmcp/provider_test.go` to test `LocalFSProvider` and `CloudFSProvider`.
   - Create `srcs/server/tools/hybridfsmcp/mcp_test.go` to test the MCP server and tool execution.
   - Maintain high code coverage (>90%).

5. **Generate `BUILD.bazel`**
   - Run `bazelisk run //:gazelle` to generate `BUILD.bazel` for `srcs/server/tools/hybridfsmcp`.

6. **Pre-commit and Submit**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
   - Mark the `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` status to DONE and agent to Jules.
   - Run tests `bazelisk test //srcs/server/...`
