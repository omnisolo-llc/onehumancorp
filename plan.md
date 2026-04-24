We also need to fix `src/server/orchestration/state_machine.go` to properly lock the parent task in `ProcessEvent` when checking subtasks.
Wait, let's look at `ProcessEvent` again:
```go
		if parentTaskID != nil && *parentTaskID != "" {
			var parentState string
			pQuery := `SELECT status FROM ohc_tasks WHERE id = $1`
			if !sm.dbProvider.IsSQLite() {
				pQuery += ` FOR UPDATE`
			}
			err = tx.QueryRow(ctx, pQuery, *parentTaskID).Scan(&parentState)
```

If we're in SQLite, `!sm.dbProvider.IsSQLite()` is false, so it doesn't do `FOR UPDATE`. But since SQLite doesn't do row-level locking, it needs application-level locking.
However, SQLite's locking in Go with `modernc.org/sqlite` during `tx.Begin()` means only one writer can be active at a time! Is there a race condition?
Wait, if it's in "Standalone mode" (SQLite), the instructions said:
"In SQLite Standalone mode, functions utilizing `FOR UPDATE SKIP LOCKED` (which is Postgres-specific) must implement application-level fallback locking using a package-level `sync.Mutex` (`TryLock()` followed by `Lock()`) that wraps the *entire transaction lifecycle*"

Wait! The prompt doesn't say "for `FOR UPDATE`", it explicitly says `FOR UPDATE SKIP LOCKED`.
This state machine doesn't use `FOR UPDATE SKIP LOCKED`, it only uses `FOR UPDATE`.
BUT the instruction also says: "Concurrency Control: Preventing race conditions when multiple workers update sub-task statuses simultaneously. ... write a concurrent integration test simulating multiple sub-tasks completing simultaneously to verify that the DB locking prevents race conditions and the parent task correctly transitions to DONE."

The existing `TestStateMachine_Concurrent` test passes.
Wait, let's look at `TestStateMachine_Concurrent`. It runs:
```go
	for _, sub := range subtasks {
		wg.Add(1)
		go func(s string) {
			defer wg.Done()
			sm.ProcessEvent(ctx, s, EventSubTaskCompleted)
		}(sub)
	}
	wg.Wait()
```
Since it creates the `sm` with `nil` `redisClient`, it uses `SQLiteMutexProvider`!
Wait! The `SQLiteMutexProvider` uses a DB table `distributed_locks` to lock:
```go
func (p *SQLiteMutexProvider) NewMutex(key string) Mutex {
	return &SQLiteMutex{
		provider: p,
		key:      key,
	}
}
```
Does it lock the parent task? No! In `ProcessEvent` it only locks `"sm:" + taskID`. It doesn't lock the parent!
So why doesn't it have a race condition?
Because SQLite transactions (`tx.Exec` etc) are serialized by the driver/DB itself for writes. Wait, if they are serialized, it works. But wait, what if we lock the parent task explicitly?
Let's add parent task locking just to be perfectly safe and rigorous according to KAIROS Distributed State Machine design.
"Concurrency Control: Preventing race conditions when multiple workers update sub-task statuses simultaneously."

Let's modify `ProcessEvent` to lock the parent task as well if it's not empty, or wait, if we lock it inside the transaction, it might deadlock in Postgres if another transaction locked the parent and is trying to lock the subtask.
Actually, in Postgres, `pQuery += " FOR UPDATE"` already locks the parent task properly. So in Postgres, it's safe.
In SQLite, we don't need explicit row-level locking because the entire DB is locked for writing when `tx, _ = sm.dbProvider.Begin(ctx)` starts!
Wait! Is the DB fully locked in `modernc.org/sqlite`?
Yes! "A write transaction in SQLite locks the entire database." So multiple goroutines doing `tx.Begin()` will be serialized by SQLite's write lock (or get `database is locked` error, but the driver handles retries or blocks).
So we don't actually need to add any other locking for SQLite! The existing `FOR UPDATE` handles Postgres, and SQLite's global write lock handles SQLite!

Wait! I should re-read the assignment.
"Instructions:
1.  **Schema Update:** Create a DB migration to add `parent_task_id` and `workflow_state` to the `ohc_tasks` table.
2.  **State Machine Logic:** Create `srcs/server/orchestration/state_machine.go`. Define the states, events, and the `ProcessEvent` logic using DB transactions to ensure atomic state transitions.
3.  **KAIROS Integration:** Integrate the state machine into the main KAIROS orchestrator loop. When KAIROS receives a new high-level request, it should create a `PENDING` task, transition it to `DECOMPOSING`, generate sub-tasks, and transition to `EXECUTING`.
4.  **Testing:** Write unit tests for the state machine transitions. Crucially, write a concurrent integration test simulating multiple sub-tasks completing simultaneously to verify that the DB locking prevents race conditions and the parent task correctly transitions to `DONE`."

Wait, EVERYTHING in instructions 1, 2, and 4 WAS ALREADY COMPLETED in the repository! The only missing part was step 3 (which I just completed by patching `ReceiveHighLevelRequest`), where the instructions explicitly said:
"When KAIROS receives a new high-level request, it should create a `PENDING` task, transition it to `DECOMPOSING`, generate sub-tasks, and transition to `EXECUTING`."

Let's verify `ReceiveHighLevelRequest` logic again:
```go
	// Create PENDING task
	_, err = tx.Exec(ctx, "INSERT INTO ohc_tasks (id, organization_id, title, status) VALUES ($1, $2, $3, $4)", taskID, orgID, title, "PENDING")
    ...
	tx.Commit(ctx) // Commit insertion of PENDING task

	sm := NewTaskStateMachine(to.db, to.redisClient)

	// Transition to DECOMPOSING
	err = sm.TransitionState(ctx, taskID, TaskStateDecomposing)
    ...
	// Generate sub-tasks
	tx2, err := to.db.Begin(ctx)
    ...
		_, err = tx2.Exec(ctx, "INSERT INTO ohc_tasks (id, organization_id, title, parent_task_id, status) VALUES ($1, $2, $3, $4, $5)", subID, orgID, fmt.Sprintf("%s - Subtask %d", title, i+1), taskID, TaskStatePending)
    ...
	tx2.Commit(ctx)

	// Transition to EXECUTING
	err = sm.ProcessEvent(ctx, taskID, EventDecompositionComplete)
```
This EXACTLY matches step 3.
Wait! One detail: `tx2` and `tx`. Should I just use one transaction?
Wait, if I use one transaction, I can't call `TransitionState` or `ProcessEvent` because they create their own transactions, and calling them before `tx.Commit()` means they won't see the row!
So my approach of doing `tx.Commit()` first, then calling `TransitionState`, then another transaction for subtasks, then `ProcessEvent` is CORRECT because the state machine functions open their own transactions!

Let's run `pre_commit_instructions` and finalize the work.
