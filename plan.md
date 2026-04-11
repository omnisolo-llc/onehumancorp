1. **Update Mission File:**
   - Replace the existing `status: PENDING` and `agent: Researcher` tags with `status: "IN_PROGRESS"` and `agent: Implementer` in `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` using the `replace_with_git_merge_diff` tool.
   - Verify with `cat` using `run_in_bash_session`.

2. **Define `FileSystemProvider` Interface and Implementations:**
   - Use `write_file` to create `srcs/server/tools/hybridfsmcp/provider.go`.
   - Define `FileSystemProvider` interface with methods: `ReadFile(ctx context.Context, path string) ([]byte, error)`, `WriteFile(ctx context.Context, path string, data []byte) error`, `ListDir(ctx context.Context, path string) ([]string, error)`.
   - Implement `LocalFSProvider` taking a `baseDir`. Ensure path traversal checks (must be within `baseDir`, checking `strings.HasPrefix(fullPath, baseDir + string(filepath.Separator))`).
   - Implement `CloudFSProvider` taking a `baseDir`. Similar logic but tenant-scoped via `auth.ClaimsFromContext(ctx)`. `OrganizationID` determines the subdirectory within `baseDir`. Note: The implementation should map `path` into `<baseDir>/<OrganizationID>/<path>`. If the underlying provider inherently resolves absolute paths against the base directory, pass only the relative scoped path (e.g. `<OrganizationID>/<path>`) to the delegate. Here we'll just write directly to the local FS for the Cloud mock using tenant-scoped paths.
   - Implement `NewFileSystemProvider(isLocal bool, baseDir string) FileSystemProvider` factory.
   - Verify creation with `cat` using `run_in_bash_session`.

3. **Implement MCP Server for File System:**
   - Use `write_file` to create `srcs/server/tools/hybridfsmcp/mcp.go`.
   - Define `Tool` struct with `Name`, `Description`, `InputSchema` (`json.RawMessage` from `encoding/json`).
   - Implement `HybridFSMCP` struct containing a `FileSystemProvider`.
   - Implement `ListTools() []Tool` returning tools: `read_file`, `write_file`, `list_directory`. Use `json.RawMessage` for `InputSchema` per memory rules.
   - Implement `CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error)` routing tool execution logic to `FileSystemProvider` interface methods.
   - Verify creation with `cat` using `run_in_bash_session`.

4. **Implement Provider Tests:**
   - Use `write_file` to create `srcs/server/tools/hybridfsmcp/provider_test.go`.
   - Write tests for `provider.go` covering `LocalFSProvider`, `CloudFSProvider` path traversal constraints and correct file system read/write operations.
   - Verify creation with `cat` using `run_in_bash_session`.

5. **Implement MCP Server Tests:**
   - Use `write_file` to create `srcs/server/tools/hybridfsmcp/mcp_test.go`.
   - Write tests for `mcp.go` to verify `ListTools` returns the correct schemas for file operations, and `CallTool` correctly invokes `read_file`, `write_file`, and `list_directory` on the `FileSystemProvider`.
   - Verify creation with `cat` using `run_in_bash_session`.
   - Run `bazelisk test //srcs/server/tools/hybridfsmcp/...` using `run_in_bash_session` to verify test coverage.

6. **Create BUILD.bazel:**
   - Use `write_file` to create `srcs/server/tools/hybridfsmcp/BUILD.bazel`.
   - Verify with `cat` using `run_in_bash_session`.
   - Run Gazelle (`bazelisk run //:gazelle`) using `run_in_bash_session`.

7. **Finalize Verification and Update Mission File:**
   - Run `bazelisk test //srcs/server/...` for the backend, and run `cd srcs/app && flutter test` followed by `git reset HEAD . && git clean -xfd` for the app to fully test across the entire repository to ensure no regressions were introduced.
   - Update `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to `status: "DONE"` using the `replace_with_git_merge_diff` tool.
   - Verify with `cat` using `run_in_bash_session`.

8. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

9. **Submit PR:**
   - Call `submit` with PR title `Integrate Hybrid File System MCP Server`.
