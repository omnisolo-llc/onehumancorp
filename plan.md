1. **Fix `RowsAffected()` calls on custom provider results**
   - The memory states: "When executing SQL updates in Go using `tx.Exec` or `db.Exec` via the custom `db.Provider`, the method returns an `(int64, error)` where the first value is the rows affected. Do not attempt to call `.RowsAffected()` on it as you would with a standard `sql.Result`."
   - I need to fix `srcs/server/orchestration/tasks.go`, `task_orchestrator.go`, `task_queue.go`, and `ultraplan.go` to use the returned `int64` directly instead of calling `RowsAffected()`.
   - In `tasks.go:428`, `task_orchestrator.go:219`, `task_queue.go:262`, `ultraplan.go:153`.

2. **Check for missing memory leak fix in map deletion**
   - Let's fix `srcs/server/orchestration/service.go:1176`. Wait, `subs` is a map of slices (`subs map[string][]chan struct{}`). Setting `subs[i] = nil` is actually setting the slice element to `nil` before doing `append(subs[:i], subs[i+1:]...)` which is a standard pattern in Go to avoid memory leaks of slice elements. The memory hint might be referring to `delete(map, key)`. Let's verify if `map` keys are being set to `nil` instead of being deleted. Let's look for `[...] = nil`.
   - Ah! Wait, I checked `grep -rnl "= nil" srcs/server/orchestration | xargs grep -n "\[.*\] = nil"` and didn't find any. The issue I need to address is just the `RowsAffected()` and potentially another proactive task since `1775146962_proactive_improvement.yml` is already DONE but `my_proactive_mission.yml` is IN_PROGRESS. Wait! I just created `my_proactive_mission.yml` in an earlier step!

Wait, let's create a real proactive improvement mission.

Memory says:
"If no pending missions exist in `.agent-task/missions/`, proactively identify and implement improvements within your specialty area, create a new mission file documenting the proactive work, and generate a PR with passing tests."

Okay, let's use the `RowsAffected()` issue as the proactive improvement!
