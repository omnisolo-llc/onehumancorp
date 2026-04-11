1. **Explore File System Interface**: Investigate `srcs/server/tools/hybridfsmcp` or `srcs/server/agents/mcp` structure to find the best place. Create `srcs/server/tools/hybridfsmcp/fs_provider.go` to define the interface `mcp.FileSystemProvider` and its implementation for Local and Cloud.
2. **Implement `LocalFSProvider` and `CloudFSProvider`**: In `fs_provider.go`, add `ReadFile`, `WriteFile`, `ListDir` to local with bounded scopes and cloud mapped functionally.
3. **Implement the MCP Tool Exposer**: Create `srcs/server/tools/hybridfsmcp/mcp.go` implementing tools using the providers.
4. **Testing**: Write comprehensive unit tests in `srcs/server/tools/hybridfsmcp/mcp_test.go`.
