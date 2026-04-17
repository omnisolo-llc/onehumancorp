with open("srcs/server/orchestration/task_orchestrator.go", 'r') as f:
    content = f.read()

# Fix AcquireReadyTask rollback
content = content.replace(
    '''\t\tif err != nil {
\t\t\ttx.Rollback(ctx)
\t\t\tif err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
\t\t\t\tvar exists bool
\t\t\t\tcheckErr := tx.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM swarm_tasks st WHERE st.status = 'READY' AND (SELECT COUNT(*) FROM swarm_task_dependencies td INNER JOIN swarm_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0)").Scan(&exists)
\t\t\t\tif checkErr == nil && exists {
\t\t\t\t\ttelemetry.RecordPostgresLockContention(ctx, "claim_task")
\t\t\t\t}
\t\t\t}
\t\t\treturn nil, nil // No task available
\t\t}''',
    '''\t\tif err != nil {
\t\t\tif err == sql.ErrNoRows || err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
\t\t\t\tvar exists bool
\t\t\t\tcheckErr := tx.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM swarm_tasks st WHERE st.status = 'READY' AND (SELECT COUNT(*) FROM swarm_task_dependencies td INNER JOIN swarm_tasks d ON td.depends_on_task_id = d.id WHERE td.task_id = st.id AND d.status != 'COMPLETED') = 0)").Scan(&exists)
\t\t\t\tif checkErr == nil && exists {
\t\t\t\t\ttelemetry.RecordPostgresLockContention(ctx, "claim_task")
\t\t\t\t}
\t\t\t}
\t\t\ttx.Rollback(ctx)
\t\t\treturn nil, nil // No task available
\t\t}'''
)

# Fix claimDecompositionTaskSQLite
content = content.replace(
    '''\t\t\tif err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
\t\t\t\tvar exists bool
\t\t\t\tcheckErr := tx.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM shared_tasks_decomposition WHERE status = 'PENDING')").Scan(&exists)
\t\t\t\tif checkErr == nil && exists {
\t\t\t\t\ttelemetry.RecordSQLiteLockContention(ctx, "claim_decomposition_task_sqlite")
\t\t\t\t}
\t\t\t\t\treturn nil, nil
\t\t\t}''',
    '''\t\t\tif err == sql.ErrNoRows || err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
\t\t\t\tvar exists bool
\t\t\t\tcheckErr := tx.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM shared_tasks_decomposition WHERE status = 'PENDING')").Scan(&exists)
\t\t\t\tif checkErr == nil && exists {
\t\t\t\t\ttelemetry.RecordSQLiteLockContention(ctx, "claim_decomposition_task_sqlite")
\t\t\t\t}
\t\t\t\treturn nil, nil
\t\t\t}'''
)

# Fix claimDecompositionTaskPostgres
content = content.replace(
    '''\t\t\tif err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
\t\t\t\tvar exists bool
\t\t\t\tcheckErr := tx.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM shared_tasks_decomposition WHERE status = 'PENDING')").Scan(&exists)
\t\t\t\tif checkErr == nil && exists {
\t\t\t\t\ttelemetry.RecordPostgresLockContention(ctx, "claim_decomposition_task_postgres")
\t\t\t\t}
\t\t\t\t\treturn nil, nil
\t\t\t}''',
    '''\t\t\tif err == sql.ErrNoRows || err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
\t\t\t\tvar exists bool
\t\t\t\tcheckErr := tx.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM shared_tasks_decomposition WHERE status = 'PENDING')").Scan(&exists)
\t\t\t\tif checkErr == nil && exists {
\t\t\t\t\ttelemetry.RecordPostgresLockContention(ctx, "claim_decomposition_task_postgres")
\t\t\t\t}
\t\t\t\treturn nil, nil
\t\t\t}'''
)

# Fix pollSubAgentQueue
content = content.replace(
    '''\t\t\tif err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
\t\t\t\t\tvar exists bool
\t\t\t\t\tcheckErr := tx.QueryRow(to.workerCtx, "SELECT EXISTS(SELECT 1 FROM shared_tasks WHERE status = 'PENDING' AND (priority = 'DELEGATED' OR payload->>'sub_agent_type' IS NOT NULL))").Scan(&exists)
\t\t\t\t\tif checkErr == nil && exists {
\t\t\t\t\t\ttelemetry.RecordPostgresLockContention(to.workerCtx, "poll_sub_agent_queue")
\t\t\t\t\t}
\t\t\t\t}
\t\t\t\treturn // sql.ErrNoRows or locking issue''',
    '''\t\t\tif err == sql.ErrNoRows || err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
\t\t\t\t\tvar exists bool
\t\t\t\t\tcheckErr := tx.QueryRow(to.workerCtx, "SELECT EXISTS(SELECT 1 FROM shared_tasks WHERE status = 'PENDING' AND (priority = 'DELEGATED' OR payload->>'sub_agent_type' IS NOT NULL))").Scan(&exists)
\t\t\t\t\tif checkErr == nil && exists {
\t\t\t\t\t\ttelemetry.RecordPostgresLockContention(to.workerCtx, "poll_sub_agent_queue")
\t\t\t\t\t}
\t\t\t\t}
\t\t\t\treturn // sql.ErrNoRows or locking issue'''
)

with open("srcs/server/orchestration/task_orchestrator.go", 'w') as f:
    f.write(content)
