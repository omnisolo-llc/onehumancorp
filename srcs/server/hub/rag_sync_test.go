package hub

import (
	"context"
	"database/sql"
	"errors"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

type MockRAGSyncService struct {
	FetchPendingSyncsFunc   func(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSyncedFunc          func(ctx context.Context, ids []string) error
	ProcessIncomingSyncFunc func(ctx context.Context, records []RAGSyncRecord) error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.FetchPendingSyncsFunc != nil {
		return m.FetchPendingSyncsFunc(ctx, limit)
	}
	return nil, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkSyncedFunc != nil {
		return m.MarkSyncedFunc(ctx, ids)
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessIncomingSyncFunc != nil {
		return m.ProcessIncomingSyncFunc(ctx, records)
	}
	return nil
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	mockService := &MockRAGSyncService{
		FetchPendingSyncsFunc: func(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
			return []RAGSyncRecord{
				{
					ID:         "1",
					Context:    "Test context",
					Vector:     []float32{0.1, 0.2, 0.3},
					SyncStatus: SyncStatusPending,
					LastSyncAt: time.Time{},
				},
			}, nil
		},
	}

	records, err := mockService.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Errorf("expected ID '1', got %s", records[0].ID)
	}
	if records[0].SyncStatus != SyncStatusPending {
		t.Errorf("expected status 'pending', got %s", records[0].SyncStatus)
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	called := false
	mockService := &MockRAGSyncService{
		MarkSyncedFunc: func(ctx context.Context, ids []string) error {
			called = true
			if len(ids) != 2 {
				t.Errorf("expected 2 ids, got %d", len(ids))
			}
			return nil
		},
	}

	err := mockService.MarkSynced(context.Background(), []string{"1", "2"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if !called {
		t.Error("expected MarkSynced to be called")
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	called := false
	mockService := &MockRAGSyncService{
		ProcessIncomingSyncFunc: func(ctx context.Context, records []RAGSyncRecord) error {
			called = true
			if len(records) != 1 {
				t.Errorf("expected 1 record, got %d", len(records))
			}
			if records[0].Context != "Incoming data" {
				t.Errorf("expected 'Incoming data', got '%s'", records[0].Context)
			}
			return nil
		},
	}

	records := []RAGSyncRecord{
		{
			ID:      "100",
			Context: "Incoming data",
			Vector:  []float32{1.0, 2.0},
		},
	}

	err := mockService.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if !called {
		t.Error("expected ProcessIncomingSync to be called")
	}
}

func TestRAGSyncService_FetchPendingSyncs_Error(t *testing.T) {
	mockService := &MockRAGSyncService{
		FetchPendingSyncsFunc: func(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
			return nil, errors.New("database connection failed")
		},
	}

	records, err := mockService.FetchPendingSyncs(context.Background(), 10)
	if err == nil {
		t.Fatal("expected error, got nil")
	}
	if err.Error() != "database connection failed" {
		t.Errorf("expected 'database connection failed', got '%s'", err.Error())
	}
	if records != nil {
		t.Errorf("expected records to be nil, got %v", records)
	}
}

func TestRAGSyncServiceImpl_SQLite(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite in-memory db: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	// Setup schema
	_, err = provider.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	// Insert test data
	_, err = provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES
		('msg1', 'Hello world', '[0.1, 0.2]', 'pending'),
		('msg2', 'Foo bar', '[0.3, 0.4]', 'pending'),
		('msg3', 'Already synced', '[0.5, 0.6]', 'synced');
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"msg1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify msg1 is synced
	pendingAgain, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pendingAgain) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pendingAgain))
	}

	if pendingAgain[0].ID != "msg2" {
		t.Errorf("expected pending msg2, got %s", pendingAgain[0].ID)
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:      "msg4",
			Context: "Incoming from cloud",
			Vector:  []float32{0.7, 0.8},
		},
		{
			ID:      "msg1", // Should update existing
			Context: "Updated from cloud",
			Vector:  []float32{0.9, 1.0},
		},
	}

	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify msg4 was inserted and msg1 was updated
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count synced records: %v", err)
	}

	// msg3 (original synced), msg1 (marked synced, then updated), msg4 (inserted as synced)
	if count != 3 {
		t.Errorf("expected 3 synced records, got %d", count)
	}
}
