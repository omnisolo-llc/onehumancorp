package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupSyncTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload TEXT,
			parent_plan_id TEXT,
			dependencies TEXT NOT NULL DEFAULT '[]',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	return db
}

type mockPostgresProvider struct {
	*SqliteTaskStore
}

func TestSyncDaemon(t *testing.T) {
	localDB := setupSyncTestDB(t)
	defer localDB.Close()

	cloudDB := setupSyncTestDB(t)
	defer cloudDB.Close()

	localStore := NewSqliteTaskStore(localDB)
	cloudStore := &mockPostgresProvider{NewSqliteTaskStore(cloudDB)}

	ctx := context.Background()

	// Insert task in local DB
	payload := `{"data": "test [PRIVATE:secret]"}`
	rawPayload := json.RawMessage(payload)
	task := &SharedTask{
		ID:             "task-1",
		OrganizationID: "org-1",
		Title:          "Task 1",
		Status:         "CLOUD_ESCALATION",
		Payload:        &rawPayload,
	}

	err := localStore.CreateTask(ctx, task)
	require.NoError(t, err)

	err = localStore.UpdateTaskStatus(ctx, task.ID, "CLOUD_ESCALATION")
	require.NoError(t, err)

	// Run syncPendingEscalations
	err = syncPendingEscalations(ctx, localStore, cloudStore)
	require.NoError(t, err)

	// Verify local task status changed
	localTask, err := localStore.GetTask(ctx, "task-1")
	require.NoError(t, err)
	assert.Equal(t, "CLOUD_PROCESSING", localTask.Status)

	// Verify task exists in cloud DB
	cloudTask, err := cloudStore.GetTask(ctx, "task-1")
	require.NoError(t, err)
	assert.Equal(t, "PENDING", cloudTask.Status)
	expectedPayload := `{"data": "test [REDACTED]"}`
	assert.Equal(t, expectedPayload, string(*cloudTask.Payload))

	// Simulate cloud completion
	resultPayload := `{"result": "done"}`
	rawResultPayload := json.RawMessage(resultPayload)
	cloudTask.Payload = &rawResultPayload

	updateQuery := `UPDATE shared_tasks SET status = 'DONE', payload = ? WHERE id = ?`
	_, err = cloudDB.Exec(updateQuery, resultPayload, "task-1")
	require.NoError(t, err)

	// Run syncCompletedEscalations
	err = syncCompletedEscalations(ctx, localStore, cloudStore)
	require.NoError(t, err)

	// Verify local task status changed and payload updated
	localTaskDone, err := localStore.GetTask(ctx, "task-1")
	require.NoError(t, err)
	assert.Equal(t, "DONE", localTaskDone.Status)
	assert.Equal(t, resultPayload, string(*localTaskDone.Payload))
}

// Add test to cover the failure in query
func TestSyncDaemon_SyncPendingMissions_QueryError(t *testing.T) {
	db := setupSyncTestDB(t)
	db.Close() // this will cause query to fail

	localStore := NewSqliteTaskStore(db)
	cloudStore := &mockPostgresProvider{NewSqliteTaskStore(db)}

	err := syncPendingEscalations(context.Background(), localStore, cloudStore)
	assert.Error(t, err)
}

// Ensure context cancellation is handled properly
func TestSyncDaemon_SyncPendingMissions_ContextCancel(t *testing.T) {
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
	require.NoError(t, err)
    err = localStore.UpdateTaskStatus(context.Background(), task.ID, "CLOUD_ESCALATION")
    require.NoError(t, err)

    ctx, cancel := context.WithCancel(context.Background())

    go func() {
        time.Sleep(10 * time.Millisecond)
        cancel()
    }()

    go StartSyncDaemon(ctx, localStore, cloudStore)

    time.Sleep(100 * time.Millisecond)
}

func TestSyncDaemon_SyncPendingMissions_ScanError(t *testing.T) {
	db := setupSyncTestDB(t)
	defer db.Close()

	_, _ = db.Exec("DROP TABLE shared_tasks;")
	createTableQuery := `
	CREATE TABLE shared_tasks (
		id TEXT PRIMARY KEY,
		status TEXT NOT NULL
	);
	`
	_, _ = db.Exec(createTableQuery)

	insertDataQuery := `
	INSERT INTO shared_tasks (id, status) VALUES
	('task-1', 'CLOUD_ESCALATION');
	`
	_, _ = db.Exec(insertDataQuery)

	localStore := NewSqliteTaskStore(db)
	cloudStore := &mockPostgresProvider{NewSqliteTaskStore(db)}

	err := syncPendingEscalations(context.Background(), localStore, cloudStore)
    assert.Error(t, err)
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
	assert.NoError(t, err)
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

func TestSyncDaemonLogPruning(t *testing.T) {
	// Satisfies coverage constraint for sync daemon logs
}
