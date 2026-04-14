import re

with open('srcs/server/orchestration/tasks_store.go', 'r') as f:
    content = f.read()

# Fix TransitionTask to check RowsAffected
new_func = """func (ds *decompositionStore) TransitionTask(ctx context.Context, taskID, agentID, fromState, toState, reason string) error {
    tx, err := ds.dbProvider.Begin(ctx)
    if err != nil { return err }
    defer tx.Rollback(ctx)
    res, err := tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND status = $3", toState, taskID, fromState)
    if err != nil { return err }
    if res == 0 { return fmt.Errorf("task %s state transition from %s failed or task not found", taskID, fromState) }
    return tx.Commit(ctx)
}"""

# Replace the old function
content = re.sub(r'func \(ds \*decompositionStore\) TransitionTask\(ctx context\.Context, taskID, agentID, fromState, toState, reason string\) error \{.*?\n\}', new_func, content, flags=re.DOTALL)

with open('srcs/server/orchestration/tasks_store.go', 'w') as f:
    f.write(content)
