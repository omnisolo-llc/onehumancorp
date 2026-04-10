1. **Mark Mission In-Progress**
   - Update `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to have `status: IN_PROGRESS` and `agent: Implementer` using `replace_with_git_merge_diff` or `run_in_bash_session` with sed.
   - Verify changes using `git diff`.

2. **Create `srcs/server/tools/hybridfsmcp/provider.go`**:
    - Use the `write_file` tool to define `FileSystemProvider` interface with `ReadFile(ctx, path)`, `WriteFile(ctx, path, data)`, `ListDir(ctx, path)` methods. Implement `LocalFSProvider` taking a base workspace path, preventing path traversal using `target == base || strings.HasPrefix(target, base+string(filepath.Separator))`. Implement `CloudFSProvider` pulling tenant info using `auth.ClaimsFromContext(ctx)` to prepend tenant-specific directories, and prevent traversal. Implement a `NewProvider(ctx)` factory that checks `os.Getenv("OHC_MULTITENANT") == "true"` to determine whether to return `CloudFSProvider` or `LocalFSProvider`. Use `os.Getenv("OHC_FS_ROOT")` as base directory.

3. **Verify `provider.go` creation**:
    - Use `cat` or `read_file` to verify the contents of `srcs/server/tools/hybridfsmcp/provider.go`.

4. **Create `srcs/server/tools/hybridfsmcp/server.go`**:
    - Use the `write_file` tool to create `HybridFSServer` wrapping `FileSystemProvider`. Expose MCP tools: `read_file`, `write_file`, `list_directory`. Format outputs correctly.

5. **Verify `server.go` creation**:
    - Use `cat` or `read_file` to verify the contents of `srcs/server/tools/hybridfsmcp/server.go`.

6. **Create tests in `srcs/server/tools/hybridfsmcp/provider_test.go` and `server_test.go`**:
    - Use `write_file` tool to create tests. Test `LocalFSProvider` ensuring bounds and read/write. Test `CloudFSProvider` using `context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)` to inject test auth claims. Ensure >90% test coverage.

7. **Verify test files creation**:
    - Use `cat` or `read_file` to verify the contents of `srcs/server/tools/hybridfsmcp/provider_test.go` and `srcs/server/tools/hybridfsmcp/server_test.go`.

8. **Create `srcs/server/tools/hybridfsmcp/BUILD.bazel`**:
    - Use `write_file` tool to add `go_library` for `hybridfsmcp` and `go_test` for the tests.

9. **Verify `BUILD.bazel` creation**:
    - Use `cat` or `read_file` to verify the contents of `srcs/server/tools/hybridfsmcp/BUILD.bazel`.

10. **Mark Mission Done**:
    - Update `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to `status: DONE` using `run_in_bash_session` with sed.
    - Verify with `git diff`.

11. **Run tests**:
    - Run `bazelisk test //srcs/server/tools/hybridfsmcp/... --test_output=all` to ensure everything compiles and tests pass.

12. **Complete pre commit steps**
    - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

13. **Submit PR**:
    - Use `submit` to commit the code and submit.
