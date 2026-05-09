package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"time"
    "encoding/json"
    "errors"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "github.com/mattn/go-sqlite3"
)

func setupSyncTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite3 memory db: %v", err)
	}

	createTableQuery := `
		CREATE TABLE shared_tasks (
			id VARCHAR PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			title VARCHAR NOT NULL,
			description TEXT,
			status VARCHAR NOT NULL DEFAULT 'PENDING',
			agent_id VARCHAR,
			priority VARCHAR NOT NULL DEFAULT 'P2',
			payload TEXT,
			parent_plan_id TEXT,
			dependencies TEXT NOT NULL DEFAULT '[]',
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
	`
	_, err = db.Exec(createTableQuery)
	if err != nil {
		t.Fatalf("Failed to create shared_tasks table: %v", err)
	}

	return db
}

type mockPostgresProvider struct {
	*SqliteTaskStore
}

func TestSyncDaemon_SyncPendingEscalations(t *testing.T) {
	localDB := setupSyncTestDB(t)
	defer localDB.Close()

	cloudDB := setupSyncTestDB(t)
	defer cloudDB.Close()

	localStore := NewSqliteTaskStore(localDB)
	cloudStore := &mockPostgresProvider{NewSqliteTaskStore(cloudDB)}

	task := &SharedTask{
		ID:             "task-1",
		OrganizationID: "org-1",
		Title:          "Task 1",
		Status:         "CLOUD_ESCALATION",
	}

	err := localStore.CreateTask(context.Background(), task)
	assert.NoError(t, err)

	err = localStore.UpdateTaskStatus(context.Background(), task.ID, "CLOUD_ESCALATION")
	assert.NoError(t, err)

	err = syncPendingEscalations(context.Background(), localStore, cloudStore)
	assert.NoError(t, err)

	// Check local DB
	localTask, err := localStore.GetTask(context.Background(), "task-1")
	assert.NoError(t, err)
	assert.Equal(t, "CLOUD_PROCESSING", localTask.Status)

	// Check cloud DB
	cloudTask, err := cloudStore.GetTask(context.Background(), "task-1")
	assert.NoError(t, err)
	assert.Equal(t, "PENDING", cloudTask.Status)
}

func TestSyncDaemon_SyncCompletedEscalations(t *testing.T) {
	localDB := setupSyncTestDB(t)
	defer localDB.Close()

	cloudDB := setupSyncTestDB(t)
	defer cloudDB.Close()

	localStore := NewSqliteTaskStore(localDB)
	cloudStore := &mockPostgresProvider{NewSqliteTaskStore(cloudDB)}

	// Setup local task in processing state
	localTask := &SharedTask{
		ID:             "task-1",
		OrganizationID: "org-1",
		Title:          "Task 1",
		Status:         "CLOUD_PROCESSING",
	}
	err := localStore.CreateTask(context.Background(), localTask)
	assert.NoError(t, err)

	err = localStore.UpdateTaskStatus(context.Background(), localTask.ID, "CLOUD_PROCESSING")
	assert.NoError(t, err)

	// Setup cloud task in DONE state
	cloudTask := &SharedTask{
		ID:             "task-1",
		OrganizationID: "org-1",
		Title:          "Task 1",
		Status:         "DONE",
	}
	err = cloudStore.CreateTask(context.Background(), cloudTask)
	assert.NoError(t, err)

	err = cloudStore.UpdateTaskStatus(context.Background(), cloudTask.ID, "DONE")
	assert.NoError(t, err)

	err = syncCompletedEscalations(context.Background(), localStore, cloudStore)
	assert.NoError(t, err)

	// Check local DB is updated to DONE
	updatedLocalTask, err := localStore.GetTask(context.Background(), "task-1")
	assert.NoError(t, err)
	assert.Equal(t, "DONE", updatedLocalTask.Status)
}

