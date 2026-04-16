import re

with open('srcs/server/orchestration/tasks_db.go', 'r') as f:
    content = f.read()

claim_pending_task_method = """
func (to *SharedTaskOrchestrator) ClaimPendingTask(ctx context.Context) (*Task, error) {
    tx, err := to.dbProvider.Begin(ctx)
    if err != nil {
        return nil, fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    query := `
        SELECT id FROM shared_tasks_v2
        WHERE status = 'PENDING'
        LIMIT 1
        FOR UPDATE SKIP LOCKED
    `
    var id string
    err = tx.QueryRow(ctx, query).Scan(&id)
    if err != nil {
        return nil, err
    }

    _, err = tx.Exec(ctx, "UPDATE shared_tasks_v2 SET status = 'IN_PROGRESS' WHERE id = $1", id)
    if err != nil {
        return nil, err
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, err
    }

    return &Task{TaskID: id, Status: "IN_PROGRESS"}, nil
}
"""

if "ClaimPendingTask" not in content:
    content += "\n" + claim_pending_task_method

with open('srcs/server/orchestration/tasks_db.go', 'w') as f:
    f.write(content)
