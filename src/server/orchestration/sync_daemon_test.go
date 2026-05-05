package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"net/http"
	"net/http/httptest"

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

func TestHybridMCPRAGDaemon_CheckSyncHealth(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	// 1. Test healthy
	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")
	err := daemon.CheckSyncHealth(context.Background())
	if err != nil {
		t.Errorf("CheckSyncHealth failed unexpectedly: %v", err)
	}

	// 2. Test mock HTTP cloud api
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/health" {
			w.WriteHeader(http.StatusOK)
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer ts.Close()

	daemonHTTP := NewHybridMCPRAGDaemon(db, ts.URL)
	err = daemonHTTP.CheckSyncHealth(context.Background())
	if err != nil {
		t.Errorf("CheckSyncHealth failed unexpectedly on real HTTP: %v", err)
	}

	// 3. Test backlog critically high
	for i := 0; i < 1005; i++ {
		_, _ = db.Exec("INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES (?, 'CLOUD_ESCALATION', 'p', FALSE)", i)
	}

	err = daemonHTTP.CheckSyncHealth(context.Background())
	if err == nil {
		t.Errorf("Expected CheckSyncHealth to fail due to critical backlog")
	}

	// 4. Test closed db
	db.Close()
	err = daemon.CheckSyncHealth(context.Background())
	if err == nil {
		t.Errorf("Expected CheckSyncHealth to fail on closed db")
	}
}

func TestHybridMCPRAGDaemon_SanitizeBacklog(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	// Insert test data
	insertDataQuery := `
	INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES
	('mission-valid', 'CLOUD_ESCALATION', '{"key": "value"}', FALSE),
	('mission-stuck-null', 'CLOUD_ESCALATION', NULL, FALSE),
	('mission-stuck-empty', 'CLOUD_ESCALATION', '', FALSE);
	`
	_, err := db.Exec(insertDataQuery)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.test")

	err = daemon.SanitizeBacklog(context.Background())
	if err != nil {
		t.Fatalf("SanitizeBacklog failed: %v", err)
	}

	rows, _ := db.Query("SELECT id, status FROM agent_missions")
	defer rows.Close()

	for rows.Next() {
		var id, status string
		_ = rows.Scan(&id, &status)
		if id == "mission-valid" && status != "CLOUD_ESCALATION" {
			t.Errorf("Valid mission status altered")
		}
		if (id == "mission-stuck-null" || id == "mission-stuck-empty") && status != "FAILED" {
			t.Errorf("Stuck mission status not FAILED: %s is %s", id, status)
		}
	}
}

func TestHybridMCPRAGDaemon_SyncPendingMissions_ErrorPaths(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()
	db := setupTestDB(t)
	defer db.Close()

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-api.fail")

	_, _ = db.Exec("INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-fail', 'CLOUD_ESCALATION', 'p', FALSE)")

	err := daemon.SyncPendingMissions(context.Background())
	if err != nil {
		t.Errorf("Expected no overall error on single mission sync fail, got: %v", err)
	}

	// Should not be synced
	var synced bool
	_ = db.QueryRow("SELECT synced_to_cloud FROM agent_missions WHERE id = 'mission-fail'").Scan(&synced)
	if synced {
		t.Errorf("Mission was synced when failure was expected")
	}

	// Close db to force query error
	db.Close()
	err = daemon.SyncPendingMissions(context.Background())
	if err == nil {
		t.Errorf("Expected error on closed DB")
	}
}
