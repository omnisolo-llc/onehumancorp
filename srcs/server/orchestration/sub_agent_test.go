package orchestration

import (
	"context"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/ohc/srcs/server/db"
)

// MockTeammateMesh is a simple mock for TeammateMesh to track broadcasts
type MockTeammateMesh struct {
	mu         sync.Mutex
	broadcasts []Task
}

func (m *MockTeammateMesh) BroadcastTask(ctx context.Context, task Task) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.broadcasts = append(m.broadcasts, task)
	return nil
}
func (m *MockTeammateMesh) BroadcastDirectMessage(ctx context.Context, task Task, agentID string) error {
	return nil
}
func (m *MockTeammateMesh) SubscribeToTasks(ctx context.Context) (<-chan Task, error) {
	return nil, nil
}
func (m *MockTeammateMesh) SubscribeToDirectMessages(ctx context.Context, agentID string) (<-chan Task, error) {
	return nil, nil
}
func (m *MockTeammateMesh) Close() error {
	return nil
}

func TestSubAgentSpawner(t *testing.T) {
	// Initialize an in-memory SQLite DB
	dbProvider := db.NewSqliteProvider("sqlite://file::memory:?cache=shared")
	err := dbProvider.Connect(context.Background())
	if err != nil {
		t.Fatalf("Failed to connect to mock DB: %v", err)
	}

	// Create tables
	_, err = dbProvider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
			status TEXT
		);
		CREATE TABLE IF NOT EXISTS agent_heartbeats (
			id TEXT PRIMARY KEY,
			task_id TEXT,
			status TEXT,
			created_at DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create tables: %v", err)
	}

	mockMesh := &MockTeammateMesh{}
	spawner := NewSubAgentSpawner(dbProvider, nil, mockMesh, dbProvider)

	task := &SharedTask{
		ID:     "test-task-1",
		Status: "IN_PROGRESS",
	}

	_, err = dbProvider.Exec(context.Background(), "INSERT INTO swarm_tasks (id, status) VALUES ($1, $2)", task.ID, task.Status)
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

	ctx := context.Background()

	// Spawn the sub-agent
	err = spawner.Spawn(ctx, task)
	if err != nil {
		t.Fatalf("Failed to spawn: %v", err)
	}

	// Wait for the goroutine to finish via Monitor
	err = spawner.Monitor(ctx)
	if err != nil {
		t.Fatalf("Failed to monitor: %v", err)
	}

	// Wait a little extra to ensure DB updates finish
	time.Sleep(100 * time.Millisecond)

	// Check DB state
	var status string
	err = dbProvider.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = 'test-task-1'").Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}
	if status != "COMPLETED" {
		t.Errorf("Expected status COMPLETED, got %s", status)
	}

	// Check heartbeats
	var hbCount int
	err = dbProvider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_heartbeats WHERE task_id = 'test-task-1'").Scan(&hbCount)
	if err != nil {
		t.Fatalf("Failed to query heartbeats: %v", err)
	}
	if hbCount != 1 {
		t.Errorf("Expected 1 heartbeat, got %d", hbCount)
	}

	// Check mesh broadcasts
	mockMesh.mu.Lock()
	defer mockMesh.mu.Unlock()
	if len(mockMesh.broadcasts) != 2 {
		t.Errorf("Expected 2 broadcasts, got %d", len(mockMesh.broadcasts))
	} else {
		if mockMesh.broadcasts[0].Action != "SUB_AGENT_SPAWNED" {
			t.Errorf("Expected first broadcast action to be SUB_AGENT_SPAWNED, got %s", mockMesh.broadcasts[0].Action)
		}
		if mockMesh.broadcasts[1].Action != "SUB_AGENT_COMPLETED" {
			t.Errorf("Expected second broadcast action to be SUB_AGENT_COMPLETED, got %s", mockMesh.broadcasts[1].Action)
		}
	}
}
