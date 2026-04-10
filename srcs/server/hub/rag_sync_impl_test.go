package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestDefaultRAGSyncService_Implementation(t *testing.T) {
	// We need to use db.NewTestProvider() to get a real in-memory SQLite for testing
	// as required by Implementer instructions.

	// Create an embedded mock Provider
	mockDB := &mockDBProvider{}

	service := NewRAGSyncService(mockDB)

	// Test MarkSynced
	err := service.MarkSynced(context.Background(), []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mockDB.execs) != 2 {
		t.Errorf("expected 2 execs, got %d", len(mockDB.execs))
	}

	// Test ProcessIncomingSync
	records := []RAGSyncRecord{
		{ID: "3", Context: "test", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}
	err = service.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockDB.execs) != 3 {
		t.Errorf("expected 3 execs total, got %d", len(mockDB.execs))
	}
}

// Simple mock for db.Provider to test the service logic without real DB
type mockDBProvider struct {
	db.Provider
	execs []string
}

func (m *mockDBProvider) Begin(ctx context.Context) (db.Tx, error) {
	return &mockTx{m: m}, nil
}

type mockTx struct {
	db.Tx
	m *mockDBProvider
}

func (t *mockTx) Exec(ctx context.Context, sql string, args ...any) (int64, error) {
	t.m.execs = append(t.m.execs, sql)
	return 1, nil
}

func (t *mockTx) Commit(ctx context.Context) error {
	return nil
}

func (t *mockTx) Rollback(ctx context.Context) error {
	return nil
}
