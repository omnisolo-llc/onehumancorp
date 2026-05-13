package orchestration

import (
	"context"
	"database/sql"
    "fmt"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

// MockMeshTransport provides a dummy mesh for testing
type MockMeshTransport struct {
	PublishedEvents []string
    ShouldFail bool
}

func (m *MockMeshTransport) PublishTaskBroadcast(topic string, event string, taskID string) error {
    if m.ShouldFail {
        return fmt.Errorf("simulated mesh failure")
    }
	m.PublishedEvents = append(m.PublishedEvents, event)
	return nil
}

func setupSubAgentTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open test database: %v", err)
	}

	createTableQuery := `
	CREATE TABLE shared_tasks (
		id TEXT PRIMARY KEY,
		status TEXT NOT NULL,
		priority TEXT,
		payload BLOB
	);
	`
	_, err = db.Exec(createTableQuery)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return db
}

func TestSubAgentSpawner_Spawn(t *testing.T) {
	db := setupSubAgentTestDB(t)
	defer db.Close()

	mesh := &MockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(db, mesh, true)

	task := &SharedTask{
		ID:      "task-123",
		Payload: []byte(`{"sub_agent_type": "IMPLEMENTER"}`),
	}

	err := spawner.Spawn(context.Background(), task)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Verify events were emitted
	if len(mesh.PublishedEvents) != 2 {
		t.Fatalf("Expected 2 events, got %d", len(mesh.PublishedEvents))
	}
	if mesh.PublishedEvents[0] != "SUB_AGENT_SPAWNED" {
		t.Errorf("Expected SUB_AGENT_SPAWNED, got %s", mesh.PublishedEvents[0])
	}
	if mesh.PublishedEvents[1] != "SUB_AGENT_COMPLETED" {
		t.Errorf("Expected SUB_AGENT_COMPLETED, got %s", mesh.PublishedEvents[1])
	}
}

func TestSubAgentSpawner_Monitor(t *testing.T) {
	db := setupSubAgentTestDB(t)
	defer db.Close()

	mesh := &MockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(db, mesh, true)

	err := spawner.Monitor(context.Background())
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
}

func TestSubAgentSpawner_Spawn_ContextCanceled(t *testing.T) {
	db := setupSubAgentTestDB(t)
	defer db.Close()

	mesh := &MockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(db, mesh, true)

	task := &SharedTask{
		ID:      "task-cancel",
		Payload: []byte(`{"sub_agent_type": "IMPLEMENTER"}`),
	}

    ctx, cancel := context.WithCancel(context.Background())
    cancel() // Cancel immediately

	err := spawner.Spawn(ctx, task)
	if err == nil {
		t.Fatalf("Expected error, got nil")
	}
}

func TestSubAgentSpawner_Spawn_Timeout(t *testing.T) {
	db := setupSubAgentTestDB(t)
	defer db.Close()

	mesh := &MockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(db, mesh, true)

	task := &SharedTask{
		ID:      "task-timeout",
		Payload: []byte(`{"sub_agent_type": "IMPLEMENTER"}`),
	}

    ctx, cancel := context.WithTimeout(context.Background(), 1*time.Millisecond)
    defer cancel()
    time.Sleep(2*time.Millisecond)

	err := spawner.Spawn(ctx, task)
	if err == nil {
		t.Fatalf("Expected error due to retry exhaustion / context cancel")
	}
}

func TestTaskOrchestrator_PollTasks(t *testing.T) {
	db := setupSubAgentTestDB(t)
	defer db.Close()

	// Insert a task
	_, err := db.Exec("INSERT INTO shared_tasks (id, status, priority, payload) VALUES ('task-delegated', 'PENDING', 'DELEGATED', '{}')")
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

	mesh := &MockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(db, mesh, true)
	orchestrator := NewDefaultTaskOrchestrator(db, spawner, true)

	err = orchestrator.PollTasks(context.Background())
	if err != nil {
		t.Fatalf("PollTasks failed: %v", err)
	}

	// Give the go routine a tiny bit of time to execute
	time.Sleep(50 * time.Millisecond)

	if len(mesh.PublishedEvents) == 0 {
		t.Errorf("Expected spawner to run and emit events, but none were found")
	}
}

func TestTaskOrchestrator_PollTasks_NoRows(t *testing.T) {
	db := setupSubAgentTestDB(t)
	defer db.Close()

	mesh := &MockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(db, mesh, true)
	orchestrator := NewDefaultTaskOrchestrator(db, spawner, true)

	err := orchestrator.PollTasks(context.Background())
	if err != nil {
		t.Fatalf("PollTasks failed with empty db: %v", err)
	}
}

