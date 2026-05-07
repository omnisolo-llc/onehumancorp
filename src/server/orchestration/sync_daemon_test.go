package orchestration

import (
	"context"
	"database/sql"
	"testing"

	// Using the blank import for modern sqlite driver
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
	CREATE TABLE agent_missions (
		id TEXT PRIMARY KEY,
		status TEXT NOT NULL,
		payload BLOB,
		synced_to_cloud BOOLEAN DEFAULT FALSE,
			sync_error TEXT,
			last_synced_at TIMESTAMP
	);
	`
	_, err = db.Exec(createTableQuery)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return db
}

func TestHybridMCPRAGDaemon_SyncPendingMissions(t *testing.T) {
	// Clean up global semaphore before and after the test
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)
	defer db.Close()

	// Insert test data
	insertDataQuery := `
	INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES
	('mission-1', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE),
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

	// Verify the database state
	rows, err := db.Query("SELECT id, synced_to_cloud FROM agent_missions")
	if err != nil {
		t.Fatalf("Failed to query database after sync: %v", err)
	}
	defer rows.Close()

	expectedState := map[string]bool{
		"mission-1": true,
		"mission-2": true,
		"mission-3": false, // Status was COMPLETED, not synced
		"mission-4": true,  // Already synced
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
// We rely on the internal syncToCloud mock rather than an HTTP mock because
// syncToCloud returns nil by default. We can mock it here if needed, but since it returns nil,
// it automatically succeeds.


func TestHybridMCPRAGDaemon_SyncPendingMissions_Cooldown(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)
	defer db.Close()

	// Insert test data with recent errors
	insertDataQuery := `
	INSERT INTO agent_missions (id, status, payload, synced_to_cloud, sync_error, last_synced_at) VALUES
	('mission-error-1', 'CLOUD_ESCALATION', '{"key": "value1"}', FALSE, 'API Timeout', datetime('now', '-1 minutes')),
	('mission-error-2', 'CLOUD_ESCALATION', '{"key": "value2"}', FALSE, 'HTTP 500', datetime('now', '-6 minutes'));
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

	syncedMap := make(map[string]bool)
	for rows.Next() {
		var id string
		var synced bool
		if err := rows.Scan(&id, &synced); err != nil {
			t.Fatalf("Failed to scan row: %v", err)
		}
		syncedMap[id] = synced
	}

	if syncedMap["mission-error-1"] != false {
		t.Errorf("Expected mission-error-1 to NOT be synced due to cooldown")
	}
	if syncedMap["mission-error-2"] != true {
		t.Errorf("Expected mission-error-2 to be synced after cooldown expired")
	}
}
