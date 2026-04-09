Plan: Integrate Hybrid File System MCP Server
1. Create `srcs/server/agents/mcp/fs.go` (or a `fs` subpackage) defining `mcp.FileSystemProvider` interface.
2. Implement `LocalFSProvider` that works with the local file system. Include boundary checking (chroot-like) if possible.
3. Implement `CloudFSProvider` that includes logic to scope access based on tenant ID.
4. Add a factory function to instantiate the correct provider based on `OHC_MULTITENANT` and `OHC_STANDALONE` environment variables.
5. Create MCP server handlers for `read_file`, `write_file`, `list_directory`.
6. Write unit tests in `fs_test.go` mocking operations and verifying both local and cloud behaviors to achieve >90% coverage.
7. Include pre commit steps to ensure everything passes tests.
