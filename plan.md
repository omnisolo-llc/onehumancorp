1. **Change the mission status to IN_PROGRESS**
   - Execute `sed -i 's/status: PENDING/status: IN_PROGRESS/g' .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`
   - Execute `sed -i 's/agent: Researcher/agent: Implementer/g' .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`
   - Verify with `cat .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`.

2. **Define `FileSystemProvider` interface**
   - Create `srcs/server/tools/hybridfsmcp/provider.go`.
   - Define `FileSystemProvider` interface with methods `ReadFile(ctx context.Context, tenantID, path string) ([]byte, error)`, `WriteFile(ctx context.Context, tenantID, path string, data []byte) error`, `ListDir(ctx context.Context, tenantID, path string) ([]string, error)` and `SearchFiles(ctx context.Context, tenantID, path, pattern string) ([]string, error)`.
   - Verify creation with `cat srcs/server/tools/hybridfsmcp/provider.go`.

3. **Implement `LocalFSProvider`**
   - Create `srcs/server/tools/hybridfsmcp/local_provider.go`.
   - Implement `LocalFSProvider` struct that satisfies `FileSystemProvider`.
   - In `LocalFSProvider` methods, securely bound paths to `baseDir` using `filepath.Rel(baseDir, absPath)` and verify it does not equal `..` or start with `../` (using `filepath.ToSlash(rel)`) to prevent directory traversal outside the workspace, as per Memory rules.
   - Verify creation with `cat srcs/server/tools/hybridfsmcp/local_provider.go`.

4. **Implement `CloudFSProvider`**
   - Create `srcs/server/tools/hybridfsmcp/cloud_provider.go`.
   - Implement `CloudFSProvider` struct that satisfies `FileSystemProvider`.
   - In `CloudFSProvider` methods, securely scope paths via tenant identifiers. Validate `tenantID` using regex `^[a-zA-Z0-9_-]+$` before appending it to `baseDir` to create the tenant-specific virtual directory, as per Memory rules. Then use the same path-bounding logic as local provider to ensure the target path stays within the tenant's directory.
   - Verify creation with `cat srcs/server/tools/hybridfsmcp/cloud_provider.go`.

5. **Create Factory Method**
   - Create `srcs/server/tools/hybridfsmcp/factory.go`.
   - Implement `NewFileSystemProvider(isStandalone bool, baseDir string) FileSystemProvider` that returns `LocalFSProvider` if `isStandalone` is true, and `CloudFSProvider` otherwise.
   - Verify creation with `cat srcs/server/tools/hybridfsmcp/factory.go`.

6. **Create MCP Server Handler**
   - Create `srcs/server/tools/hybridfsmcp/mcp_server.go`.
   - Implement `FSMCPServer` struct wrapping `FileSystemProvider`.
   - Add methods exposing standard filesystem tools: `HandleReadFile(ctx context.Context, tenantID, reqPath string) ([]byte, error)`, `HandleWriteFile(ctx context.Context, tenantID, reqPath string, data []byte) error`, `HandleListDirectory(ctx context.Context, tenantID, reqPath string) ([]string, error)`, and `HandleSearchFiles(ctx context.Context, tenantID, reqPath, pattern string) ([]string, error)`.
   - Verify creation with `cat srcs/server/tools/hybridfsmcp/mcp_server.go`.

7. **Add Unit Tests for FS Providers and Server**
   - Create `srcs/server/tools/hybridfsmcp/provider_test.go` and `srcs/server/tools/hybridfsmcp/mcp_server_test.go`.
   - Implement specific test cases:
     - `LocalFSProvider`: path bounds success, directory traversal attempt (`../../etc/passwd`).
     - `CloudFSProvider`: valid tenant ID bounds success, invalid tenant ID regex failure, tenant isolation success.
     - `FSMCPServer`: test all handlers.
   - Verify creation with `ls -l srcs/server/tools/hybridfsmcp/`.

8. **Update BUILD.bazel**
   - Create `srcs/server/tools/hybridfsmcp/BUILD.bazel`.
   - Define `go_library` and `go_test` for the new package. Base package `github.com/onehumancorp/mono/srcs/server/tools/hybridfsmcp`.
   - Verify with `cat srcs/server/tools/hybridfsmcp/BUILD.bazel`.

9. **Run tests**
   - Run `bazelisk test //srcs/server/... --test_output=errors` to satisfy Completeness Rule and verify changes system-wide.
   - Run `go test -v -cover ./srcs/server/tools/hybridfsmcp/...` in bash session to ensure >90% coverage requirement is met.

10. **Pre-commit checks**
    - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

11. **Submit**
    - Execute `sed -i 's/status: IN_PROGRESS/status: DONE/g' .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`.
    - Verify with `cat .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`.
    - Submit PR.
