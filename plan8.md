Ah, `TestQueueManagerLoop` is failing because of `no such table: sub_agent_queue`. But earlier it created the schema.
Wait, memory constraints explicitly say:
"If an in-memory SQLite test database fails with schema conflicts (e.g., missing columns) because a table was created or altered differently by older migrations in the test suite, prepend `DROP TABLE IF EXISTS <table_name>;` before `CREATE TABLE IF NOT EXISTS` in the new migration to ensure the correct final schema state."

Let me add `DROP TABLE IF EXISTS sub_agent_queue;` before the schema creation in `src/server/orchestration/queue/queue_manager_loop_test.go` and `src/server/orchestration/queue/queue_test.go`. Wait, `queue_test.go` may not need it if it already works, but doing it in `queue_manager_loop_test.go` fixes the issue. Wait, if it's failing with `no such table`, then perhaps `context.TODO()` at line 84 runs *after* the `db.NewTestProvider(t)` db has been closed because the `cancel()` cancels the test context... wait, `db.NewTestProvider` cleans up when `t` finishes. Maybe the `QueryRow` is using `context.TODO()` but the test is ending? No, `t.Fatalf` would stop it. Wait, `db.NewTestProvider(t)` registers a cleanup function. If the context is canceled, does it drop tables? No. But wait!

```go
	var status1, status2 string
	// Retry loop for SQLITE_BUSY
	for i := 0; i < 5; i++ {
		err = provider.QueryRow(context.TODO(), "SELECT status FROM sub_agent_queue WHERE id = 'job-1'").Scan(&status1)
```
Why is the table missing? "no such table: sub_agent_queue".
Maybe I should check `src/server/orchestration/queue/queue_test.go` if it has the same drop table.

Let me just apply the memory constraint:
"If an in-memory SQLite test database fails with schema conflicts (e.g., missing columns) because a table was created or altered differently by older migrations in the test suite, prepend DROP TABLE IF EXISTS <table_name>; before CREATE TABLE IF NOT EXISTS in the new migration to ensure the correct final schema state."
