package interop

import (
	"context"
	"os"
	"testing"
    "database/sql"

	_ "modernc.org/sqlite"
)

func TestClaimMission(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open db: %v", err)
	}
	defer db.Close()

    // setup basic sqlite schema matching postgres roughly for test
	_, err = db.Exec(`
        ATTACH DATABASE ':memory:' AS ohc_tasks;
		CREATE TABLE ohc_tasks.mission_queue (
			mission_id TEXT PRIMARY KEY,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'QUEUED',
			assigned_agent TEXT,
			priority TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
        INSERT INTO ohc_tasks.mission_queue (mission_id, title, status, priority, payload)
        VALUES ('123', 'test-mission', 'QUEUED', 'high', '{"test": true}');
	`)
    if err != nil {
		t.Fatalf("failed to seed db: %v", err)
	}

    // test SQLite fallback path
	os.Setenv("OHC_STANDALONE", "true")

	mission, err := ClaimMission(context.Background(), db, "agent-1")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
    if mission == nil {
        t.Fatalf("expected mission, got nil")
    }
    if mission.MissionID != "123" {
        t.Errorf("expected mission ID 123, got %s", mission.MissionID)
    }
    if mission.AssignedAgent != "agent-1" {
        t.Errorf("expected assigned agent agent-1, got %s", mission.AssignedAgent)
    }

    // Test postgres path error
    os.Setenv("OHC_STANDALONE", "false")
    _, err = ClaimMission(context.Background(), db, "agent-1")
	if err == nil {
		t.Errorf("expected error running postgres query against sqlite")
	}

}
