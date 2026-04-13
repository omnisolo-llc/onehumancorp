package orchestration

import (
	"context"
	"database/sql"
	"sync"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

func TestStateMachine_Concurrent(t *testing.T) {
	conn, err := sql.Open("sqlite", ":memory:")
	assert.NoError(t, err)
	defer conn.Close()

	provider := db.NewSqliteProvider(conn)

	ctx := context.Background()
	tx, _ := provider.Begin(ctx)
	tx.Exec(ctx, `CREATE TABLE shared_tasks (id TEXT PRIMARY KEY, organization_id TEXT, title TEXT, parent_task_id TEXT, status TEXT, workflow_state TEXT, updated_at TIMESTAMP)`)
	tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('parent', 'org1', 'title1', 'EXECUTING')`)
	tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, parent_task_id, status) VALUES ('sub1', 'org1', 'title1', 'parent', 'EXECUTING')`)
	tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, parent_task_id, status) VALUES ('sub2', 'org1', 'title1', 'parent', 'EXECUTING')`)
	tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, parent_task_id, status) VALUES ('sub3', 'org1', 'title1', 'parent', 'EXECUTING')`)
	tx.Commit(ctx)

	sm := NewTaskStateMachine(provider, nil)

	var wg sync.WaitGroup
	subtasks := []string{"sub1", "sub2", "sub3"}

	for _, sub := range subtasks {
		wg.Add(1)
		go func(s string) {
			defer wg.Done()
			sm.ProcessEvent(ctx, s, EventSubTaskCompleted)
		}(sub)
	}

	wg.Wait()

	tx, _ = provider.Begin(ctx)
	var parentStatus string
	tx.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'parent'").Scan(&parentStatus)
	tx.Rollback(ctx)

	assert.Equal(t, TaskStateDone, parentStatus)
}
