package hub

import (
	"context"
	"testing"
	"time"
)

type MockRAGSyncService struct {
	PendingSyncs []RAGSyncRecord
	SyncedIDs    []string
	IncomingRecs []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit < len(m.PendingSyncs) {
		return m.PendingSyncs[:limit], nil
	}
	return m.PendingSyncs, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.SyncedIDs = append(m.SyncedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.IncomingRecs = append(m.IncomingRecs, records...)
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	mock := &MockRAGSyncService{
		PendingSyncs: []RAGSyncRecord{
			{ID: "rec1", Context: "test context 1", SyncStatus: SyncStatusPending},
			{ID: "rec2", Context: "test context 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs returned error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("Expected 2 pending syncs, got %d", len(pending))
	}

	// Test MarkSynced
	err = mock.MarkSynced(ctx, []string{"rec1", "rec2"})
	if err != nil {
		t.Fatalf("MarkSynced returned error: %v", err)
	}
	if len(mock.SyncedIDs) != 2 {
		t.Errorf("Expected 2 synced IDs, got %d", len(mock.SyncedIDs))
	}

	// Test ProcessIncomingSync
	err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "rec1", Context: "test context 1", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	})
	if err != nil {
		t.Fatalf("ProcessIncomingSync returned error: %v", err)
	}
	if len(mock.IncomingRecs) != 1 {
		t.Errorf("Expected 1 incoming record, got %d", len(mock.IncomingRecs))
	}
}

type mockDBRows struct {
	records []RAGSyncRecord
	index   int
}

func (r *mockDBRows) Next() bool {
	if r.index < len(r.records) {
		return true
	}
	return false
}

func (r *mockDBRows) Scan(dest ...interface{}) error {
	rec := r.records[r.index]
	*dest[0].(*string) = rec.ID
	*dest[1].(*string) = rec.Context
	*dest[2].(*[]byte) = rec.Vector
	*dest[3].(*SyncStatus) = rec.SyncStatus
	*dest[4].(*time.Time) = rec.LastSyncAt
	r.index++
	return nil
}

func (r *mockDBRows) Close() error { return nil }

type mockDB struct {
	isSQLite bool
	execErr  error
	queryErr error
	rows     Rows
}

func (m *mockDB) ExecContext(ctx context.Context, query string, args ...interface{}) error {
	return m.execErr
}

func (m *mockDB) QueryContext(ctx context.Context, query string, args ...interface{}) (Rows, error) {
	return m.rows, m.queryErr
}

func (m *mockDB) IsSQLite() bool {
	return m.isSQLite
}

func TestRAGSyncServiceImpl(t *testing.T) {
	ctx := context.Background()

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		db := &mockDB{
			isSQLite: true,
			rows: &mockDBRows{
				records: []RAGSyncRecord{
					{ID: "1", Context: "ctx1"},
				},
			},
		}
		service := NewRAGSyncService(db)

		records, err := service.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(records) != 1 {
			t.Fatalf("expected 1 record, got %d", len(records))
		}
	})

	t.Run("MarkSynced", func(t *testing.T) {
		db := &mockDB{isSQLite: true}
		service := NewRAGSyncService(db)

		err := service.MarkSynced(ctx, []string{"1", "2"})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		db := &mockDB{isSQLite: false}
		service := NewRAGSyncService(db)

		err := service.ProcessIncomingSync(ctx, []RAGSyncRecord{
			{ID: "1", Context: "ctx1"},
		})
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
	})
}
