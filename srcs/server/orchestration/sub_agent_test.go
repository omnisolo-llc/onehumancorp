package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"gopkg.in/yaml.v3"
	"os"
	"path/filepath"
	"testing"
	"onehumancorp/srcs/server/pb"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type mockMeshTransport struct {
	published []string
}

func (m *mockMeshTransport) Publish(ctx context.Context, channel string, data []byte) error {
	m.published = append(m.published, string(data))
	return nil
}


func (m *mockMeshTransport) AdvertiseCapabilities(ctx context.Context, agent pb.Agent) error {
    return nil
}

func (m *mockMeshTransport) DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error) {
    return nil, nil
}

func (m *mockMeshTransport) StartHeartbeat(ctx context.Context, agent pb.Agent) {
}
func (m *mockMeshTransport) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	return nil
}

func setupTestDBForSubAgent(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", "file::memory:?cache=shared")
	require.NoError(t, err)

	query := `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT,
			title TEXT,
			description TEXT,
			status TEXT,
			assigned_agent_id TEXT,
			priority TEXT,
			payload JSON,
			parent_plan_id TEXT,
			dependencies JSON,
			created_at TIMESTAMP,
			updated_at TIMESTAMP
		);
	`
	_, err = db.Exec(query)
	require.NoError(t, err)

	return db
}

func TestSubAgentSpawner_SpawnStandalone(t *testing.T) {
	mesh := &mockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(mesh, true, 2)

	task := &SharedTask{
		ID: "test-task-standalone-1",
	}

	err := spawner.Spawn(context.Background(), task)
	assert.NoError(t, err)

	// Allow time for the goroutine to run (transient failures may take time to retry)
	time.Sleep(7 * time.Second)
	time.Sleep(7 * time.Second)

	// Check MeshTransport events
	foundSpawned := false
	foundCompleted := false
	for _, msg := range mesh.published {
		var payload map[string]interface{}
		_ = json.Unmarshal([]byte(msg), &payload)
		if payload["event"] == "SUB_AGENT_SPAWNED" && payload["task_id"] == "test-task-standalone-1" {
			foundSpawned = true
		}
		if payload["event"] == "SUB_AGENT_COMPLETED" && payload["task_id"] == "test-task-standalone-1" {
			foundCompleted = true
		}
	}
	assert.True(t, foundSpawned)
	assert.True(t, foundCompleted)

	// Check heartbeat file
	statusFile := filepath.Join(".agent-task", "status", "test-task-standalone-1.yml")
	_, err = os.Stat(statusFile)
	assert.NoError(t, err)

	bytes, _ := os.ReadFile(statusFile)
	var finalData map[string]interface{}
	_ = yaml.Unmarshal(bytes, &finalData)
	assert.Equal(t, "COMPLETED", finalData["status"])
}

func TestSubAgentSpawner_SpawnCloud(t *testing.T) {
	mesh := &mockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(mesh, false, 0)

	task := &SharedTask{
		ID: "test-task-cloud-1",
	}

	err := spawner.Spawn(context.Background(), task)
	assert.NoError(t, err)

	// Allow time for the goroutine to run
	time.Sleep(7 * time.Second)

	// Check MeshTransport events
	foundSpawned := false
	foundCompleted := false
	for _, msg := range mesh.published {
		var payload map[string]interface{}
		_ = json.Unmarshal([]byte(msg), &payload)
		if payload["event"] == "SUB_AGENT_SPAWNED" && payload["task_id"] == "test-task-cloud-1" {
			foundSpawned = true
		}
		if payload["event"] == "SUB_AGENT_COMPLETED" && payload["task_id"] == "test-task-cloud-1" {
			foundCompleted = true
		}
	}
	assert.True(t, foundSpawned)
	assert.True(t, foundCompleted)
}

func TestTaskOrchestrator_PollAndSpawn(t *testing.T) {
	db := setupTestDBForSubAgent(t)
	defer db.Close()

	store := NewSqliteTaskStore(db)

	// Create a DELEGATED task
	task := &SharedTask{
		ID:             "delegated-task-1",
		OrganizationID: "org-1",
		Title:          "Delegated Work",
		Status:         "PENDING",
		Priority:       "DELEGATED",
	}
	err := store.CreateTask(context.Background(), task)
	require.NoError(t, err)

	mesh := &mockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(mesh, true, 2)
	orchestrator := NewDefaultTaskOrchestrator(store, spawner)

	err = orchestrator.PollTasks(context.Background())
	assert.NoError(t, err)

	// Check if task status updated to ASSIGNED
	fetchedTask, err := store.GetTask(context.Background(), task.ID, task.OrganizationID)
	require.NoError(t, err)
	assert.Equal(t, "ASSIGNED", fetchedTask.Status)

	// Give spawner time
	time.Sleep(7 * time.Second)

	foundCompleted := false
	for _, msg := range mesh.published {
		var payload map[string]interface{}
		_ = json.Unmarshal([]byte(msg), &payload)
		if payload["event"] == "SUB_AGENT_COMPLETED" && payload["task_id"] == "delegated-task-1" {
			foundCompleted = true
		}
	}
	assert.True(t, foundCompleted)
}

