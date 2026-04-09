1. **Mark Mission In Progress**
   - Run `sed -i 's/^status: PENDING/status: IN_PROGRESS\nagent: Jules/' .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to mark the mission as IN_PROGRESS.
   - Run `cat .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to verify the frontmatter modification.

2. **Create `FileSystemProvider` Interface**
   - Run `run_in_bash_session` to write the interface definition to `srcs/server/tools/hybridfsmcp/provider.go` using `cat << 'EOF'`.
   - Run `ls -l srcs/server/tools/hybridfsmcp/provider.go` to verify creation.

3. **Create `LocalFSProvider`**
   - Run `run_in_bash_session` to write `LocalFSProvider` implementation to `srcs/server/tools/hybridfsmcp/local_provider.go` using `cat << 'EOF'`.
   - Run `ls -l srcs/server/tools/hybridfsmcp/local_provider.go` to verify creation.

4. **Create `CloudFSProvider`**
   - Run `run_in_bash_session` to write `CloudFSProvider` implementation to `srcs/server/tools/hybridfsmcp/cloud_provider.go` using `cat << 'EOF'`.
   - Run `ls -l srcs/server/tools/hybridfsmcp/cloud_provider.go` to verify creation.

5. **Create `HybridFSMCP` Server**
   - Run `run_in_bash_session` to write `HybridFSMCP` implementation to `srcs/server/tools/hybridfsmcp/mcp.go` using `cat << 'EOF'`.
   - Run `ls -l srcs/server/tools/hybridfsmcp/mcp.go` to verify creation.

6. **Create Test File**
   - Run `run_in_bash_session` to write test implementations to `srcs/server/tools/hybridfsmcp/mcp_test.go` using `cat << 'EOF'`.
   - Run `ls -l srcs/server/tools/hybridfsmcp/mcp_test.go` to verify creation.

7. **Generate Bazel Build File**
   - Run `bazelisk run //:gazelle` to generate `BUILD.bazel` for `srcs/server/tools/hybridfsmcp`.
   - Run `git status` and `cat srcs/server/tools/hybridfsmcp/BUILD.bazel` to verify the generated output.

8. **Run Tests**
   - Run `bazelisk test //srcs/server/tools/hybridfsmcp/...` to verify the implementation.

9. **Complete pre commit steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

10. **Mark mission DONE**
    - Run `sed -i 's/^status: IN_PROGRESS/status: DONE/' .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to mark the mission as DONE.
