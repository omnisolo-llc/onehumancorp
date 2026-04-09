package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockDBRows struct {
	records []RAGSyncRecord
	idx     int
}
func (m *mockDBRows) Close() {}
func (m *mockDBRows) Err() error { return nil }
func (m *mockDBRows) Next() bool {
	if m.idx < len(m.records) {
		m.idx++
		return true
	}
	return false
}
func (m *mockDBRows) Scan(dest ...any) error {
	r := m.records[m.idx-1]
	*dest[0].(*string) = r.ID
	*dest[1].(*string) = r.Context
	*dest[2].(*[]byte) = nil
	*dest[3].(*SyncStatus) = r.SyncStatus

	now := time.Now()
	*dest[4].(**time.Time) = &now
	return nil
}
func (m *mockDBRows) Columns() ([]string, error) { return nil, nil }

type mockDBProvider struct {
	execCount int
}
func (m *mockDBProvider) Exec(ctx context.Context, query string, args ...any) (int64, error) {
	m.execCount++
	return 1, nil
}
func (m *mockDBProvider) Query(ctx context.Context, query string, args ...any) (db.Rows, error) {
	return &mockDBRows{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test context", SyncStatus: SyncStatusPending},
		},
	}, nil
}
func (m *mockDBProvider) IsSQLite() bool {
	return true
}

func TestDefaultRAGSyncService(t *testing.T) {
	mockDB := &mockDBProvider{}
	svc := NewDefaultRAGSyncService(mockDB)

	ctx := context.Background()
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected ID '1', got '%s'", records[0].ID)
	}

	err = svc.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if mockDB.execCount != 2 {
		t.Errorf("expected 2 execs, got %d", mockDB.execCount)
	}

	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "3", Context: "test3"},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if mockDB.execCount != 3 {
		t.Errorf("expected 3 execs, got %d", mockDB.execCount)
	}
}