func TestSubAgentTimeout(t *testing.T) {
	mesh := &mockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(mesh, false, 5)

	task := &SharedTask{
		ID: "timeout-task-1",
	}

	// Provide an already-canceled context so that executeTask checks the Done channel immediately and returns ctx.Err()
	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	err := spawner.executeTask(ctx, task)
	assert.Error(t, err)
	assert.ErrorIs(t, err, context.Canceled)
}

func TestSubAgentSpawner_CircuitBreaker(t *testing.T) {
	mesh := &mockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(mesh, false, 0)
	spawner.cb.threshold = 1 // Trip after 1 failure

	task := &SharedTask{
		ID:             "test-cb",
		OrganizationID: "org-test",
	}

	// First execution with short timeout will fail and trip breaker
	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Millisecond)
	defer cancel()

	spawner.runSubAgent(ctx, task)

	// Second execution should be blocked by circuit breaker
	task2 := &SharedTask{
		ID:             "test-cb-2",
		OrganizationID: "org-test",
	}
	spawner.runSubAgent(context.Background(), task2)

	foundPaused := false
	for _, msg := range mesh.published {
		var payload map[string]interface{}
		_ = json.Unmarshal([]byte(msg), &payload)
		if payload["event"] == "SUB_AGENT_PAUSED" && payload["task_id"] == "test-cb-2" {
			foundPaused = true
		}
	}
	assert.True(t, foundPaused)
}
func TestSubAgentSpawner_Monitor(t *testing.T) {
	spawner := NewDefaultSubAgentSpawner(&mockMeshTransport{}, false, 0)
	err := spawner.Monitor(context.Background())
	assert.NoError(t, err)
}

func TestSubAgentSpawner_CircuitBreaker_HalfOpen(t *testing.T) {
	cb := NewCircuitBreaker(1, 10*time.Millisecond)
	cb.RecordFailure()
	assert.False(t, cb.Allow())

	// Wait for timeout
	time.Sleep(15 * time.Millisecond)

	// Should enter half-open state and return true
	assert.True(t, cb.Allow())
}

func TestSubAgentSpawner_NilMesh(t *testing.T) {
	spawner := NewDefaultSubAgentSpawner(nil, false, 0)
	// Should not panic when broadcasting with nil mesh
	spawner.broadcastLifecycleEvent(context.Background(), "task-1", "EVENT")
}

func TestSubAgentSpawner_MaxConcurrencyFallback(t *testing.T) {
	spawner := NewDefaultSubAgentSpawner(&mockMeshTransport{}, true, -1)
	assert.NotNil(t, spawner.semaphore)
	assert.Equal(t, 5, cap(spawner.semaphore)) // Default maxConcurrency is 5
}

func TestTaskOrchestrator_StartBackgroundWorker(t *testing.T) {
	db := setupTestDBForSubAgent(t)
	defer db.Close()

	store := NewSqliteTaskStore(db)

	// Create a delegated task
	err := store.CreateTask(context.Background(), &SharedTask{
		ID:       "worker-delegated-1",
		Status:   "PENDING",
		Priority: "DELEGATED",
	})
	assert.NoError(t, err)

	spawner := NewDefaultSubAgentSpawner(&mockMeshTransport{}, true, 2)
	orchestrator := NewDefaultTaskOrchestrator(store, spawner)

	ctx, cancel := context.WithCancel(context.Background())
	orchestrator.StartBackgroundWorker(ctx)

	// Wait for the worker to poll
	time.Sleep(3 * time.Second)
	cancel() // Stop the worker

	// Ensure task was processed
	fetchedTask, _ := store.GetTask(context.Background(), "worker-delegated-1", "")
	assert.Equal(t, "ASSIGNED", fetchedTask.Status)
}

// mockTaskStore implements TaskStore for testing errors
type errorMockTaskStore struct {
	*SqliteTaskStore // Embed to fulfill interface mostly
}

func (m *errorMockTaskStore) PollDelegatedTasks(ctx context.Context, limit int) ([]*SharedTask, error) {
	return nil, assert.AnError
}

func TestTaskOrchestrator_PollTasks_Error(t *testing.T) {
	db := setupTestDBForSubAgent(t)
	defer db.Close()

	baseStore := NewSqliteTaskStore(db)
	store := &errorMockTaskStore{SqliteTaskStore: baseStore}

	spawner := NewDefaultSubAgentSpawner(&mockMeshTransport{}, true, 2)
	orchestrator := NewDefaultTaskOrchestrator(store, spawner)

	err := orchestrator.PollTasks(context.Background())
	assert.Error(t, err)
}
