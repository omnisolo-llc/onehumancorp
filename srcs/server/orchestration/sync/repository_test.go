package sync

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestSqliteLocalRepository(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			synced_to_cloud BOOLEAN DEFAULT false,
            cloud_mission_id TEXT,
            sync_error TEXT,
            last_synced_at TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_missions table: %v", err)
	}

	_, err = sqlDB.Exec(`
		INSERT INTO agent_missions (id, status, payload, synced_to_cloud, cloud_mission_id, sync_error, last_synced_at)
		VALUES
			('m1', 'PENDING', '{"task":"test-mission", "details":"[PRIVATE:secret] email is a@b.com"}', false, NULL, NULL, NULL),
			('m2', 'COMPLETED', '{"task":"synced-mission"}', true, NULL, NULL, NULL),
			('m3', 'BURSTING', '{"task":"burst-mission"}', false, NULL, NULL, NULL),
			('m4', 'PENDING', '{"task":"active-escalation"}', true, 'cloud-123', NULL, NULL)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}
	repo := NewSqliteLocalRepository(dbWrapper)

	ctx := context.Background()

	// Test GetPendingSync
	pending, err := repo.GetPendingSync(ctx, 10)
	if err != nil {
		t.Fatalf("GetPendingSync failed: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending missions, got %d", len(pending))
	}

	// Test MarkSynced
	err = repo.MarkSynced(ctx, "m1", "cloud-m1")
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	var synced bool
	var cloudID string
	err = sqlDB.QueryRow("SELECT synced_to_cloud, cloud_mission_id FROM agent_missions WHERE id = 'm1'").Scan(&synced, &cloudID)
	if err != nil || !synced || cloudID != "cloud-m1" {
		t.Errorf("MarkSynced failed to update db correctly")
	}

	// Test MarkSyncError
	err = repo.MarkSyncError(ctx, "m3", "some error")
	if err != nil {
		t.Fatalf("MarkSyncError failed: %v", err)
	}

	// Test GetActiveEscalations
	escalations, err := repo.GetActiveEscalations(ctx)
	if err != nil {
		t.Fatalf("GetActiveEscalations failed: %v", err)
	}
	if len(escalations) != 2 { // m1 and m4 now
		t.Errorf("expected 2 active escalations, got %d", len(escalations))
	}

	// Test UpdateLocalStatus
	err = repo.UpdateLocalStatus(ctx, "m4", "COMPLETED")
	if err != nil {
		t.Fatalf("UpdateLocalStatus failed: %v", err)
	}

	var status string
	err = sqlDB.QueryRow("SELECT status FROM agent_missions WHERE id = 'm4'").Scan(&status)
	if err != nil || status != "COMPLETED" {
		t.Errorf("UpdateLocalStatus failed to update db correctly")
	}
}