func TestSyncDaemon_SyncPendingMissions_SanitizeMockError(t *testing.T) {
	localDB := setupSyncTestDB(t)
	defer localDB.Close()

	cloudDB := setupSyncTestDB(t)
	defer cloudDB.Close()

	localStore := NewSqliteTaskStore(localDB)
	cloudStore := &mockPostgresProvider{NewSqliteTaskStore(cloudDB)}

    payload := `{"data": "test [PRIVATE:secret]"}`
	rawPayload := json.RawMessage(payload)
	task := &SharedTask{
		ID:             "task-1",
		OrganizationID: "org-1",
		Title:          "Task 1",
		Status:         "CLOUD_ESCALATION",
		Payload:        &rawPayload,
	}

	err := localStore.CreateTask(context.Background(), task)
	require.NoError(t, err)
    err = localStore.UpdateTaskStatus(context.Background(), task.ID, "CLOUD_ESCALATION")
    require.NoError(t, err)

    // Override SanitizePayloadFunc
    originalSanitize := SanitizePayloadFunc
    defer func() { SanitizePayloadFunc = originalSanitize }()
    SanitizePayloadFunc = func(payload string) (string, error) {
        return "", errors.New("mock sanitize error")
    }

	err = syncPendingEscalations(context.Background(), localStore, cloudStore)
	assert.NoError(t, err) // It continues on error
}

func TestSyncDaemon_SyncCompletedEscalations_QueryError(t *testing.T) {
	db := setupSyncTestDB(t)
	db.Close() // this will cause query to fail

	localStore := NewSqliteTaskStore(db)
	cloudStore := &mockPostgresProvider{NewSqliteTaskStore(db)}

	err := syncCompletedEscalations(context.Background(), localStore, cloudStore)
	assert.Error(t, err)
}

func TestSyncDaemon_SyncCompletedEscalations_CloudGetError(t *testing.T) {
	localDB := setupSyncTestDB(t)
	defer localDB.Close()

	cloudDB := setupSyncTestDB(t)

	localStore := NewSqliteTaskStore(localDB)
	cloudStore := &mockPostgresProvider{NewSqliteTaskStore(cloudDB)}

    task := &SharedTask{
		ID:             "task-1",
		OrganizationID: "org-1",
		Title:          "Task 1",
		Status:         "CLOUD_PROCESSING",
	}

	err := localStore.CreateTask(context.Background(), task)
	require.NoError(t, err)
    err = localStore.UpdateTaskStatus(context.Background(), task.ID, "CLOUD_PROCESSING")
    require.NoError(t, err)

    cloudDB.Close() // this will cause GetTask to fail

	err = syncCompletedEscalations(context.Background(), localStore, cloudStore)
	// Because we use AgentHarness with exponential backoff and circuit breaker
    // it will return the error instead of swallowing it if it fails repeatedly, but
    // StartSyncDaemon swallows it. Directly calling syncCompletedEscalations exposes it.
	assert.Error(t, err)
}

// ClearSemaphore drains the throttleSemaphore to prevent test deadlocks.
func ClearSemaphore() {
	for {
		select {
		case <-throttleSemaphore:
		default:
			return
		}
	}
}

func TestHybridMCPRAGDaemon_SyncPendingMissions(t *testing.T) {
	// Clean up global semaphore before and after the test
	ClearSemaphore()
	defer ClearSemaphore()

	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)
	defer db.Close()

	createTableQuery := `
	CREATE TABLE agent_missions (
		id TEXT PRIMARY KEY,
		status TEXT NOT NULL,
		payload BLOB,
		synced_to_cloud BOOLEAN DEFAULT FALSE
	);
	`
	_, err = db.Exec(createTableQuery)
	require.NoError(t, err)

	insertDataQuery := `
	INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES
	('mission-1', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE),
	('mission-2', 'CLOUD_ESCALATION', '{"key": "value2"}', FALSE),
	('mission-3', 'COMPLETED', '{"key": "value3"}', FALSE),
	('mission-4', 'CLOUD_ESCALATION', '{"key": "value4"}', TRUE);
	`
	_, err = db.Exec(insertDataQuery)
	require.NoError(t, err)

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")

	err = daemon.SyncPendingMissions(context.Background())
	require.NoError(t, err)

	rows, err := db.Query("SELECT id, synced_to_cloud FROM agent_missions")
	require.NoError(t, err)
	defer rows.Close()

	expectedState := map[string]bool{
		"mission-1": true,
		"mission-2": true,
		"mission-3": false,
		"mission-4": true,
	}

	for rows.Next() {
		var id string
		var synced bool
		err := rows.Scan(&id, &synced)
		require.NoError(t, err)

		expected, ok := expectedState[id]
		require.True(t, ok)
		require.Equal(t, expected, synced)
	}
}

