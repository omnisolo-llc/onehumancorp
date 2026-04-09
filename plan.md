1.  **Define Structure and Interface:**
    *   Create `srcs/server/tools/hybridfsmcp/provider.go`.
    *   Define `FileSystemProvider` interface: `ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)`, `WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error`, `ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)`.
    *   Define `HybridFSMCP` struct.
2.  **Implement Local & Cloud Providers:**
    *   Create `srcs/server/tools/hybridfsmcp/local.go`. Implement `LocalFSProvider` using standard `os` and `filepath` tools. Use `filepath.Rel` to prevent directory traversal and enforce boundary checking within a given root workspace directory. It will ignore `auth.Claims`.
    *   Create `srcs/server/tools/hybridfsmcp/cloud.go`. Implement `CloudFSProvider`. It relies on `claims.OrganizationID` to securely chroot directory requests. It will build virtual tenant paths like `filepath.Join(rootDir, claims.OrganizationID, path)`. Ensure strictly `claims != nil` and `claims.OrganizationID != ""`.
3.  **Implement MCP Tool Handler:**
    *   Create `srcs/server/tools/hybridfsmcp/mcp.go`.
    *   Define `Tool` struct (Name, Description, InputSchema).
    *   Implement `ListTools() []Tool` to expose `read_file`, `write_file`, and `list_directory`.
    *   Implement `CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error)` which extracts `claims := auth.ClaimsFromContext(ctx)`, routes the request to the configured `FileSystemProvider`, and returns standard MCP responses.
    *   Create a factory method `NewHybridFSMCP(rootDir string)` that checks `os.Getenv("OHC_STANDALONE") == "true"` to instantiate either `LocalFSProvider` or `CloudFSProvider`.
4.  **Verify Implementations:**
    *   Use `cat` to read the contents of `srcs/server/tools/hybridfsmcp/provider.go`, `srcs/server/tools/hybridfsmcp/local.go`, `srcs/server/tools/hybridfsmcp/cloud.go`, and `srcs/server/tools/hybridfsmcp/mcp.go` to verify they were created correctly.
5.  **Implement Unit Tests for the HybridFS MCP:**
    *   Create `srcs/server/tools/hybridfsmcp/mcp_test.go`.
    *   Implement `TestLocalFSProvider_PathTraversal` testing traversal block (e.g. `../../../etc/passwd`).
    *   Implement `TestCloudFSProvider_Isolation` testing missing claims and cross-tenant leakage.
    *   Implement `TestCallTool_Routing` testing happy paths for all 3 tools.
6.  **Verify Tests:**
    *   Use `cat` to read the contents of `srcs/server/tools/hybridfsmcp/mcp_test.go` to verify they were created correctly.
7.  **Gazelle:**
    *   Run `~/go/bin/bazelisk run //:gazelle`.
8.  **Verify Gazelle:**
    *   Run `git diff` to verify the generated BUILD file modifications.
9.  **Run Tests:**
    *   Execute `~/go/bin/bazelisk test //srcs/server/... //srcs/app/... --test_output=errors --jobs=4 --local_test_jobs=1`.
10. **Update Mission File:**
    *   Mark `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` as `status: DONE` using `sed`.
    *   Use `cat` to verify the file was successfully updated.
11. **Pre-commit:**
    *   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
12. **Submit:**
    *   Submit pull request with the validated implementation.
