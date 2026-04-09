1. **Understand Problem & Setup**: Read the pending mission for "Integrate Hybrid File System MCP Server" at `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`.
2. **Claim Mission**: Mark the mission as `IN_PROGRESS` and assign it to myself.
3. **Design & Implementation**: Create `srcs/server/tools/hybridfsmcp` directory. Create `hybridfsmcp.go` with a `FileSystemProvider` interface defining `ReadFile`, `WriteFile`, `ListDir`, and `SearchFiles`. Implement `LocalFSProvider` with safety bounding and `CloudFSProvider` with `auth.Claims` scoping.
4. **Build MCP Server Wrapper**: Create `mcp_server.go` to expose the providers as MCP tools (`read_file`, `write_file`, `list_directory`, `search_files`).
5. **Add Tests & Coverage**: Create `hybridfsmcp_test.go` and `mcp_server_test.go` checking isolation bounds, interface usage, json serialization errors, missing claim context, dir traversal errors, testing standard execution flows. Iterate over these until `go tool cover` hits > 90% (achieved: 100%).
6. **Build Rules**: Create `BUILD.bazel` so it runs natively and successfully over `bazelisk test //srcs/server/tools/hybridfsmcp/...`.
7. **Complete Mission**: Mark the mission status to `DONE`.
8. **Pre-commit**: Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
9. **Finalize**: Submit the change.
