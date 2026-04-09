1. **Design `FileSystemProvider` Interface**
   - Create `srcs/server/tools/hybridfsmcp/provider.go`.
   - Define interface `FileSystemProvider` with `ReadFile`, `WriteFile`, `ListDir`, and `IsLocal() bool`.

2. **Implement `LocalFSProvider`**
   - Create `srcs/server/tools/hybridfsmcp/local_provider.go`.
   - Binds to a local workspace path (e.g. from environment variable or default `./workspace`).
   - Implement safety bounds to ensure no access outside the local workspace.

3. **Implement `CloudFSProvider`**
   - Create `srcs/server/tools/hybridfsmcp/cloud_provider.go`.
   - Mocks/implements a tenant-scoped environment by scoping files to `/tenant/{OrganizationID}/`.

4. **Implement `HybridFSMCP` Server**
   - Create `srcs/server/tools/hybridfsmcp/mcp.go`.
   - Similar to `DBInspectorMCP` and `BlobInspectorMCP`, implements `ListTools` and `CallTool`.
   - Implements tools: `read_file`, `write_file`, `list_directory`, `search_files`.
   - Extracts tenant `OrganizationID` from `auth.ClaimsFromContext(ctx)` if not in `IsLocal()` mode.

5. **Implement Tests**
   - Create `srcs/server/tools/hybridfsmcp/mcp_test.go` and `provider_test.go`.
   - Run tests `bazelisk test //srcs/server/tools/hybridfsmcp/...` to achieve > 90% coverage.
