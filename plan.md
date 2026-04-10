# 1. Mission Update
Update the status of the mission `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to `IN_PROGRESS` and the agent to `jules`.
Check with `grep`.

# 2. Add File System Provider Interface
Create `srcs/server/tools/hybridfsmcp/provider.go`.
Define `FileSystemProvider` interface with methods:
- `ReadFile(ctx context.Context, path string) ([]byte, error)`
- `WriteFile(ctx context.Context, path string, content []byte) error`
- `ListDir(ctx context.Context, path string) ([]string, error)`

# 3. Implement LocalFSProvider
Create `srcs/server/tools/hybridfsmcp/local_fs.go`.
Implement `FileSystemProvider` backed by local OS file system, bound to a specific workspace directory.
Path bounding must ensure files can't be read/written outside the workspace.

# 4. Implement CloudFSProvider
Create `srcs/server/tools/hybridfsmcp/cloud_fs.go`.
Implement `FileSystemProvider` taking tenant scope from `auth.ClaimsFromContext(ctx)` into account. For now, it will map to a tenant-specific sub-directory inside a base cloud storage path, effectively virtualizing the local FS per tenant.

# 5. Add Factory Logic
Create `srcs/server/tools/hybridfsmcp/factory.go`.
Create `NewFileSystemProvider(workspace string, cloudBase string)` that returns either `LocalFSProvider` or `CloudFSProvider` depending on `os.Getenv("OHC_STANDALONE")`.

# 6. Implement MCP Server/Tool Handlers
Create `srcs/server/tools/hybridfsmcp/mcp_server.go`.
Create an MCP server abstraction that exposes `read_file`, `write_file`, and `list_directory` tools which use the `FileSystemProvider`.
Ensure `InputSchema` is `json.RawMessage`.

# 7. Write Unit Tests
Create `srcs/server/tools/hybridfsmcp/provider_test.go` and `srcs/server/tools/hybridfsmcp/mcp_server_test.go` to test functionality and ensure >90% code coverage.
Run tests with `bazelisk test //srcs/server/tools/hybridfsmcp/...`. Check code coverage using go test cover profile.

# 8. Complete pre-commit steps
Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

# 9. Update Mission and Create PR
Update the status of `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to `DONE`.
Create a GitHub Pull Request with the changes.
