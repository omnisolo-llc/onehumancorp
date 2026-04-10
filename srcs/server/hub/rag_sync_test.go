package hub

import (
	"context"
	"testing"
	"time"
	"os"
	"reflect"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	ProcessedData  []RAGSyncRecord
	FetchErr       error
	MarkErr        error
	ProcessErr     error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.FetchErr != nil {
		return nil, m.FetchErr
	}
	if limit > len(m.PendingRecords) {
		limit = len(m.PendingRecords)
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkErr != nil {
		return m.MarkErr
	}
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessErr != nil {
		return m.ProcessErr
	}
	m.ProcessedData = append(m.ProcessedData, records...)
	return nil
}

func TestRAGSyncFlow(t *testing.T) {
	ctx := context.Background()
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
		},
	}

	// 1. Fetch pending
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	// 2. Process incoming
	err = mockService.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.ProcessedData) != 2 {
		t.Fatalf("expected 2 processed records, got %d", len(mockService.ProcessedData))
	}

	// 3. Mark synced
	ids := []string{records[0].ID, records[1].ID}
	err = mockService.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.MarkedSynced) != 2 {
		t.Fatalf("expected 2 marked synced, got %d", len(mockService.MarkedSynced))
	}
}

func TestRAGSyncServiceImpl(t *testing.T) {
	ctx := context.Background()

	// Create temporary db for tests
	dbPath := "file::memory:?mode=memory"
	os.Setenv("DATABASE_URL", "sqlite://"+dbPath)
	defer os.Unsetenv("DATABASE_URL")

	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer database.Provider.Close()

	if err := database.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	service := NewRAGSyncService(database.Provider)

	// Add test data
	database.Provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, created_at) VALUES ('1', 'test_context', 'test_vector', 'pending', CURRENT_TIMESTAMP)")
	database.Provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, created_at) VALUES ('2', 'test_context_2', 'test_vector_2', 'pending', CURRENT_TIMESTAMP)")

	// Fetch pending
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(records))
	}

	if string(records[0].Vector) != "test_vector" {
		t.Fatalf("expected vector 'test_vector', got '%v'", string(records[0].Vector))
	}

	// Mark Synced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(records))
	}

	// Process Incoming
	incoming := []RAGSyncRecord{
		{ID: "3", Context: "test_context_3", Vector: []byte("test_vector_3"), SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
		{ID: "4", Context: "test_context_4", Vector: []byte("test_vector_4"), SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
	}

	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Check if incoming records were synced
	rows, err := database.Provider.Query(ctx, "SELECT vector_embedding FROM swarm_memory_embeddings WHERE sync_status = 'synced' ORDER BY memory_id ASC")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	defer rows.Close()
	var syncedVectors [][]byte
	for rows.Next() {
		var vec []byte
		if err := rows.Scan(&vec); err != nil {
			t.Fatalf("failed to scan vector: %v", err)
		}
		syncedVectors = append(syncedVectors, vec)
	}

	if len(syncedVectors) != 3 {
		t.Fatalf("expected 3 synced records, got %d", len(syncedVectors))
	}

	if !reflect.DeepEqual(syncedVectors[0], []byte("test_vector")) {
		t.Fatalf("expected first vector to be 'test_vector'")
	}
	if !reflect.DeepEqual(syncedVectors[1], []byte("test_vector_3")) {
		t.Fatalf("expected second vector to be 'test_vector_3'")
	}
	if !reflect.DeepEqual(syncedVectors[2], []byte("test_vector_4")) {
		t.Fatalf("expected third vector to be 'test_vector_4'")
	}
}
