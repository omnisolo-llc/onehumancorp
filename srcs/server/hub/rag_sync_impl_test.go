package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// A simplistic mock provider for basic interface testing.
type mockDBProvider struct {
	isSQLite bool
	execFunc func(ctx context.Context, sql string, arguments ...any) (int64, error)
	queryFunc func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error)
}

func (m *mockDBProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if m.execFunc != nil {
		return m.execFunc(ctx, sql, arguments...)
	}
	return 0, nil
}
func (m *mockDBProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	if m.queryFunc != nil {
		return m.queryFunc(ctx, sql, optionsAndArgs...)
	}
	return nil, nil
}
func (m *mockDBProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return nil }
func (m *mockDBProvider) Begin(ctx context.Context) (db.Tx, error) { return nil, nil }
func (m *mockDBProvider) Close() {}
func (m *mockDBProvider) IsSQLite() bool { return m.isSQLite }
func (m *mockDBProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) { return nil, nil }

// Mock rows
type mockRows struct {
	records []RAGSyncRecord
	index   int
}

func (m *mockRows) Next() bool {
	if m.index < len(m.records) {
		m.index++
		return true
	}
	return false
}

func (m *mockRows) Scan(dest ...any) error {
	rec := m.records[m.index-1]
	*dest[0].(*string) = rec.ID
	*dest[1].(*string) = rec.Context
	*dest[2].(*SyncStatus) = rec.SyncStatus

	lastSyncPtr := dest[3].(**time.Time)
	now := time.Now()
	*lastSyncPtr = &now

	return nil
}
func (m *mockRows) Close() {}

func TestRAGSyncServiceImpl_FetchPendingSyncs(t *testing.T) {
	provider := &mockDBProvider{
		isSQLite: true,
		queryFunc: func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
			return &mockRows{
				records: []RAGSyncRecord{
					{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
				},
			}, nil
		},
	}

	svc := NewRAGSyncService(provider)
	records, err := svc.FetchPendingSyncs(context.Background(), 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", records[0].ID)
	}
}

func TestRAGSyncServiceImpl_MarkSynced(t *testing.T) {
	execCount := 0
	provider := &mockDBProvider{
		isSQLite: true,
		execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
			execCount++
			return 1, nil
		},
	}

	svc := NewRAGSyncService(provider)
	err := svc.MarkSynced(context.Background(), []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if execCount != 2 {
		t.Errorf("expected 2 execs, got %d", execCount)
	}
}

func TestRAGSyncServiceImpl_ProcessIncomingSync(t *testing.T) {
	execCount := 0
	provider := &mockDBProvider{
		isSQLite: false,
		execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
			execCount++
			return 1, nil
		},
	}

	svc := NewRAGSyncService(provider)
	err := svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{
		{ID: "1", Context: "test1"},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if execCount != 1 {
		t.Errorf("expected 1 exec, got %d", execCount)
	}
}

// Adding missing methods to mockRows
func (m *mockRows) Columns() ([]string, error) {
	return []string{"key", "value", "sync_status", "last_sync_at"}, nil
}

func (m *mockRows) Err() error {
	return nil
}
