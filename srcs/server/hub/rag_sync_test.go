package hub

import (
	"context"
	"database/sql"
	"testing"
	"reflect"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func newTestProvider(t *testing.T) db.Provider {
	t.Helper()
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := sqliteDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		sqliteDB.Close()
	})

	return db.NewSqliteProvider(sqliteDB)
}

func TestRAGSyncService(t *testing.T) {
	dbProvider := newTestProvider(t)

	ctx := context.Background()

	// Apply necessary schema manually for the test
	_, err := dbProvider.Exec(ctx, `
		CREATE TABLE consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT, -- TEXT for SQLite compatibility of vectors
			source_type TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	service := NewRAGSyncService(dbProvider)

	agentID := "agent-1"
	expectedVector := []float32{1.0, 2.5, -3.1}

	// Insert test data
	err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{
			ID:             "mem-1",
			OrganizationID: "org-1",
			AgentID:        &agentID,
			Context:        "hello world",
			Vector:         expectedVector,
			SourceType:     "agent",
			SyncStatus:     SyncStatusPending,
		},
	})
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Fetch pending
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "mem-1" {
		t.Errorf("expected id mem-1, got %s", records[0].ID)
	}
	if !reflect.DeepEqual(records[0].Vector, expectedVector) {
		t.Errorf("expected vector %v, got %v", expectedVector, records[0].Vector)
	}

	// Mark Synced
	err = service.MarkSynced(ctx, []string{"mem-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Fetch pending again, should be empty
	records2, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs 2 failed: %v", err)
	}
	if len(records2) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(records2))
	}
}
