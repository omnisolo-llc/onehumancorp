1. **Define `FileSystemProvider` interface** in a new package `srcs/server/tools/hybridfsmcp/provider.go`. It should have methods: `ReadFile`, `WriteFile`, `ListDir`.
2. **Implement `LocalFSProvider`**: Maps directly to local file system with safety bounds (e.g., chroot to a base directory).
3. **Implement `CloudFSProvider`**: Scopes access based on `auth.Claims` (e.g., `tenant_id`). It could be backed by local FS for now but simulates a tenant-scoped path or a mock S3-backed one. Given the prompt says "Maps to Tenant-scoped Kubernetes Persistent Volumes or a virtualized S3-backed file system interface", we can implement it over the local FS but prepending tenant ID to the path.
4. **Implement MCP server `HybridFSProxy`** in `srcs/server/tools/hybridfsmcp/mcp.go`: Exposes standard filesystem tools (`read_file`, `write_file`, `list_directory`, `search_files`).
5. **Add Factory logic**: based on `OHC_MULTITENANT` and `OHC_STANDALONE` modes, instantiate either local or cloud provider.
6. **Add Unit Tests** for both Local and Cloud providers in `srcs/server/tools/hybridfsmcp/mcp_test.go` ensuring >90% coverage.
7. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
8. **Submit**.
