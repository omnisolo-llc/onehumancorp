package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestClaimTask(t *testing.T) {
	ctx := context.Background()

	// Use in-memory SQLite for testing
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared")
	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("Failed to create db provider: %v", err)
	}
	defer provider.Close()

	if err := provider.Migrate(ctx); err != nil {
		t.Fatalf("Failed to run migrations: %v", err)
	}

	h := NewHub()
	sipDB := &SIPDB{db: provider.DB()}
	h.SetSIPDB(sipDB)

	// Create a dummy task
	taskID := "00000000-0000-0000-0000-000000000000"
	_, err = sipDB.DB().Exec(ctx, "INSERT INTO swarm_tasks (id, title, status, payload) VALUES ($1, $2, $3, $4)", taskID, "Test Task", "PENDING", "{}")
	if err != nil {
		t.Fatalf("Failed to insert dummy task: %v", err)
	}

	// Test claiming the task
	agentID := "agent-123"
	claimed, err := h.ClaimTask(ctx, taskID, agentID)
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}
	if !claimed {
		t.Fatalf("Expected to claim task successfully")
	}

	// Verify task is locked
	var status string
	var assigned string
	row := sipDB.DB().QueryRow(ctx, "SELECT status, assigned_agent_id FROM swarm_tasks WHERE id = $1", taskID)
	err = row.Scan(&status, &assigned)
	if err != nil {
		t.Fatalf("Failed to read back task: %v", err)
	}

	if status != "IN_PROGRESS" || assigned != agentID {
		t.Fatalf("Task not updated properly. Status: %s, Assigned: %s", status, assigned)
	}

	// Try claiming again with another agent
	claimedAgain, err := h.ClaimTask(ctx, taskID, "agent-456")
	if err != nil {
		t.Fatalf("Second ClaimTask failed with error: %v", err)
	}
	if claimedAgain {
		t.Fatalf("Should not be able to claim a task that is IN_PROGRESS")
	}
}
