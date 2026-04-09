1. **Create `FileSystemProvider` Interface and Types:**
   - In `srcs/server/tools/hybridfsmcp/provider.go`, define an interface `FileSystemProvider` with methods `ReadFile(path string) ([]byte, error)`, `WriteFile(path string, data []byte) error`, and `ListDir(path string) ([]string, error)`.
   - Also, define a factory function `NewProvider(ctx context.Context, baseDir string) FileSystemProvider` that returns a `LocalFSProvider` if `OHC_STANDALONE=true` (or if it's the default logic and not `OHC_MULTITENANT`) or `CloudFSProvider` otherwise.

2. **Implement `LocalFSProvider`:**
   - In `srcs/server/tools/hybridfsmcp/local_provider.go`, implement the interface by interacting with the local file system (using `os` and `path/filepath`). Path bounds checking will be done using `filepath.Clean` to ensure the requested path is within the allowed workspace directory.

3. **Implement `CloudFSProvider`:**
   - In `srcs/server/tools/hybridfsmcp/cloud_provider.go`, implement the interface scoped to a tenant. It extracts `auth.Claims` from the `context.Context` (via `auth.ClaimsFromContext`), reads the `OrganizationID`, and prefixes paths with the tenant ID (e.g., `<baseDir>/<OrganizationID>/<path>`).

4. **Implement `MCPServer`:**
   - In `srcs/server/tools/hybridfsmcp/server.go`, expose an object that uses `FileSystemProvider` to handle tools like `read_file`, `write_file`, and `list_directory`. We can format results matching what `mcp.FormatExecutionResult` expects, or just provide typed tool methods.

5. **Write Unit Tests:**
   - In `srcs/server/tools/hybridfsmcp/provider_test.go`, test local bounding, cloud tenant isolation (using `context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})`), and the factory function. Aim for >90% coverage.

6. **Create `BUILD.bazel` for `hybridfsmcp`:**
   - Create `srcs/server/tools/hybridfsmcp/BUILD.bazel` to define `go_library` and `go_test` for the new package.

7. **Pre-commit steps:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

8. **Submit:**
   - Submit the PR with the title "🗺️ Guide: [new onboarding feature]".
