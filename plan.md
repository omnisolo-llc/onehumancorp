1. **Fix missing `search_files` tool**
   - In `srcs/server/tools/hybridfsmcp/mcp.go`, add `SearchFiles(ctx context.Context, query string) ([]string, error)` to `FileSystemProvider`.
   - Implement `SearchFiles` in `LocalFSProvider` and `CloudFSProvider`.
   - Add `search_files` to `ListTools()`.
   - Add `case "search_files":` in `CallTool()`.
2. **Update tests in `mcp_test.go`**
   - Add test cases for `SearchFiles` in `TestLocalFSProvider`, `TestCloudFSProvider`, and `TestHybridFSMCP`.
3. **Run tests**
   - `bazelisk test //srcs/server/tools/hybridfsmcp:hybridfsmcp_test`
4. **Complete pre-commit steps**
   - Initiate memory recording.
5. **Submit**
