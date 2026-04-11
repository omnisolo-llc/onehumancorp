1. **Update mission status:** Mark `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` as `IN_PROGRESS` and assign agent `Implementer`.
2. **Implement FS Providers**:
   - Create `srcs/server/agents/mcp/fs_provider.go`
   - Define interface `FileSystemProvider` with methods `ReadFile(ctx context.Context, path string) ([]byte, error)`, `WriteFile(ctx context.Context, path string, data []byte) error`, and `ListDir(ctx context.Context, path string) ([]string, error)`.
   - Implement `LocalFSProvider` that sanitizes paths (rejects any containing `..` and bounds to a safe dir, though we can just bound it to the absolute workspace root, we need to ensure safety bounds are met, specifically using `strings.Contains(path, "..")`).
   - Implement `CloudFSProvider` that scopes paths by `auth.ClaimsFromContext(ctx).OrganizationID`.
   - Implement a factory `NewFileSystemProvider(isStandalone bool, workspaceRoot string) FileSystemProvider`.
3. **Implement MCP Server for FS**:
   - Create `srcs/server/agents/mcp/fs_mcp.go`.
   - Implement an MCP wrapper similar to `BlobInspectorMCP` that exposes `read_file`, `write_file`, and `list_directory` tools.
   - Inject the `FileSystemProvider`.
   - Use `auth.ClaimsFromContext` to validate cloud mode access.
4. **Implement Tests**:
   - Create `srcs/server/agents/mcp/fs_provider_test.go` and `fs_mcp_test.go`.
   - Mock context using `context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)` for auth.
   - Achieve >90% test coverage.
5. **Update BUILD.bazel**: Update `srcs/server/agents/mcp/BUILD.bazel` to include the new files and update dependencies if needed.
6. **Testing and Verification**: Run `bazelisk test //srcs/server/agents/mcp/...` to verify. Run `./test.sh //...` globally.
7. **Complete pre-commit steps**: Call `pre_commit_instructions` tool.
8. **Finalize**: Update mission file to `status: DONE` and run global test suite.
