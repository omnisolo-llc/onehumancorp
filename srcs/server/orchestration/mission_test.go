package orchestration

import (
	"context"
	"testing"
	"encoding/json"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestClaimMission(t *testing.T) {
	ctx := context.Background()

	// In memory DB for testing
	database, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}

	_, err = database.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS mission_queue (
			mission_id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'QUEUED',
			assigned_agent TEXT,
			priority TEXT NOT NULL,
			payload JSON NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	_, err = database.Exec(ctx, `
		INSERT INTO mission_queue (title, priority, payload)
		VALUES ('Test Mission', 'P0', '{}');
	`)
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}

	agentID := "worker-1"
	mission, err := ClaimMission(ctx, database, agentID)
	if err != nil {
		t.Fatalf("failed to claim mission: %v", err)
	}

	if mission.Status != "IN_PROGRESS" {
		t.Errorf("expected status IN_PROGRESS, got %s", mission.Status)
	}

	if mission.AssignedAgent == nil || *mission.AssignedAgent != agentID {
		t.Errorf("expected agentID %s, got %v", agentID, mission.AssignedAgent)
	}
}