func TestSyncDaemon_SyncPendingMissions_ContextCancel(t *testing.T) {
	// Clean up global semaphore before and after the test
	ClearSemaphore()
	defer ClearSemaphore()

	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)
	defer db.Close()

	createTableQuery := `
	CREATE TABLE agent_missions (
		id TEXT PRIMARY KEY,
		status TEXT NOT NULL,
		payload BLOB,
		synced_to_cloud BOOLEAN DEFAULT FALSE
	);
	`
	_, err = db.Exec(createTableQuery)
	require.NoError(t, err)

	// Insert more than 10 to fill up the throttle channel and force wait
	insertDataQuery := `
	INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES
	('mission-1', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE),
	('mission-2', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE),
	('mission-3', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE),
	('mission-4', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE),
	('mission-5', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE),
	('mission-6', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE),
	('mission-7', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE),
	('mission-8', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE),
	('mission-9', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE),
	('mission-10', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE),
	('mission-11', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE);
	`
	_, err = db.Exec(insertDataQuery)
	require.NoError(t, err)

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")
	// Block the semaphore so it hangs
	for i := 0; i < 10; i++ {
		throttleSemaphore <- struct{}{}
	}

	ctx, cancel := context.WithCancel(context.Background())
	// Cancel the context so it exits early
	go func() {
		time.Sleep(10 * time.Millisecond)
		cancel()
	}()
	err = daemon.SyncPendingMissions(ctx)

	assert.ErrorIs(t, err, context.Canceled)
}

func TestSyncDaemon_SyncPendingMissions_QueryError(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:") // Empty DB, no tables
	require.NoError(t, err)
	defer db.Close()

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")
	err = daemon.SyncPendingMissions(context.Background())

	assert.Error(t, err)
	assert.Contains(t, err.Error(), "sync_daemon: failed to query agent_missions")
}

func TestSyncDaemon_SyncPendingMissions_ScanError(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)
	defer db.Close()

	createTableQuery := `
	CREATE TABLE agent_missions (
		id TEXT PRIMARY KEY,
		status TEXT NOT NULL
		-- MISSING payload COLUMN to force scan error
	);
	`
	_, err = db.Exec(createTableQuery)
	require.NoError(t, err)

	insertDataQuery := `
	INSERT INTO agent_missions (id, status) VALUES
	('mission-1', 'CLOUD_ESCALATION');
	`
	_, err = db.Exec(insertDataQuery)
	require.NoError(t, err)

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")

	err = daemon.SyncPendingMissions(context.Background())
	assert.Error(t, err)
}

func TestSyncDaemon_StartSyncDaemon(t *testing.T) {
	localDB := setupSyncTestDB(t)
	defer localDB.Close()

	cloudDB := setupSyncTestDB(t)
	defer cloudDB.Close()

	localStore := NewSqliteTaskStore(localDB)
	cloudStore := &mockPostgresProvider{NewSqliteTaskStore(cloudDB)}

	task := &SharedTask{
		ID:             "task-1",
		OrganizationID: "org-1",
		Title:          "Task 1",
		Status:         "CLOUD_ESCALATION",
	}

	err := localStore.CreateTask(context.Background(), task)
	assert.NoError(t, err)

	err = localStore.UpdateTaskStatus(context.Background(), task.ID, "CLOUD_ESCALATION")
	assert.NoError(t, err)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go StartSyncDaemon(ctx, localStore, cloudStore)

	// Wait for a few iterations
	time.Sleep(200 * time.Millisecond)

	// Check local DB
	localTask, err := localStore.GetTask(context.Background(), "task-1")
	assert.NoError(t, err)
	assert.Equal(t, "CLOUD_PROCESSING", localTask.Status)

	// Check cloud DB
	cloudTask, err := cloudStore.GetTask(context.Background(), "task-1")
	assert.NoError(t, err)
	assert.Equal(t, "PENDING", cloudTask.Status)
}
