package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type mockMeshHub struct {
	published []string
}

func (m *mockMeshHub) Publish(ctx context.Context, channel string, data []byte) error {
	m.published = append(m.published, string(data))
	return nil
}

func (m *mockMeshHub) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
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
			agent_id TEXT,
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
	mesh := &mockMeshHub{}
	spawner := NewDefaultSubAgentSpawner(mesh, true, 2)

	task := &SharedTask{
		ID: "test-task-standalone-1",
	}

	err := spawner.Spawn(context.Background(), task)
	assert.NoError(t, err)

	// Allow time for the goroutine to run (transient failures may take time to retry)
	time.Sleep(2 * time.Second)
	time.Sleep(2 * time.Second)

	// Check MeshHub events
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
	statusFile := filepath.Join(".agent-task", "status", "test-task-standalone-1.json")
	_, err = os.Stat(statusFile)
	assert.NoError(t, err)

	bytes, _ := os.ReadFile(statusFile)
	var finalData map[string]interface{}
	_ = json.Unmarshal(bytes, &finalData)
	assert.Equal(t, "COMPLETED", finalData["status"])
}

func TestSubAgentSpawner_SpawnCloud(t *testing.T) {
	mesh := &mockMeshHub{}
	spawner := NewDefaultSubAgentSpawner(mesh, false, 0)

	task := &SharedTask{
		ID: "test-task-cloud-1",
	}

	err := spawner.Spawn(context.Background(), task)
	assert.NoError(t, err)

	// Allow time for the goroutine to run
	time.Sleep(2 * time.Second)

	// Check MeshHub events
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

	mesh := &mockMeshHub{}
	spawner := NewDefaultSubAgentSpawner(mesh, true, 2)
	orchestrator := NewDefaultTaskOrchestrator(store, spawner)

	err = orchestrator.PollTasks(context.Background())
	assert.NoError(t, err)

	// Check if task status updated to ASSIGNED
	fetchedTask, err := store.GetTask(context.Background(), task.ID)
	require.NoError(t, err)
	assert.Equal(t, "ASSIGNED", fetchedTask.Status)

	// Give spawner time
	time.Sleep(2 * time.Second)

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
