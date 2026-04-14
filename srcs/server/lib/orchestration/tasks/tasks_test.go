package tasks

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func NewTestProvider(t *testing.T) db.Provider {
	t.Helper()
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := sqliteDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		sqliteDB.Close()
	})

	return db.NewSqliteProvider(sqliteDB)
}

func TestTaskStore(t *testing.T) {
	ctx := context.Background()

	// Create an in-memory SQLite provider for testing
	provider := NewTestProvider(t)

	// Apply schema manually since test provider might not run migrations automatically
	schema := `
		CREATE TABLE IF NOT EXISTS shared_tasks_v2 (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload TEXT,
			parent_plan_id TEXT,
			dependencies TEXT NOT NULL DEFAULT '[]',
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`
	_, err := provider.Exec(ctx, schema)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	store := NewTaskStore(provider)

	// Test DecomposeMission
	t.Run("DecomposeMission", func(t *testing.T) {
		mission := Mission{
			OrganizationID: "org-1",
			Title:          "Test Mission",
			Description:    "Test Description",
			Priority:       "P1",
			Payload:        `{"key":"value"}`,
			ParentPlanID:   "plan-1",
			Dependencies:   []string{},
		}

		tasks, err := store.DecomposeMission(ctx, mission)
		if err != nil {
			t.Fatalf("Failed to decompose mission: %v", err)
		}

		if len(tasks) != 1 {
			t.Fatalf("Expected 1 task, got %d", len(tasks))
		}

		if tasks[0].Title != mission.Title {
			t.Errorf("Expected title %s, got %s", mission.Title, tasks[0].Title)
		}

		if tasks[0].Status != "PENDING" {
			t.Errorf("Expected status PENDING, got %s", tasks[0].Status)
		}
	})

	// Test ClaimNextTask
	t.Run("ClaimNextTask", func(t *testing.T) {
		agentID := "agent-1"
		task, err := store.ClaimNextTask(ctx, agentID)
		if err != nil {
			t.Fatalf("Failed to claim task: %v", err)
		}

		if task == nil {
			t.Fatalf("Expected task, got nil")
		}

		if task.Status != "IN_PROGRESS" {
			t.Errorf("Expected status IN_PROGRESS, got %s", task.Status)
		}

		if task.AssignedAgentID != agentID {
			t.Errorf("Expected assigned agent %s, got %s", agentID, task.AssignedAgentID)
		}

		if task.LockedUntil.IsZero() {
			t.Errorf("Expected locked_until to be set")
		}

		// Ensure another claim returns nothing
		task2, err := store.ClaimNextTask(ctx, "agent-2")
		if err != nil {
			t.Fatalf("Failed to claim task: %v", err)
		}

		if task2 != nil {
			t.Fatalf("Expected nil task, got %v", task2)
		}
	})

	// Test Parallel Claims
	t.Run("ParallelClaims", func(t *testing.T) {
		// Insert a new task
		mission := Mission{
			OrganizationID: "org-1",
			Title:          "Parallel Mission",
			Priority:       "P1",
		}
		_, err := store.DecomposeMission(ctx, mission)
		if err != nil {
			t.Fatalf("Failed to decompose mission: %v", err)
		}

		// Run parallel claims
		claims := make(chan *Task, 5)
		for i := 0; i < 5; i++ {
			go func(i int) {
				task, _ := store.ClaimNextTask(ctx, "agent-parallel")
				claims <- task
			}(i)
		}

		// Collect results
		var successfulClaims int
		for i := 0; i < 5; i++ {
			select {
			case task := <-claims:
				if task != nil {
					successfulClaims++
				}
			case <-time.After(1 * time.Second):
				t.Fatal("Timeout waiting for claim")
			}
		}

		if successfulClaims != 1 {
			t.Errorf("Expected 1 successful claim, got %d", successfulClaims)
		}
	})
}