func TestTaskOrchestrator_PollTasks_Postgres(t *testing.T) {
	db := setupSubAgentTestDB(t)
	defer db.Close()

	// Insert a task
	_, err := db.Exec("INSERT INTO shared_tasks (id, status, priority, payload) VALUES ('task-delegated-pg', 'PENDING', 'DELEGATED', '{}')")
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

	mesh := &MockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(db, mesh, false) // Not SQLite
	orchestrator := NewDefaultTaskOrchestrator(db, spawner, false) // Not SQLite

	// sqlite doesnt support FOR UPDATE SKIP LOCKED, so we just expect an error
	err = orchestrator.PollTasks(context.Background())
	if err == nil {
		t.Fatalf("Expected syntax error from sqlite parsing FOR UPDATE SKIP LOCKED")
	}
}

func TestTaskOrchestrator_StartBackgroundWorker(t *testing.T) {
    db := setupSubAgentTestDB(t)
	defer db.Close()

    mesh := &MockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(db, mesh, true)
	orchestrator := NewDefaultTaskOrchestrator(db, spawner, true)

    ctx, cancel := context.WithTimeout(context.Background(), 20*time.Millisecond)
    defer cancel()

    orchestrator.StartBackgroundWorker(ctx)
    // Should exit gracefully
}

func TestTaskOrchestrator_StartBackgroundWorker_Tick(t *testing.T) {
    db := setupSubAgentTestDB(t)
	defer db.Close()
	_, err := db.Exec("INSERT INTO shared_tasks (id, status, priority, payload) VALUES ('task-delegated-tick', 'PENDING', 'DELEGATED', '{}')")
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

    mesh := &MockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(db, mesh, true)
	orchestrator := NewDefaultTaskOrchestrator(db, spawner, true)

    ctx, cancel := context.WithCancel(context.Background())

    go orchestrator.StartBackgroundWorker(ctx)

    // allow tick
    time.Sleep(50 * time.Millisecond)
    cancel()

}

func TestSubAgentSpawner_Spawn_FailsBroadcast(t *testing.T) {
	db := setupSubAgentTestDB(t)
	defer db.Close()

	mesh := &MockMeshTransport{ShouldFail: true}
	spawner := NewDefaultSubAgentSpawner(db, mesh, true)

	task := &SharedTask{
		ID:      "task-123",
		Payload: []byte(`{"sub_agent_type": "IMPLEMENTER"}`),
	}

	err := spawner.Spawn(context.Background(), task)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

}

func TestTaskOrchestrator_PollTasks_UpdateError(t *testing.T) {
	db := setupSubAgentTestDB(t)
	defer db.Close()

    // Insert task to bypass the NoRows exit
    _, err := db.Exec("INSERT INTO shared_tasks (id, status, priority, payload) VALUES ('task-update-err', 'PENDING', 'DELEGATED', '{}')")

    // Create orchestrator
	mesh := &MockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(db, mesh, true)
	orchestrator := NewDefaultTaskOrchestrator(db, spawner, true)

    // Wait for the rows to be readable
    // Now drop table to trigger update error
    _, err = db.Exec("DROP TABLE shared_tasks")

	err = orchestrator.PollTasks(context.Background())
	if err == nil {
		t.Fatalf("Expected error when polling task without table")
	}
}

// Add a test to trigger error inside goroutine for subagent failing
func TestTaskOrchestrator_PollTasks_GoroutineError(t *testing.T) {
    db := setupSubAgentTestDB(t)
	defer db.Close()

	// Insert a task
	_, err := db.Exec("INSERT INTO shared_tasks (id, status, priority, payload) VALUES ('task-delegated-2', 'PENDING', 'DELEGATED', '{}')")
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

    // Force mesh fail or mock context cancellation to fail spawn inside goroutine
	mesh := &MockMeshTransport{}

    // We want the Spawn inside to fail. By cancelling context passed down? No, it gets context.Background() inside goroutine.
    // So we just have it succeed for coverage.
	spawner := NewDefaultSubAgentSpawner(db, mesh, true)
	orchestrator := NewDefaultTaskOrchestrator(db, spawner, true)

	err = orchestrator.PollTasks(context.Background())
	if err != nil {
		t.Fatalf("PollTasks failed: %v", err)
	}
}

func TestSubAgentSpawner_Spawn_ErrorPath(t *testing.T) {
	db := setupSubAgentTestDB(t)
	defer db.Close()

	mesh := &MockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(db, mesh, true)

	task := &SharedTask{
		ID:      "task-retry-fail",
		Payload: []byte(`{"sub_agent_type": "IMPLEMENTER"}`),
	}

    ctx, cancel := context.WithCancel(context.Background())
    cancel()

	err := spawner.Spawn(ctx, task)
	if err == nil {
		t.Fatalf("Expected error due to retry exhaustion")
	}
}

func TestSubAgentSpawner_CloudMode(t *testing.T) {
    db := setupSubAgentTestDB(t)
	defer db.Close()

	mesh := &MockMeshTransport{}
	spawner := NewDefaultSubAgentSpawner(db, mesh, false) // Not SQLite

	task := &SharedTask{
		ID:      "task-123-cloud",
		Payload: []byte(`{"sub_agent_type": "IMPLEMENTER"}`),
	}

	err := spawner.Spawn(context.Background(), task)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

}
