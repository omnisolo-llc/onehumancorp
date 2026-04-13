1. **Mark Mission as IN_PROGRESS:** Mark `.agent-task/missions/2026-04-12T22-25-19Z_kairos_teammate_mesh_dashboard.md` as `status: IN_PROGRESS` and `agent: Echo`.
   - Verify by running `cat .agent-task/missions/2026-04-12T22-25-19Z_kairos_teammate_mesh_dashboard.md | head -n 5`.

2. **Implement KAIROS Analytics UI (`srcs/app/lib/screens/kairos_dashboard.dart`)**:
   - Create a Flutter UI matching the mission spec with Glassmorphism aesthetic tokens.
   - Run `cat << 'EOF' > srcs/app/lib/screens/kairos_dashboard.dart ... EOF`.
   - Verify by running `ls -l srcs/app/lib/screens/kairos_dashboard.dart`.

3. **Implement UI Widget Tests (`srcs/app/test/screens/kairos_dashboard_test.dart`)**:
   - Add widget test.
   - Run `cat << 'EOF' > srcs/app/test/screens/kairos_dashboard_test.dart ... EOF`.
   - Verify by running `ls -l srcs/app/test/screens/kairos_dashboard_test.dart`.

4. **Update App Router (`srcs/app/lib/router.dart`)**:
   - Ensure the new `/kairos_dashboard` route is accessible via navigation.
   - Verify by running `git diff srcs/app/lib/router.dart`.

5. **Implement Backend WebSocket Stream (`srcs/server/api/kairos_stream.go`)**:
   - Add `HandleKairosStream` to expose teammate mesh messages via SSE.
   - Run `cat << 'EOF' > srcs/server/api/kairos_stream.go ... EOF`.
   - Verify by running `ls -l srcs/server/api/kairos_stream.go`.

6. **Implement Backend Tests (`srcs/server/api/kairos_stream_test.go`)**:
   - Add backend tests for `HandleKairosStream`.
   - Run `cat << 'EOF' > srcs/server/api/kairos_stream_test.go ... EOF`.
   - Verify by running `ls -l srcs/server/api/kairos_stream_test.go`.

7. **Update BUILD files**:
   - Ensure Bazel includes the new files in `srcs/server/api/BUILD.bazel`.
   - Verify by running `git diff srcs/server/api/BUILD.bazel`.

8. **Mark Mission as DONE:** Update the mission file frontmatter to `status: DONE`.
   - Verify by running `cat .agent-task/missions/2026-04-12T22-25-19Z_kairos_teammate_mesh_dashboard.md | head -n 5`.

9. **Run test verification**: Run `export PATH=$PATH:/home/jules/go/bin && bazelisk test //srcs/server/api/... && cd srcs/app && flutter test test/screens/kairos_dashboard_test.dart` to verify tests pass.

10. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

11. **Submit Change**: Submit a PR titled `🗺️ Guide: [Implement KAIROS Swarm Analytics Dashboard UI]`.
