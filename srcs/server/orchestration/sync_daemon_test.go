package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

// ClearSemaphore drains the global throttleSemaphore so tests don't deadlock
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
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite3 memory db: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload BLOB,
			synced_to_cloud BOOLEAN DEFAULT false,
			sync_error TEXT,
			last_synced_at DATETIME
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_missions table: %v", err)
	}

	return db
}

func TestSyncPendingMissions(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)
	defer db.Close()

	// Insert test data
	_, err := db.Exec(`
		INSERT INTO agent_missions (id, status, payload, synced_to_cloud)
		VALUES
		('mission-1', 'CLOUD_ESCALATION', '{"data":"payload1"}', false),
		('mission-2', 'CLOUD_ESCALATION', '{"data":"payload2"}', true), -- already synced
		('mission-3', 'COMPLETED', '{"data":"payload3"}', false), -- wrong status
		('mission-4', 'BURSTING', '{"data":"payload4"}', false)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-url.com")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	err = daemon.SyncPendingMissions(ctx)
	if err != nil {
		t.Fatalf("SyncPendingMissions failed: %v", err)
	}

	// Verify database state
	rows, err := db.Query("SELECT id, synced_to_cloud FROM agent_missions ORDER BY id")
	if err != nil {
		t.Fatalf("failed to query agent_missions: %v", err)
	}
	defer rows.Close()

	expected := map[string]bool{
		"mission-1": true,
		"mission-2": true,
		"mission-3": false,
		"mission-4": false, // Based on the prompt we should fetch only CLOUD_ESCALATION
	}

	for rows.Next() {
		var id string
		var synced bool
		if err := rows.Scan(&id, &synced); err != nil {
			t.Fatalf("failed to scan row: %v", err)
		}
		if expected[id] != synced {
			t.Errorf("mission %s: expected synced_to_cloud=%v, got %v", id, expected[id], synced)
		}
	}
}

func TestSyncPendingMissions_ErrorHandling(t *testing.T) {
	ClearSemaphore()
	defer ClearSemaphore()

	db := setupTestDB(t)
	defer db.Close()

	daemon := NewHybridMCPRAGDaemon(db, "http://remote-url.com")

	ctx, cancel := context.WithCancel(context.Background())
	cancel() // Cancel immediately

	err := daemon.SyncPendingMissions(ctx)
	if err != nil {
		if err.Error() != "sync_daemon: failed to query agent_missions: context canceled" {
			t.Errorf("Expected context canceled error, got: %v", err)
		}
	}
}
