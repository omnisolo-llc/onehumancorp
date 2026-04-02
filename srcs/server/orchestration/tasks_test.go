package orchestration

import (
	"context"
	"encoding/json"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDB(t *testing.T) *db.DB {
	os.Setenv("DATABASE_URL", "sqlite://file:test_tasks_db?mode=memory&cache=shared")
	ctx := context.Background()
	testDB, err := db.New(ctx)
	if err != nil {
		t.Fatalf("Failed to create test db: %v", err)
	}

	schema := `
	CREATE TABLE IF NOT EXISTS swarm_tasks (
		id TEXT PRIMARY KEY,
		mission_id TEXT NOT NULL,
		title TEXT NOT NULL,
		status TEXT NOT NULL,
		assigned_agent_id TEXT,
		locked_until DATETIME,
		payload TEXT NOT NULL,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);
	`
	_, err = testDB.Exec(ctx, schema)
	if err != nil {
		t.Fatalf("Failed to setup schema: %v", err)
	}

	return testDB
}

func TestTaskManager_CreateAndClaim(t *testing.T) {
	testDB := setupTestDB(t)

	tm := NewTaskManager(testDB, nil, nil)

	ctx := context.Background()

	task := &SwarmTask{
		MissionID: "mission-123",
		Title:     "Test Task",
		Payload:   json.RawMessage(`{"data":"test"}`),
	}

	created, err := tm.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("CreateTask failed: %v", err)
	}

	if created.ID == "" {
		t.Error("Expected task ID to be set")
	}

	claimed, err := tm.ClaimTask(ctx, created.ID, "agent-007")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if !claimed {
		t.Error("Expected task to be claimed")
	}

	claimed2, err := tm.ClaimTask(ctx, created.ID, "agent-008")
	if err != nil {
		t.Fatalf("ClaimTask 2 failed: %v", err)
	}

	if claimed2 {
		t.Error("Expected task to not be claimed again")
	}

	err = tm.CompleteTask(ctx, created.ID, created.MissionID)
	if err != nil {
		t.Fatalf("CompleteTask failed: %v", err)
	}
}
