1. **Update Mission File:**
   - Update `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to have `status: IN_PROGRESS` and `agent: Scribe`. I will run `sed -i 's/status: PENDING/status: IN_PROGRESS/g' .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` and `sed -i 's/agent: Researcher/agent: Scribe/g' .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`.
   - Run `head -n 5 .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to verify.

2. **Implement FileSystemProvider Interface (`srcs/server/tools/hybridfsmcp/provider.go`):**
   - Create `provider.go` using `cat << 'EOF' > srcs/server/tools/hybridfsmcp/provider.go`.
   - The file will contain the `FileSystemProvider` interface with `ReadFile`, `WriteFile`, `ListDir`.
   - `LocalFSProvider` logic using `filepath.Clean`, `filepath.IsAbs`, and `strings.HasPrefix(cleanPath, baseDir + string(filepath.Separator))` to prevent directory traversal. Bound to `OHC_WORKSPACE_DIR` falling back to `/tmp/ohc_workspace`.
   - `CloudFSProvider` logic using `OHC_TENANT_PV_DIR` falling back to `/tmp/ohc_pv`. It will take tenant info from auth claims.
   - Run `cat srcs/server/tools/hybridfsmcp/provider.go | head -n 15` to verify.

3. **Implement HybridFSMCP Tool (`srcs/server/tools/hybridfsmcp/mcp.go`):**
   - Create `mcp.go` using `cat << 'EOF' > srcs/server/tools/hybridfsmcp/mcp.go`.
   - Create `HybridFSInspectorMCP` adhering to MCP.
   - Extract `auth.ClaimsFromContext(ctx)` in `CallTool`.
   - `NewHybridFSInspectorMCP` factory that uses `os.Getenv("OHC_MULTITENANT")` and `os.Getenv("OHC_STANDALONE")`.
   - Run `cat srcs/server/tools/hybridfsmcp/mcp.go | head -n 15` to verify.

4. **Add Unit Tests (`srcs/server/tools/hybridfsmcp/provider_test.go` and `srcs/server/tools/hybridfsmcp/mcp_test.go`):**
   - Create tests using `cat << 'EOF' > ...` to achieve high coverage. Test directory traversal protections and claims injection using `auth.ClaimsContextKeyForTest`.
   - Run `cat srcs/server/tools/hybridfsmcp/provider_test.go | head -n 15` and `cat srcs/server/tools/hybridfsmcp/mcp_test.go | head -n 15` to verify.

5. **Update Build Configuration (`srcs/server/tools/hybridfsmcp/BUILD.bazel`):**
   - Create the build config using `cat << 'EOF' > srcs/server/tools/hybridfsmcp/BUILD.bazel` to define `go_library` and `go_test` targets.
   - Run `cat srcs/server/tools/hybridfsmcp/BUILD.bazel | head -n 15` to verify.

6. **Format and Verify:**
   - Run `gofmt -w srcs/server/tools/hybridfsmcp/provider.go srcs/server/tools/hybridfsmcp/mcp.go srcs/server/tools/hybridfsmcp/provider_test.go srcs/server/tools/hybridfsmcp/mcp_test.go`.
   - Run `bazelisk test //srcs/server/tools/hybridfsmcp/... --test_output=errors --jobs=4 --local_test_jobs=1`.

7. **Complete pre-commit steps:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

8. **Finalize:**
   - Update mission file status to `DONE` using `sed` and visually verify with `head -n 5`.
   - Submit the PR with title `✍️ Scribe: [new documentation feature] Integrate Hybrid File System MCP Server`.
