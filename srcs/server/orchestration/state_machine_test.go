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

func TestStateMachine_TransitionAndDependencies(t *testing.T) {
	conn, err := sql.Open("sqlite", ":memory:")
	assert.NoError(t, err)
	defer conn.Close()

	provider := db.NewSqliteProvider(conn)
	ctx := context.Background()

	// Initialize tables needed for tests
	tx, _ := provider.Begin(ctx)
	tx.Exec(ctx, `CREATE TABLE shared_tasks (id TEXT PRIMARY KEY, organization_id TEXT, title TEXT, status TEXT, updated_at TIMESTAMP)`)
	tx.Exec(ctx, `CREATE TABLE swarm_task_dependencies (task_id TEXT, depends_on_task_id TEXT, PRIMARY KEY(task_id, depends_on_task_id))`)

	// Insert tasks and dependency
	tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('taskA', 'org1', 'Task A', 'PENDING')`)
	tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('taskB', 'org1', 'Task B', 'BLOCKED')`)
	tx.Exec(ctx, `INSERT INTO swarm_task_dependencies (task_id, depends_on_task_id) VALUES ('taskB', 'taskA')`)
	tx.Commit(ctx)

	sm := NewTaskStateMachine(provider, nil)

	// Check dependencies for taskB
	ready, err := sm.CheckDependencies(ctx, "taskB")
	assert.NoError(t, err)
	assert.False(t, ready) // taskA is PENDING, so taskB is not ready

	// Process task.completed for taskA
	err = sm.ProcessEvent(ctx, "taskA", EventTaskCompleted)
	assert.NoError(t, err)

	// Verify taskA is DONE
	tx, _ = provider.Begin(ctx)
	var taskAStatus string
	tx.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'taskA'").Scan(&taskAStatus)
	assert.Equal(t, TaskStateDone, taskAStatus)
	tx.Rollback(ctx)

	// Check dependencies for taskB again
	ready, err = sm.CheckDependencies(ctx, "taskB")
	assert.NoError(t, err)
	assert.True(t, ready) // taskA is DONE, so taskB is ready

	// Verify taskB is READY
	tx, _ = provider.Begin(ctx)
	var taskBStatus string
	tx.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'taskB'").Scan(&taskBStatus)
	assert.Equal(t, TaskStateReady, taskBStatus)
	tx.Rollback(ctx)
}
