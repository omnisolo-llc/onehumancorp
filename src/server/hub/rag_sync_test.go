package hub

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
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

    // Test ProcessIncomingSync
    err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{
        {ID: "2", Context: "test2", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
    })
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(mock.process) != 1 || mock.process[0].ID != "2" {
        t.Fatalf("expected 1 processed record with ID 2, got %v", mock.process)
    }
}



func TestRAGSyncServiceImpl(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()

	ctx := context.Background()

	// Setup swarm_memory_embeddings table
	_, err := provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at DATETIME
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Test ProcessIncomingSync (INSERT)
	now := time.Now().Truncate(time.Second)
	vector := []float32{0.1, 0.2, 0.3}
	records := []RAGSyncRecord{
		{
			ID:         "mem1",
			Context:    "test context 1",
			Vector:     vector,
			SyncStatus: SyncStatusPending,
			LastSyncAt: now,
		},
		{
			ID:         "mem2",
			Context:    "test context 2",
			Vector:     nil,
			SyncStatus: SyncStatusPending,
			LastSyncAt: now,
		},
	}

	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify insertion
	var count int
	_ = provider.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings").Scan(&count)
	if count != 2 {
		t.Fatalf("Expected 2 records inserted, got %d", count)
	}

	// Test FetchPendingSyncs
	fetched, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(fetched) != 2 {
		t.Fatalf("Expected 2 fetched records, got %d", len(fetched))
	}

	// Verify Vector deserialization
	var foundMem1 bool
	for _, r := range fetched {
		if r.ID == "mem1" {
			foundMem1 = true
			if len(r.Vector) != 3 || r.Vector[0] != 0.1 {
				t.Errorf("Expected vector [0.1, 0.2, 0.3], got %v", r.Vector)
			}
		}
	}
	if !foundMem1 {
		t.Error("mem1 not found in fetched records")
	}

	// Test MarkSynced (Batch update)
	err = service.MarkSynced(ctx, []string{"mem1", "mem2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify update
	var syncStatus string
	_ = provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'mem1'").Scan(&syncStatus)
	if syncStatus != "synced" {
		t.Errorf("Expected sync_status 'synced', got %s", syncStatus)
	}

	// Fetch should return 0 now
	fetched, _ = service.FetchPendingSyncs(ctx, 10)
	if len(fetched) != 0 {
		t.Errorf("Expected 0 fetched records after MarkSynced, got %d", len(fetched))
	}

	// Test ProcessIncomingSync (UPDATE)
	records[0].Context = "updated context"
	records[0].Vector = []float32{0.9, 0.8}
	err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{records[0]})
	if err != nil {
		t.Fatalf("ProcessIncomingSync (UPDATE) failed: %v", err)
	}

	var updatedContext string
	var updatedVectorBytes []byte
	err = provider.QueryRow(ctx, "SELECT context, vector_embedding FROM swarm_memory_embeddings WHERE memory_id = 'mem1'").Scan(&updatedContext, &updatedVectorBytes)
	if err != nil {
		t.Fatalf("Failed to query updated record: %v", err)
	}
	if updatedContext != "updated context" {
		t.Errorf("Expected 'updated context', got '%s'", updatedContext)
	}
	var updatedVector []float32
	json.Unmarshal(updatedVectorBytes, &updatedVector)
	if len(updatedVector) != 2 || updatedVector[0] != 0.9 {
		t.Errorf("Expected vector [0.9, 0.8], got %v", updatedVector)
	}
}
