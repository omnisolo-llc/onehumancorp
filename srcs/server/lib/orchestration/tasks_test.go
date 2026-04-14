package orchestration

import (
	"context"
	"database/sql"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)


func NewTestProvider(t *testing.T) db.Provider {
	t.Helper()
	database, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	return db.NewSqliteProvider(database)
}

func setupTestDB(t *testing.T) db.Provider {
	provider := NewTestProvider(t)
	_, err := provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks_v2 (
			id VARCHAR PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			title VARCHAR NOT NULL,
			description TEXT,
			status VARCHAR NOT NULL DEFAULT 'PENDING',
			assigned_agent_id VARCHAR,
			priority VARCHAR NOT NULL DEFAULT 'P2',
			payload TEXT,
			parent_plan_id TEXT,
			dependencies TEXT NOT NULL DEFAULT '[]',
			locked_until TIMESTAMP,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}
	_, _ = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS ohc_memory_embeddings (
			id VARCHAR PRIMARY KEY,
			tenant_id VARCHAR NOT NULL,
			memory_type TEXT NOT NULL,
			content TEXT NOT NULL,
			embedding BLOB,
			created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
			source_task_id VARCHAR
		)
	`)
	return provider
}

func TestTaskStore_DecomposeAndClaim(t *testing.T) {
	t.Parallel()
	os.Setenv("OHC_STANDALONE", "true")
	provider := setupTestDB(t)
	store := NewTaskStore(provider)

	ctx := context.Background()
	mission := Mission{
		ID:             "m1",
		OrganizationID: "org1",
		Tasks: []Task{
			{
				ID:             "t1",
				OrganizationID: "org1",
				Title:          "Task 1",
				Description:    "Desc",
				Priority:       "P0",
				Payload:        "{}",
				Dependencies:   "[]",
			},
		},
	}

	_, err := store.DecomposeMission(ctx, mission)
	if err != nil {
		t.Fatalf("DecomposeMission failed: %v", err)
	}

	task, err := store.ClaimNextTask(ctx, "agent1")
	if err != nil {
		t.Fatalf("ClaimNextTask failed: %v", err)
	}
	if task == nil {
		t.Fatalf("Expected task, got nil")
	}
	if task.ID != "t1" {
		t.Errorf("Expected task ID t1, got %s", task.ID)
	}

	task2, err := store.ClaimNextTask(ctx, "agent2")
	if err != nil {
		t.Fatalf("Second ClaimNextTask failed: %v", err)
	}
	if task2 != nil {
		t.Fatalf("Expected no tasks available, got %s", task2.ID)
	}
}

func TestAutoDreamListener_BatchCompletedTasks(t *testing.T) {
	t.Parallel()
	os.Setenv("OHC_STANDALONE", "true")
	provider := setupTestDB(t)
	listener := NewAutoDreamListener(provider)
	ctx := context.Background()

	_, err := provider.Exec(ctx, `INSERT INTO shared_tasks_v2 (id, organization_id, title, status, payload) VALUES ('t-done', 'org1', 'done task', 'DONE', '{}')`)
	if err != nil {
		t.Fatalf("Insert failed: %v", err)
	}

	err = listener.BatchCompletedTasks(ctx)
	if err != nil {
		t.Fatalf("BatchCompletedTasks failed: %v", err)
	}

	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM shared_tasks_v2 WHERE id = 't-done'").Scan(&status)
	if err != nil {
		t.Fatalf("Query failed: %v", err)
	}
	if status != "ARCHIVED" {
		t.Errorf("Expected status ARCHIVED, got %s", status)
	}
}
