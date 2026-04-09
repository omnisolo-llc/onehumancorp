package hub

import (
	"context"
	"testing"
	"time"
    "github.com/onehumancorp/mono/srcs/server/db"
)

type mockProvider struct {
    db.Provider
}

func (m *mockProvider) Query(ctx context.Context, query string, args ...interface{}) (db.Rows, error) {
    return &mockRows{}, nil
}

func (m *mockProvider) Exec(ctx context.Context, query string, args ...interface{}) (int64, error) {
    return 1, nil
}

func (m *mockProvider) QueryRow(ctx context.Context, query string, args ...interface{}) db.Row {
    return &mockRow{}
}

type mockRows struct {
    count int
}

func (m *mockRows) Next() bool {
    if m.count == 0 {
        m.count++
        return true
    }
    return false
}

func (m *mockRows) Scan(dest ...interface{}) error {
    id := dest[0].(*string)
    ctx := dest[1].(*string)
    status := dest[2].(*SyncStatus)

    *id = "1"
    *ctx = "mock context"
    *status = SyncStatusPending
    return nil
}

func (m *mockRows) Close() {}
func (m *mockRows) Err() error { return nil }
func (m *mockRows) Columns() ([]string, error) { return nil, nil }

type mockRow struct {}
func (m *mockRow) Scan(dest ...interface{}) error {
    status := dest[0].(*string)
    *status = "synced"
    return nil
}

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()
    provider := &mockProvider{}
    service := NewSQLRAGSyncService(provider)

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].SyncStatus != SyncStatusPending {
		t.Errorf("expected pending status, got %s", records[0].SyncStatus)
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{
			ID:         "2",
			Context:    "incoming context",
			Vector:     []float32{0.4, 0.5, 0.6},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	})
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
}
