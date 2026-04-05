package orchestration

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
)

func TestSubAgentSpawner(t *testing.T) {
	// Setup DB
	provider := db.NewSQLiteProvider("file::memory:?cache=shared")
	defer provider.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	// Run migrations
	_, err := provider.Exec(ctx, `
		CREATE TABLE swarm_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT,
			title TEXT,
			description TEXT,
			status TEXT,
			payload TEXT,
			assigned_agent_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create tables: %v", err)
	}

	// Create Spawner
	spawner := NewSubAgentSpawner(provider, nil)

	// Create a dummy task
	taskID := "test-sub-agent-task-1"
	task := &models.Task{
		ID:        taskID,
		MissionID: "mission-1",
		Title:     "Test Delegate Task",
		Status:    "READY",
	}

	payloadMap := map[string]interface{}{
		"priority": "DELEGATED",
	}
	payloadBytes, _ := json.Marshal(payloadMap)
	task.Payload = string(payloadBytes)

	// Insert dummy task
	_, err = provider.Exec(ctx, `
		INSERT INTO swarm_tasks (id, mission_id, title, status, payload)
		VALUES ($1, $2, $3, $4, $5)
	`, task.ID, task.MissionID, task.Title, task.Status, task.Payload)
	if err != nil {
		t.Fatalf("Failed to insert dummy task: %v", err)
	}

	// Call Spawn
	err = spawner.Spawn(ctx, task)
	if err != nil {
		t.Fatalf("Failed to spawn sub-agent: %v", err)
	}

	// Check DB immediately for IN_PROGRESS status
	var status string
	var assignedAgentID string
	err = provider.QueryRow(ctx, `SELECT status, assigned_agent_id FROM swarm_tasks WHERE id = $1`, taskID).Scan(&status, &assignedAgentID)
	if err != nil {
		t.Fatalf("Failed to query task: %v", err)
	}

	if status != "IN_PROGRESS" {
		t.Errorf("Expected status to be IN_PROGRESS, got %s", status)
	}

	expectedAgentID := "sub_agent_" + taskID
	if assignedAgentID != expectedAgentID {
		t.Errorf("Expected assigned_agent_id to be %s, got %s", expectedAgentID, assignedAgentID)
	}

	// Wait for the background simulation to complete it
	time.Sleep(200 * time.Millisecond)

	err = provider.QueryRow(ctx, `SELECT status FROM swarm_tasks WHERE id = $1`, taskID).Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query task after simulation: %v", err)
	}

	if status != "COMPLETED" {
		t.Errorf("Expected status to be COMPLETED, got %s", status)
	}
}
