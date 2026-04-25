package kairos

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"
	_ "modernc.org/sqlite"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/telemetry"
	"github.com/stretchr/testify/assert"
)

func setupTestDBSharedTasks(t *testing.T) db.Provider {
	conn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	p := db.NewSqliteProvider(conn)

	ctx := context.Background()

	// Create the required table
	_, err = p.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id VARCHAR PRIMARY KEY,
			agent_id VARCHAR,
			status VARCHAR NOT NULL DEFAULT 'PENDING',
			payload TEXT,
			created_at TEXT
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return p
}

func TestSharedTaskClaim(t *testing.T) {
	// Fulfill telemetry requirement
	telemetry.InitTelemetry()

	provider := setupTestDBSharedTasks(t)
	defer provider.Close()

	ctx := context.Background()

	repo := NewSharedTaskRepo(provider)

	// Insert task
	task := &SharedTask{
		ID:        "task-1",
		AgentID:   "",
		Status:    "PENDING",
		Payload:   json.RawMessage("{}"),
		CreatedAt: time.Now(),
	}

	err := repo.Insert(ctx, task)
	assert.NoError(t, err)

	claimedTask, err := repo.ClaimTask(ctx, "agent-1")
	assert.NoError(t, err)
	assert.NotNil(t, claimedTask)
	assert.Equal(t, "IN_PROGRESS", claimedTask.Status)
	assert.Equal(t, "agent-1", claimedTask.AgentID)
	assert.Equal(t, "task-1", claimedTask.ID)

	// Claiming again should yield no task
	claimedTask2, err := repo.ClaimTask(ctx, "agent-2")
	assert.NoError(t, err)
	assert.Nil(t, claimedTask2)
}

func TestSharedTaskClaim_Concurrent(t *testing.T) {
	telemetry.InitTelemetry()

	provider := setupTestDBSharedTasks(t)
	defer provider.Close()

	ctx := context.Background()

	repo := NewSharedTaskRepo(provider)

	task := &SharedTask{
		ID:        "task-1",
		AgentID:   "",
		Status:    "PENDING",
		Payload:   json.RawMessage("{}"),
		CreatedAt: time.Now(),
	}

	err := repo.Insert(ctx, task)
	assert.NoError(t, err)

	numWorkers := 10
	claimedCount := 0
	errCount := 0
	done := make(chan bool)

	for i := 0; i < numWorkers; i++ {
		go func(agent string) {
			claimedTask, err := repo.ClaimTask(context.Background(), agent)
			if err != nil {
				// We expect some to fail with concurrent SQLite transactions depending on locking,
				// or just return nil if task is already taken. Let's just count successful claims.
				done <- false
			} else {
				if claimedTask != nil {
					done <- true
				} else {
					done <- false
				}
			}
		}("agent-" + string(rune(i)))
	}

	for i := 0; i < numWorkers; i++ {
		success := <-done
		if success {
			claimedCount++
		} else {
			errCount++
		}
	}

	assert.Equal(t, 1, claimedCount, "Only one agent should successfully claim the task")
}
