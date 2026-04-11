package hub

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// mockProvider implements db.Provider
type mockProvider struct {
	isSQLite bool
	tx       *mockTx
	err      error
	rows     *mockRows
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 0, m.err
}

func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	if m.err != nil {
		return nil, m.err
	}
	return m.rows, nil
}

func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return nil
}

func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) {
	if m.err != nil {
		return nil, m.err
	}
	return m.tx, nil
}

func (m *mockProvider) Close() {}

func (m *mockProvider) IsSQLite() bool {
	return m.isSQLite
}

func (m *mockProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) {
	return nil, nil
}

// mockRows implements db.Rows
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
	*dest[2].(*[]byte) = rec.Vector
	*dest[3].(*SyncStatus) = rec.SyncStatus

	lastSyncPtr := dest[4].(**time.Time)
	if !rec.LastSyncAt.IsZero() {
		t := rec.LastSyncAt
		*lastSyncPtr = &t
	} else {
		*lastSyncPtr = nil
	}
	return nil
}

func (m *mockRows) Close() {}

func (m *mockRows) Columns() ([]string, error) {
	return nil, nil
}

func (m *mockRows) Err() error {
	return nil
}

// mockTx implements db.Tx
type mockTx struct {
	execErr   error
	commitErr error
	queries   []string
}

func (m *mockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	m.queries = append(m.queries, sql)
	return 0, m.execErr
}

func (m *mockTx) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return nil, nil
}

func (m *mockTx) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return nil
}

func (m *mockTx) Commit(ctx context.Context) error {
	return m.commitErr
}

func (m *mockTx) Rollback(ctx context.Context) error {
	return nil
}

func TestDefaultRAGSyncService_FetchPendingSyncs(t *testing.T) {
	tTime := time.Now()
	rows := &mockRows{
		records: []RAGSyncRecord{
			{ID: "1", Context: "ctx1", SyncStatus: SyncStatusPending, LastSyncAt: tTime},
			{ID: "2", Context: "ctx2", SyncStatus: SyncStatusPending},
		},
	}
	provider := &mockProvider{isSQLite: true, rows: rows}
	svc := NewDefaultRAGSyncService(provider)

	records, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}
	if records[0].ID != "1" || records[0].Context != "ctx1" || records[0].LastSyncAt != tTime {
		t.Errorf("record 1 mismatch")
	}
	if records[1].ID != "2" || records[1].Context != "ctx2" || !records[1].LastSyncAt.IsZero() {
		t.Errorf("record 2 mismatch")
	}

	// Test error case
	providerErr := &mockProvider{err: errors.New("db error")}
	svcErr := NewDefaultRAGSyncService(providerErr)
	_, err = svcErr.FetchPendingSyncs(context.Background(), 10)
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestDefaultRAGSyncService_MarkSynced(t *testing.T) {
	tx := &mockTx{}
	provider := &mockProvider{isSQLite: false, tx: tx}
	svc := NewDefaultRAGSyncService(provider)

	err := svc.MarkSynced(context.Background(), []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(tx.queries) != 2 {
		t.Fatalf("expected 2 queries, got %d", len(tx.queries))
	}
}

func TestDefaultRAGSyncService_ProcessIncomingSync(t *testing.T) {
	tx := &mockTx{}
	provider := &mockProvider{isSQLite: true, tx: tx}
	svc := NewDefaultRAGSyncService(provider)

	records := []RAGSyncRecord{
		{ID: "1", Context: "ctx1", Vector: []byte{1, 2, 3}},
	}

	err := svc.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(tx.queries) != 1 {
		t.Fatalf("expected 1 query, got %d", len(tx.queries))
	}
}
