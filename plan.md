1. **Change mission state**
   - Update `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to indicate `IN_PROGRESS`.
2. **Create interface and providers**
   - In `srcs/server/tools/hybridfsmcp/provider.go`, define `mcp.FileSystemProvider` interface with `ReadFile`, `WriteFile`, `ListDir`, and `SearchFiles`.
   - Implement `LocalFSProvider` (with path bounding to a workspace directory).
   - Implement `CloudFSProvider` (with tenant-scoped paths utilizing `auth.Claims` and `auth.ClaimsFromContext(ctx)`).
3. **Create the MCP server**
   - Expose tools (`read_file`, `write_file`, `list_directory`, `search_files`) in the MCP server using the `FileSystemProvider`.
   - Include a factory to instantiate the right provider depending on `OHC_STANDALONE` or `OHC_MULTITENANT` mode.
4. **Implement unit tests**
   - Create `srcs/server/tools/hybridfsmcp/provider_test.go` and `srcs/server/tools/hybridfsmcp/server_test.go` with tests reaching >90% coverage.
5. **Add to Bazel BUILD**
   - Run `bazelisk run //:gazelle` and adjust `BUILD.bazel` to include new packages.
6. **Execute Pre-commit tests**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
7. **Change mission state**
   - Update `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to indicate `DONE`.
8. **Submit the PR**
   - Commit and push changes.
