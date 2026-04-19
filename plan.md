1. **Fix `ClaimTask` in `srcs/server/orchestration/tasks.go`**:
   - Ensure the query checks DAG dependencies using the `task_dependencies` join table properly: `AND (SELECT COUNT(*) FROM task_dependencies td INNER JOIN shared_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0`. Wait, the original `ClaimTask` code in `tasks.go` (that I overwrote/modified) ALREADY had all of this logic according to `get_claimtask.py` from earlier!
   - Ensure `FOR UPDATE SKIP LOCKED` is present for Postgres, and two-step select-then-update is used for SQLite, checking `organization_id`.
   - Ah! When I wrote `patch_claimtask.py`, I replaced only the transition. Let me restore `ClaimTask` to its exact expected state from the prompt. Actually, wait! My previous `get_claimtask.py` showed that `ClaimTask` ALREADY had `FOR UPDATE SKIP LOCKED` and the dependency check. But the code review says: "The patch fails to implement the core requirements. It does not implement DAG sequence blocking... it does not introduce FOR UPDATE SKIP LOCKED... minor update it makes to ClaimTask fails to enforce the requested organization_id multi-tenant isolation." Wait, did I overwrite the file entirely at some point? Or did `tasks.go` never have that?
   Let's check `tasks.go`'s `ClaimTask` implementation directly to see what it *actually* is currently.

2. **Fix Schema Replaces**:
   - Revert any files containing `assigned_assigned_agent_id` back to `assigned_agent_id`.
   - The prompt says: "When creating IDs in SQLite, manually generate them in Go." I need to ensure if `IsSQLite()` is true, IDs are generated.

3. **Replan**:
   - Inspect `tasks.go` properly.
   - Run tests again.
