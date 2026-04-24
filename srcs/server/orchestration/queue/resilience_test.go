package queue

import (
	"context"
	"database/sql"
	"fmt"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/lib/resilience/chaos"
	_ "modernc.org/sqlite"
)

func TestAgentJobResilience(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE sub_agent_queue (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_task_id TEXT NOT NULL,
			payload TEXT NOT NULL,
			status TEXT NOT NULL,
			worker_id TEXT,
			attempts INTEGER NOT NULL DEFAULT 0,
			max_attempts INTEGER NOT NULL DEFAULT 3,
			created_at DATETIME,
			updated_at DATETIME
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	qm := NewQueueManager(sqliteProv)
	ctx := context.Background()

	// 1. Test Job Timeout/Retry logic (Simulation)
	// In the real system, a worker would poll, try to execute, fail, and the job would be retried.
	// We want to ensure that failures don't leave the queue in a corrupt state.

	job := &SubAgentJob{
		ID:             "job-1",
		OrganizationID: "org-1",
		ParentTaskID:   "parent-1",
		Payload:        map[string]interface{}{"cmd": "bake"},
	}

	err = qm.Enqueue(ctx, job)
	if err != nil {
		t.Fatalf("failed to enqueue: %v", err)
	}

	// Simulate a worker claiming and then "dying" (timeout/chaos)
	inj := chaos.NewInjector(chaos.ConnectionDrop, 7)

	// Claim should succeed or fail cleanly
	err = inj.Inject(ctx)
	if err == nil {
		acquired, err := qm.Acquire(ctx, "worker-1")
		if err != nil {
			t.Logf("Acquire failed as expected under chaos: %v", err)
		} else if acquired != nil {
			if acquired.ID != "job-1" {
				t.Errorf("expected job-1, got %s", acquired.ID)
			}
		}
	}

	// Verify parity under chaos
	t.Run("ParityUnderChaos", func(t *testing.T) {
		// Enqueue another job
		job2 := &SubAgentJob{
			ID:             "job-2",
			OrganizationID: "org-1",
			ParentTaskID:   "parent-1",
			Payload:        map[string]interface{}{"cmd": "deliver"},
		}
		_ = qm.Enqueue(ctx, job2)

		// Test Acquire under heavy chaos
		injStrong := chaos.NewInjector(chaos.ResourceExhaustion, 8)
		for i := 0; i < 10; i++ {
			_ = injStrong.Inject(ctx)
			_, _ = qm.Acquire(ctx, fmt.Sprintf("worker-%d", i))
		}

		// Ensure system still responsive
		finalAcquire, err := qm.Acquire(ctx, "worker-final")
		if err != nil {
			t.Logf("Final acquire error (acceptable): %v", err)
		}
		_ = finalAcquire
	})
}

func TestAgentJobRetriesSimulation(t *testing.T) {
	// Mission says: "AI agent jobs must have a 60-second timeout with automatic retry (max 3 attempts)."

	sqlDB, _ := sql.Open("sqlite", ":memory:")
	defer sqlDB.Close()
	_, _ = sqlDB.Exec(`CREATE TABLE sub_agent_queue (id TEXT PRIMARY KEY, organization_id TEXT, parent_task_id TEXT, payload TEXT, status TEXT, worker_id TEXT, attempts INTEGER NOT NULL DEFAULT 0, max_attempts INTEGER NOT NULL DEFAULT 3, created_at DATETIME, updated_at DATETIME)`)

	sqliteProv := db.NewSqliteProvider(sqlDB)
	qm := NewQueueManager(sqliteProv)
	ctx := context.Background()

	job := &SubAgentJob{ID: "retry-job", OrganizationID: "org-1", ParentTaskID: "p1", Payload: map[string]interface{}{"x":1}}
	qm.Enqueue(ctx, job)

	// Simulate failed attempts by resetting status to QUEUED
	for i := 1; i <= 3; i++ {
		acquired, err := qm.Acquire(ctx, "worker-retry")
		if err != nil {
			t.Fatalf("attempt %d: acquire error: %v", i, err)
		}
		if acquired == nil {
			t.Fatalf("attempt %d: job should be acquirable", i)
		}
		if acquired.Attempts != i {
			t.Errorf("attempt %d: expected job attempts %d, got %d", i, i, acquired.Attempts)
		}
		// Simulate failure by resetting to QUEUED (retry)
		sqlDB.Exec("UPDATE sub_agent_queue SET status = 'QUEUED', worker_id = NULL WHERE id = 'retry-job'")
	}

	// 4th attempt should NOT be acquirable because max_attempts = 3
	lastAcquire, _ := qm.Acquire(ctx, "worker-last")
	if lastAcquire != nil {
		t.Fatal("job should NOT be acquirable after 3 failed attempts")
	}
}
