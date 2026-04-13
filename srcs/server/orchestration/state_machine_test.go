package orchestration

import (
	"context"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTaskStateMachine_ProcessEvent(t *testing.T) {
	// Use connection URI that supports concurrent accesses well for SQLite testing
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared&_txlock=immediate&_busy_timeout=5000")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	ctx := context.Background()

	_, err = pool.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('parent-1', 'org1', 'title', 'PENDING')")
	if err != nil {
		t.Fatalf("failed to insert parent: %v", err)
	}

	for i := 0; i < 5; i++ {
		_, err = pool.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, title, parent_task_id, status) VALUES ($1, 'org1', 'title', 'parent-1', 'PENDING')", "child-"+string(rune(i+'0')))
		if err != nil {
			t.Fatalf("failed to insert child: %v", err)
		}
	}

	sm := NewTaskStateMachine(pool.Provider)

	var wg sync.WaitGroup
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(id string) {
			defer wg.Done()

			// Simple retry logic for SQLite BUSY in concurrent test for updates
			maxRetries := 20
			for r := 0; r < maxRetries; r++ {
				_, err := pool.Exec(ctx, "UPDATE shared_tasks SET status = 'COMPLETED' WHERE id = $1", id)
				if err != nil {
					time.Sleep(10 * time.Millisecond)
					continue
				}

				err = sm.ProcessEvent(ctx, id, EventSubTaskCompleted)
				if err != nil {
					time.Sleep(10 * time.Millisecond)
					continue
				}
				break
			}
		}("child-" + string(rune(i+'0')))
	}
	wg.Wait()

	var parentStatus string
	err = pool.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'parent-1'").Scan(&parentStatus)
	if err != nil {
		t.Fatalf("failed to query parent status: %v", err)
	}

	if parentStatus != "VERIFYING" {
		t.Errorf("expected parent status VERIFYING, got %s", parentStatus)
	}
}
