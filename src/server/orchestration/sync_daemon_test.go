package orchestration

import (
	"context"
	"database/sql"
	"testing"
    "time"

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
	CREATE TABLE IF NOT EXISTS sub_agent_jobs (
		id TEXT PRIMARY KEY,
		parent_task_id TEXT,
		agent_role TEXT NOT NULL,
		payload TEXT NOT NULL,
		status TEXT NOT NULL DEFAULT 'QUEUED',
		attempts INTEGER DEFAULT 0,
		max_attempts INTEGER DEFAULT 3,
		run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		locked_until TIMESTAMPTZ,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
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

    oldTime := time.Now().Add(-2 * time.Hour).Format("2006-01-02 15:04:05")
    recentTime := time.Now().Add(-5 * time.Minute).Format("2006-01-02 15:04:05")

    _, err = db.Exec(`INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, updated_at) VALUES ('stuck-job', 'parent', 'role', '{}', 'RUNNING', ?)`, oldTime)
    if err != nil { t.Fatalf("Failed to insert job %v", err) }
    _, err = db.Exec(`INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, updated_at) VALUES ('active-job', 'parent', 'role', '{}', 'RUNNING', ?)`, recentTime)
    if err != nil { t.Fatalf("Failed to insert job %v", err) }

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

    var stuckStatus string
    err = db.QueryRow("SELECT status FROM sub_agent_jobs WHERE id = 'stuck-job'").Scan(&stuckStatus)
    if err != nil { t.Fatalf("query failed") }
    if stuckStatus != "FAILED" { t.Fatalf("expected FAILED, got %v", stuckStatus) }

    var activeStatus string
    err = db.QueryRow("SELECT status FROM sub_agent_jobs WHERE id = 'active-job'").Scan(&activeStatus)
    if err != nil { t.Fatalf("query failed") }
    if activeStatus != "RUNNING" { t.Fatalf("expected RUNNING, got %v", activeStatus) }
}
