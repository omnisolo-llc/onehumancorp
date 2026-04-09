1.  **Define Interface `mcp.FileSystemProvider`**
    *   Create `srcs/server/tools/hybridfsmcp/provider.go`.
    *   Define interface `FileSystemProvider` with methods `ReadFile`, `WriteFile`, `ListDir`.
    *   Implement `LocalFSProvider` (Standalone mode).
        *   Takes a `workspaceRoot` parameter to prevent directory traversal.
        *   Uses standard `os` package but ensures operations stay within the `workspaceRoot`.
    *   Implement `CloudFSProvider` (Cloud-Native mode).
        *   Takes a `baseDir` parameter.
        *   Applies tenant-isolation by prepending `auth.Claims.OrganizationID`.
        *   Uses standard `os` package but ensures operations stay within the `baseDir/OrganizationID`.
2.  **Implement `HybridFSMCP` Server**
    *   Create `srcs/server/tools/hybridfsmcp/mcp.go`.
    *   Define `HybridFSMCP` struct containing a `FileSystemProvider`.
    *   Implement `ListTools` returning `read_file`, `write_file`, `list_directory`.
    *   Implement `CallTool` which delegates to the underlying `FileSystemProvider`.
    *   Ensure `CallTool` extracts `auth.Claims` from context and passes them to `CloudFSProvider` operations.
3.  **Implement Factory and Mode Selection**
    *   Create `NewHybridFSMCP` in `mcp.go`.
    *   The factory should check `os.Getenv("OHC_STANDALONE") == "true"` to instantiate `LocalFSProvider`.
    *   Otherwise, it instantiates `CloudFSProvider` (acting as the Kubernetes PV abstraction).
4.  **Write Tests**
    *   Create `srcs/server/tools/hybridfsmcp/mcp_test.go` and `provider_test.go`.
    *   Test both `LocalFSProvider` and `CloudFSProvider` with bounded paths.
    *   Test `HybridFSMCP` tool dispatching.
    *   Ensure >90% code coverage.
5.  **Pre-commit steps**
    *   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
6.  **Submit**
    *   Update the task status to `DONE` and submit changes.
