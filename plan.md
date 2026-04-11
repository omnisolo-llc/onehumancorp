1. **Define FileSystemProvider interface**
   - Create `srcs/server/agents/mcp/fs_provider.go`
   - Define `FileSystemProvider` interface with `ReadFile`, `WriteFile`, `ListDir` methods.
   - Add `mcp_server_hybrid_fs.go` that will expose standard tools (`read_file`, `write_file`, `list_directory`, `search_files`).

2. **Implement LocalFSProvider**
   - Create `srcs/server/agents/mcp/local_fs.go`.
   - Map directly to the local file system.
   - Ensure paths are safely bounded using `strings.HasPrefix(fullPath, baseDir + string(filepath.Separator))`.
   - Properly handle exact matches `fullPath == baseDir`.
   - Prevent absolute path bypasses in `filepath.Join` via `strings.TrimPrefix(filepath.Clean(reqPath), "/")`.

3. **Implement CloudFSProvider**
   - Create `srcs/server/agents/mcp/cloud_fs.go`.
   - Tenant-scoped file operations using `auth.ClaimsFromContext(ctx).OrganizationID`.
   - Construct a virtual directory scope `/tenant-{org_id}/` (could map to a shared persistent volume like `/mnt/pvc/tenant-{org_id}`).

4. **MCP Server Integration**
   - Define tools in the `mcp_server_hybrid_fs.go` file using `mcp.Tool` structs.
   - Parse `json.RawMessage` correctly for tool arguments.
   - Use `os.Getenv("OHC_MULTITENANT")` and `os.Getenv("OHC_STANDALONE")` to instantiate the correct provider in a factory method `NewHybridFSProvider(baseDir string) FileSystemProvider`.

5. **Testing & Coverage**
   - Create `srcs/server/agents/mcp/fs_provider_test.go`.
   - Test path traversal prevention for local provider.
   - Test multitenancy directory resolution for cloud provider by injecting `auth.ClaimsContextKeyForTest` directly into `context.WithValue`.
   - Ensure coverage >90%.

6. **Repository Verification**
   - Run system-wide test command: `bazelisk test //srcs/server/...` and `cd srcs/app && flutter test && git clean -xfd`
   - Update mission status to `DONE`.
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
