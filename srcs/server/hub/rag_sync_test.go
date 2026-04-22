package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockRAGSyncService struct {
	pending []RAGSyncRecord
	synced  []string
	process []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.pending) {
		return m.pending, nil
	}
	return m.pending[:limit], nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.synced = append(m.synced, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.process = append(m.process, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mock := &mockRAGSyncService{
		pending: []RAGSyncRecord{
			{ID: "1", Context: "test", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test Fetch
	records, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	// Test MarkSynced
	err = mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.synced) != 1 || mock.synced[0] != "1" {
		t.Fatalf("expected 1 synced record with ID 1, got %v", mock.synced)
	}

	now := time.Now()
	// Test ProcessIncomingSync
	err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "2", Context: "test2", SyncStatus: SyncStatusSynced, LastSyncAt: &now},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mock.process) != 1 || mock.process[0].ID != "2" {
		t.Fatalf("expected 1 processed record with ID 2, got %v", mock.process)
	}
}

func TestRAGSyncService_Implementation(t *testing.T) {
	ctx := context.Background()
	t.Setenv("DATABASE_URL", "sqlite://:memory:")

	// Use real in-memory SQLite database
	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to create database: %v", err)
	}
	defer database.Close()

	if err := database.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	service := NewRAGSyncService(database.Provider)

	// 1. Seed some data
	_, err = database.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES ('mem1', 'context1', 'pending'), ('mem2', 'context2', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to seed data: %v", err)
	}

	// 2. Fetch pending syncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	// 3. Mark synced
	err = service.MarkSynced(ctx, []string{"mem1", "mem2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// 4. Verify marked synced
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 pending records, got %d", len(records))
	}

	// 5. Process incoming sync (Upsert)
	now := time.Now().UTC()
	newRecords := []RAGSyncRecord{
		{
			ID:         "mem1",
			Context:    "context1-updated",
			Vector:     []float32{0.1, 0.2, 0.3},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: &now,
		},
		{
			ID:         "mem3",
			Context:    "context3",
			Vector:     []float32{0.4, 0.5, 0.6},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: &now,
		},
	}
	err = service.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// 6. Verify upsert
	var count int
	_ = database.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings").Scan(&count)
	if count != 3 {
		t.Fatalf("expected 3 total records, got %d", count)
	}

	var ctxUpdated string
	_ = database.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'mem1'").Scan(&ctxUpdated)
	if ctxUpdated != "context1-updated" {
		t.Fatalf("expected context1-updated, got %s", ctxUpdated)
	}
}
