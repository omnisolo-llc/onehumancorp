package hub

import (
	"context"
	"testing"
	"time"
	"database/sql"

	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type MockRAGSyncService struct {
	PendingSyncs []RAGSyncRecord
	MarkedIDs    []string
	Processed    []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if len(m.PendingSyncs) > limit {
		return m.PendingSyncs[:limit], nil
	}
	return m.PendingSyncs, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedIDs = append(m.MarkedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Processed = append(m.Processed, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mock := &MockRAGSyncService{
		PendingSyncs: []RAGSyncRecord{
			{ID: "m1", Context: "test context", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	pending, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending) != 1 || pending[0].ID != "m1" {
		t.Fatalf("Expected 1 pending record with ID m1, got %v", pending)
	}

	err = mock.MarkSynced(ctx, []string{"m1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	if len(mock.MarkedIDs) != 1 || mock.MarkedIDs[0] != "m1" {
		t.Fatalf("Expected m1 to be marked synced, got %v", mock.MarkedIDs)
	}

	records := []RAGSyncRecord{
		{ID: "m2", Context: "incoming sync", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	}

	err = mock.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	if len(mock.Processed) != 1 || mock.Processed[0].ID != "m2" {
		t.Fatalf("Expected 1 processed record with ID m2, got %v", mock.Processed)
	}
}

func TestRAGSyncService_Concrete(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqliteDB.Close()

	_, err = sqliteDB.Exec(`CREATE TABLE swarm_memory_embeddings (
		memory_id TEXT PRIMARY KEY,
		context TEXT,
		sync_status TEXT,
		last_sync_at DATETIME
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)
	service := NewRAGSyncService(provider)

	ctx := context.Background()

	// 1. Insert pending record
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ($1, $2, $3)", "m1", "test context", string(SyncStatusPending))
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}

	// 2. Fetch Pending
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != "m1" {
		t.Fatalf("Expected 1 pending record with ID m1, got %v", pending)
	}

	// 3. Mark Synced
	err = service.MarkSynced(ctx, []string{"m1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify MarkSynced
	row := provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = $1", "m1")
	var status string
	if err := row.Scan(&status); err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != string(SyncStatusSynced) {
		t.Fatalf("expected status %s, got %s", SyncStatusSynced, status)
	}

	// 4. Process Incoming Sync
	records := []RAGSyncRecord{
		{ID: "m2", Context: "incoming context", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
		{ID: "m1", Context: "updated context", SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	}
	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify ProcessIncomingSync
	row = provider.QueryRow(ctx, "SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = $1", "m2")
	var ctxStr string
	if err := row.Scan(&ctxStr, &status); err != nil {
		t.Fatalf("failed to query m2: %v", err)
	}
	if ctxStr != "incoming context" || status != string(SyncStatusSynced) {
		t.Fatalf("m2 data incorrect")
	}

	row = provider.QueryRow(ctx, "SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = $1", "m1")
	if err := row.Scan(&ctxStr, &status); err != nil {
		t.Fatalf("failed to query m1: %v", err)
	}
	if ctxStr != "updated context" || status != string(SyncStatusSynced) {
		t.Fatalf("m1 data incorrect")
	}
}
