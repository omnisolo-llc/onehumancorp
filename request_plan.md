1. **Mark Mission as In Progress**:
   - Command: `sed -i 's/status: PENDING/status: IN_PROGRESS\nagent: Implementer/' .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`
   - Verification: `cat .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md | head -n 5`

2. **Implement FileSystemProvider Interfaces**:
   - Command: `cat << 'EOF' > srcs/server/tools/hybridfsmcp/fs_provider.go` with the implementations for `FileSystemProvider`, `LocalFSProvider`, and `CloudFSProvider`. It will properly check `strings.Contains(path, "..")` for security bounds.
   - Verification: `cat srcs/server/tools/hybridfsmcp/fs_provider.go` via run_in_bash_session.

3. **Implement FS MCP Wrapper**:
   - Command: `cat << 'EOF' > srcs/server/tools/hybridfsmcp/mcp.go` with an implementation mimicking `BlobInspectorMCP` but exposing `read_file`, `write_file`, and `list_directory`.
   - Verification: `cat srcs/server/tools/hybridfsmcp/mcp.go` via run_in_bash_session.

4. **Add Tests**:
   - Command: `cat << 'EOF' > srcs/server/tools/hybridfsmcp/fs_provider_test.go` and `cat << 'EOF' > srcs/server/tools/hybridfsmcp/mcp_test.go`.
   - Verification: `ls srcs/server/tools/hybridfsmcp/` via run_in_bash_session.

5. **Create BUILD.bazel file**:
   - Command: `cat << 'EOF' > srcs/server/tools/hybridfsmcp/BUILD.bazel` to define the `go_library` and `go_test` targets.
   - Verification: `cat srcs/server/tools/hybridfsmcp/BUILD.bazel` via run_in_bash_session.

6. **Local Test Execution**:
   - Command: `bazelisk test //srcs/server/tools/hybridfsmcp/...`
   - Verification: Confirm tests pass in the console output.

7. **Pre-Commit Check**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

8. **Mark Mission Done & Global Tests**:
   - Command: `sed -i 's/status: IN_PROGRESS/status: DONE/' .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`
   - Command: `bazelisk test //...`
   - Verification: Ensure no system-wide regressions.
