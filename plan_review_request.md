The mission asks to create a `Hybrid File System MCP Server` (`hybridfsmcp`) that provides a `FileSystemProvider` interface, an MCP server using this provider, and implementations for both Standalone (`LocalFSProvider`) and Cloud (`CloudFSProvider`) mode. The `FileSystemProvider` interface needs `ReadFile`, `WriteFile`, and `ListDir`.

Proposed Plan:
1.  **Define `FileSystemProvider` interface**
    *   Create `srcs/server/tools/hybridfsmcp/provider.go`.
    *   Define `FileSystemProvider` with `ReadFile`, `WriteFile`, and `ListDir` methods.
2.  **Implement `LocalFSProvider` and `CloudFSProvider`**
    *   Create `srcs/server/tools/hybridfsmcp/local_provider.go`. Implement `LocalFSProvider` that maps to the local file system with path bounding.
    *   Create `srcs/server/tools/hybridfsmcp/cloud_provider.go`. Implement `CloudFSProvider` with tenant-scoped access via `auth.Claims` from context. Wait, how do we get `auth.Claims`? We will need to check the codebase to see how auth.Claims is used. Let's look up auth.Claims usage.
    *   Let's check if there's an `auth` package.
3.  **Implement `Server` and Factory**
    *   Create `srcs/server/tools/hybridfsmcp/server.go`. Create an MCP server (`Server`) that takes a `FileSystemProvider`.
    *   Create a factory method `NewFileSystemProvider(ctx context.Context, basePath string)` that returns either `LocalFSProvider` or `CloudFSProvider` depending on the `OHC_STANDALONE` or `OHC_MULTITENANT` environment variables.
4.  **Write Tests**
    *   Create `srcs/server/tools/hybridfsmcp/provider_test.go` and `server_test.go`.
    *   Write tests to ensure >90% test coverage.
5.  **Gazelle**
    *   Run `bazelisk run //:gazelle` to update `BUILD.bazel` files.
6.  **Mark Mission Complete and submit**
    *   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
    *   Update `agent: Jules` and `status: DONE` in `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`.
    *   Submit the PR.
