package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

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

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open test database: %v", err)
	}

	createTableQuery := `
	CREATE TABLE IF NOT EXISTS agent_missions (
		id TEXT PRIMARY KEY,
		status TEXT NOT NULL,
		payload BLOB,
		synced_to_cloud BOOLEAN DEFAULT FALSE
	);
	`
	_, err = db.Exec(createTableQuery)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return db
}

func TestHybridMCPRAGDaemon_SyncPendingMissions(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)
	defer db.Close()

	db.Exec("DELETE FROM agent_missions")
	insertDataQuery := `
	INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES
	('mission-1', 'CLOUD_ESCALATION', '{"key": "value1 [PRIVATE:secret]"}', FALSE),
	('mission-2', 'CLOUD_ESCALATION', '{"key": "value2"}', FALSE),
	('mission-3', 'COMPLETED', '{"key": "value3"}', FALSE),
	('mission-4', 'CLOUD_ESCALATION', '{"key": "value4"}', TRUE);
	`
	_, err := db.Exec(insertDataQuery)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")

	err = daemon.SyncPendingMissions(context.Background())
	if err != nil {
		t.Fatalf("SyncPendingMissions failed: %v", err)
	}

	rows, err := db.Query("SELECT id, synced_to_cloud FROM agent_missions")
	if err != nil {
		t.Fatalf("Failed to query database after sync: %v", err)
	}
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
		if err := rows.Scan(&id, &synced); err != nil {
			t.Fatalf("Failed to scan row: %v", err)
		}

		expected, ok := expectedState[id]
		if !ok {
			t.Fatalf("Unexpected mission ID found: %s", id)
		}
		if synced != expected {
			t.Errorf("Mission %s: expected synced_to_cloud=%v, got %v", id, expected, synced)
		}
	}
}

func TestHybridMCPRAGDaemon_SyncPendingMissions_QueryError(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)
	db.Close()

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")
	err := daemon.SyncPendingMissions(context.Background())
	if err == nil {
		t.Fatalf("Expected SyncPendingMissions to fail due to closed db")
	}
}

func TestHybridMCPRAGDaemon_SyncPendingMissions_ContextCancel(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)
	defer db.Close()

	db.Exec("DELETE FROM agent_missions")
	insertDataQuery := `
	INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES
	('mission-1', 'CLOUD_ESCALATION', '{"key": "value1 [PRIVATE:secret]"}', FALSE);
	`
	_, err := db.Exec(insertDataQuery)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")

	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	err = daemon.SyncPendingMissions(ctx)
	if err == nil {
		t.Fatalf("Expected SyncPendingMissions to fail due to context cancelled")
	}
}

func TestHybridMCPRAGDaemon_SyncPendingMissions_ScanError(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)
	defer db.Close()

	_, _ = db.Exec("DROP TABLE agent_missions;")
	createTableQuery := `
	CREATE TABLE agent_missions (
		id TEXT PRIMARY KEY,
		status TEXT NOT NULL,
		synced_to_cloud BOOLEAN DEFAULT FALSE
	);
	`
	_, _ = db.Exec(createTableQuery)

	insertDataQuery := `
	INSERT INTO agent_missions (id, status, synced_to_cloud) VALUES
	('mission-1', 'CLOUD_ESCALATION', FALSE);
	`
	_, _ = db.Exec(insertDataQuery)

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")
	err := daemon.SyncPendingMissions(context.Background())
	if err == nil {
		t.Fatalf("Expected error because of missing column payload")
	}
}

func TestHybridMCPRAGDaemon_SyncPendingMissions_SyncToCloudError(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)

	db.Exec("DELETE FROM agent_missions")
	insertDataQuery := `
	INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES
	('mission-1', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE);
	`
	_, err := db.Exec(insertDataQuery)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")
	daemon.syncToCloudFunc = func(ctx context.Context, id string, payload []byte) error {
		return context.DeadlineExceeded
	}

	err = daemon.SyncPendingMissions(context.Background())
	if err != nil {
		t.Fatalf("SyncPendingMissions should ignore individual syncToCloud errors")
	}
}

func TestHybridMCPRAGDaemon_SyncPendingMissions_UpdateError(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)

	db.Exec("DELETE FROM agent_missions")
	insertDataQuery := `
	INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES
	('mission-1', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE);
	`
	_, err := db.Exec(insertDataQuery)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")

	daemon.syncToCloudFunc = func(ctx context.Context, id string, payload []byte) error {
		db.Close()
		return nil
	}

	err = daemon.SyncPendingMissions(context.Background())
	if err != nil {
		t.Fatalf("SyncPendingMissions should ignore individual update errors")
	}
}

func TestHybridMCPRAGDaemon_SyncToCloudDefault(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")
	err := daemon.defaultSyncToCloud(context.Background(), "id", []byte("payload"))
	if err != nil {
		t.Fatalf("Expected nil from defaultSyncToCloud")
	}
}

func TestHybridMCPRAGDaemon_SyncPendingMissions_ContextDoneInLoop(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)
	defer db.Close()

	insertDataQuery := `
	INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES
	('mission-1', 'CLOUD_ESCALATION', '{"key": "value1 [PRIVATE:secret]"}', FALSE);
	`
	db.Exec(insertDataQuery)

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")
	ctx, cancel := context.WithCancel(context.Background())

	for i := 0; i < 10; i++ {
		throttleSemaphore <- struct{}{}
	}

	go func() {
		time.Sleep(10 * time.Millisecond)
		cancel()
	}()

	err := daemon.SyncPendingMissions(ctx)
	if err == nil {
		t.Fatalf("Expected SyncPendingMissions to fail due to context cancelled blocking on semaphore")
	}
}

func TestHybridMCPRAGDaemon_SyncPendingMissions_SanitizeMockError(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)
	defer db.Close()

	insertDataQuery := `
	INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES
	('mission-1', 'CLOUD_ESCALATION', '{"key": "value1 [PRIVATE:secret]"}', FALSE);
	`
	db.Exec(insertDataQuery)

	originalSanitize := SanitizePayloadFunc
	defer func() { SanitizePayloadFunc = originalSanitize }()
	SanitizePayloadFunc = func(payload string) (string, error) {
		return "", errors.New("mock sanitize error")
	}

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")
	err := daemon.SyncPendingMissions(context.Background())
	if err != nil {
		t.Fatalf("SyncPendingMissions failed: %v", err)
	}
}
