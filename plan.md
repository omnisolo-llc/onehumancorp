1. **Define the `FileSystemProvider` interface.**
   - Create `srcs/server/agents/mcp/fsprovider.go`.
   - Define `FileSystemProvider` interface with `ReadFile`, `WriteFile`, and `ListDir`.

2. **Implement `LocalFSProvider` and `CloudFSProvider`.**
   - In `srcs/server/tools/hybridfsmcp/local_provider.go`, implement `LocalFSProvider`. Enforce path bounding to prevent traversal attacks.
   - In `srcs/server/tools/hybridfsmcp/cloud_provider.go`, implement `CloudFSProvider`. Enforce path bounding using `auth.Claims` to scope paths by `organization_id` in cloud environments.
   - Wait, actually, the mission says: "Abstract the file writing and reading logic behind an interface `mcp.FileSystemProvider` with methods `ReadFile`, `WriteFile`, `ListDir`... Implement `LocalFSProvider` (with path bounding to a workspace dir) and `CloudFSProvider` (tenant-scoped)". I will put the interface in `srcs/server/agents/mcp/fsprovider.go` or `srcs/server/tools/hybridfsmcp/provider.go`. Let's put everything in `srcs/server/tools/hybridfsmcp/` to keep it modular, and define the interface there or just in `mcp` package. I'll put the interface in `mcp` since it's an abstract provider.
   - Create `srcs/server/agents/mcp/fs_provider.go` with `FileSystemProvider` interface.

3. **Implement MCP Server for the FS Tools.**
   - Create `srcs/server/tools/hybridfsmcp/mcp.go`.
   - Implement `HybridFSServer` that takes a `FileSystemProvider`.
   - Expose methods: `HandleReadFile`, `HandleWriteFile`, `HandleListDir` (or a generic `CallTool` method).
   - Implement factory logic `NewHybridFSServer(ctx context.Context, baseDir string)` that checks `os.Getenv("OHC_MULTITENANT") == "true"` to return `CloudFSProvider` or `LocalFSProvider`.
   - Include path bounds checking using `filepath.Clean` and `strings.HasPrefix(target, base+string(filepath.Separator)) || target == base`.

4. **Write Tests.**
   - Write `srcs/server/tools/hybridfsmcp/mcp_test.go` checking path bounds (path traversal), standard read/write, cloud mode tenant isolation.
   - Verify 90% test coverage for this module.

5. **Create BUILD.bazel for new package.**
   - Add `srcs/server/tools/hybridfsmcp/BUILD.bazel`.

6. **Pre-commit Instructions.**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

7. **Submit Change.**
   - Update mission file `status: DONE` before the pre-commit step, and submit.
