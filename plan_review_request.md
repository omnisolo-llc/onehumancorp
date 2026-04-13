1. **Frontend Component (`srcs/app/lib/screens/kairos_dashboard.dart`):**
   - Use `run_in_bash_session` to write exactly the provided code string for `srcs/app/lib/screens/kairos_dashboard.dart`.
   - Verify file was created correctly using `cat srcs/app/lib/screens/kairos_dashboard.dart`.

2. **Backend API (`srcs/server/dashboard/kairos_stream.go`):**
   - Use `run_in_bash_session` to write exactly the provided Go code string that implements `handleKairosStream` SSE handler in `srcs/server/dashboard/kairos_stream.go`.
   - Verify file was created using `cat srcs/server/dashboard/kairos_stream.go`.

3. **Backend Integration (`srcs/server/dashboard/server.go`):**
   - Patch `srcs/server/dashboard/server.go` using `sed` to attach `handleKairosStream` to the mux at `/api/kairos/stream`.
   - Verify the patch applied using `grep -n -B 2 -A 2 "kairos/stream" srcs/server/dashboard/server.go`.

4. **Frontend Integration (`srcs/app/lib/router.dart`):**
   - Patch `srcs/app/lib/router.dart` using a bash script to add `KairosDashboardScreen` to `GoRoute` array and sidebar `_NavItem`.
   - Verify using `cat srcs/app/lib/router.dart`.

5. **Testing & Verification:**
   - Write test files `srcs/server/dashboard/kairos_stream_test.go` and `srcs/app/lib/screens/kairos_dashboard_test.dart` using exact bash strings.
   - Verify file was created correctly using `cat srcs/server/dashboard/kairos_stream_test.go` and `cat srcs/app/lib/screens/kairos_dashboard_test.dart`.
   - Update `BUILD.bazel` to include tests using `sed`.
   - Execute tests using `bazelisk test //srcs/server/dashboard:dashboard_test` and `cd srcs/app && flutter test lib/screens/kairos_dashboard_test.dart`.

6. **Mark Mission Done:**
   - Update the mission status by creating a new file in `.agent-task/missions/` with the current timestamp that overrides the previous status.
   - Verify file was created using `cat .agent-task/missions/*kairos_teammate_mesh_dashboard_done.yml`.

7. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

8. **Submit:**
   - Submit pull request to branch `jules-6958013220272975542-93cd4417` via submit tool.
