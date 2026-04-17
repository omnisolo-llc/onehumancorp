package sync

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/testutils"
)

func TestLocalRepository(t *testing.T) {
	ctx := context.Background()

	// Create temporary directory for SQLite db
	tmpDir, err := os.MkdirTemp("", "localrepo_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	dbPath := filepath.Join(tmpDir, "localrepo.db")

	// Initialize SQLite database
	dbInstance, err := db.NewProvider("sqlite", dbPath, false)
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}

	// Create schema manually for test
	_, err = dbInstance.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			synced_to_cloud BOOLEAN DEFAULT FALSE,
			cloud_mission_id TEXT,
			sync_error TEXT,
			last_synced_at TIMESTAMP,
			organization_id TEXT DEFAULT 'system'
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	repo := NewLocalRepository(dbInstance)

	// Test GetPendingSync
	t.Run("GetPendingSync_Empty", func(t *testing.T) {
		missions, err := repo.GetPendingSync(ctx, 10)
		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}
		if len(missions) != 0 {
			t.Fatalf("expected 0 missions, got: %d", len(missions))
		}
	})

	t.Run("GetPendingSync_WithData", func(t *testing.T) {
		_, err := dbInstance.Exec(ctx, `INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('m-1', 'PENDING', '{"role":"dev","task":"do stuff"}', false)`)
		if err != nil {
			t.Fatalf("failed to insert: %v", err)
		}

		missions, err := repo.GetPendingSync(ctx, 10)
		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}
		if len(missions) != 1 {
			t.Fatalf("expected 1 mission, got: %d", len(missions))
		}
		if missions[0].ID != "m-1" {
			t.Errorf("expected mission ID m-1, got %s", missions[0].ID)
		}
		if missions[0].Payload.Role != "dev" {
			t.Errorf("expected role dev, got %s", missions[0].Payload.Role)
		}
	})

	// Test MarkSynced
	t.Run("MarkSynced", func(t *testing.T) {
		err := repo.MarkSynced(ctx, "m-1", "c-1")
		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		var syncedToCloud bool
		var cloudID string
		err = dbInstance.QueryRow(ctx, `SELECT synced_to_cloud, cloud_mission_id FROM agent_missions WHERE id = 'm-1'`).Scan(&syncedToCloud, &cloudID)
		if err != nil {
			t.Fatalf("failed to query: %v", err)
		}
		if !syncedToCloud {
			t.Errorf("expected synced_to_cloud to be true")
		}
		if cloudID != "c-1" {
			t.Errorf("expected cloud_mission_id to be c-1, got %s", cloudID)
		}
	})

	// Test MarkSyncError
	t.Run("MarkSyncError", func(t *testing.T) {
		err := repo.MarkSyncError(ctx, "m-1", "network error")
		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		var syncError string
		err = dbInstance.QueryRow(ctx, `SELECT sync_error FROM agent_missions WHERE id = 'm-1'`).Scan(&syncError)
		if err != nil {
			t.Fatalf("failed to query: %v", err)
		}
		if syncError != "network error" {
			t.Errorf("expected sync error to be network error, got %s", syncError)
		}
	})

	// Test GetActiveEscalations
	t.Run("GetActiveEscalations", func(t *testing.T) {
		_, err := dbInstance.Exec(ctx, `INSERT INTO agent_missions (id, status, payload, synced_to_cloud, cloud_mission_id) VALUES ('m-2', 'BURSTING', '{}', true, 'c-2')`)
		if err != nil {
			t.Fatalf("failed to insert: %v", err)
		}

		missions, err := repo.GetActiveEscalations(ctx)
		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}
		// m-1 is PENDING, m-2 is BURSTING. Both have synced_to_cloud = true and cloud_mission_id != ''
		if len(missions) != 2 {
			t.Fatalf("expected 2 missions, got: %d", len(missions))
		}
	})

	// Test UpdateLocalStatus
	t.Run("UpdateLocalStatus", func(t *testing.T) {
		err := repo.UpdateLocalStatus(ctx, "m-2", "DONE")
		if err != nil {
			t.Fatalf("expected no error, got: %v", err)
		}

		var status string
		err = dbInstance.QueryRow(ctx, `SELECT status FROM agent_missions WHERE id = 'm-2'`).Scan(&status)
		if err != nil {
			t.Fatalf("failed to query: %v", err)
		}
		if status != "DONE" {
			t.Errorf("expected status DONE, got %s", status)
		}
	})
}
