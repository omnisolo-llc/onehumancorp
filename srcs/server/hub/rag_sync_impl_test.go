package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"testing"
	"time"
)

type mockRows struct {
	records []RAGSyncRecord
	index   int
	err     error
}

func (m *mockRows) Next() bool {
	if m.index < len(m.records) {
		return true
	}
	return false
}

func (m *mockRows) Scan(dest ...any) error {
	r := m.records[m.index]
	*dest[0].(*string) = r.ID
	*dest[1].(*string) = r.Context

	vectorJSON, _ := json.Marshal(r.Vector)
	b := dest[2].(*[]byte)
	*b = vectorJSON

	*dest[3].(*SyncStatus) = r.SyncStatus

	ns := dest[4].(*sql.NullTime)
	ns.Valid = true
	ns.Time = r.LastSyncAt

	m.index++
	return nil
}

func (m *mockRows) Close() error { return nil }
func (m *mockRows) Columns() ([]string, error) {
	return []string{"memory_id", "context", "vector_embedding", "sync_status", "last_sync_at"}, nil
}
func (m *mockRows) Err() error { return m.err }

type mockTx struct {
	execErr   error
	commitErr error
}

func (t *mockTx) ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error) {
	return nil, t.execErr
}
func (t *mockTx) Commit() error {
	return t.commitErr
}
func (t *mockTx) Rollback() error {
	return nil
}

type mockDBProvider struct {
	queryErr error
	execErr  error
	txErr    error
	rows     Rows
	tx       Tx
}

func (m *mockDBProvider) QueryContext(ctx context.Context, query string, args ...any) (Rows, error) {
	if m.queryErr != nil {
		return nil, m.queryErr
	}
	return m.rows, nil
}

func (m *mockDBProvider) ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error) {
	return nil, m.execErr
}

func (m *mockDBProvider) BeginTx(ctx context.Context, opts *sql.TxOptions) (Tx, error) {
	if m.txErr != nil {
		return nil, m.txErr
	}
	return m.tx, nil
}

func TestFetchPendingSyncsSuccess(t *testing.T) {
	rows := &mockRows{
		records: []RAGSyncRecord{
			{ID: "1", Context: "ctx", Vector: []float32{1.0}, SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
		},
	}
	db := &mockDBProvider{rows: rows}
	svc := NewRAGSyncService(db)

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

func TestFetchPendingSyncsQueryError(t *testing.T) {
	db := &mockDBProvider{queryErr: errors.New("query error")}
	svc := NewRAGSyncService(db)

	_, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err == nil {
		t.Error("expected error, got nil")
	}
}

func TestFetchPendingSyncsRowsError(t *testing.T) {
	rows := &mockRows{
		records: []RAGSyncRecord{{ID: "1"}},
		err:     errors.New("rows error"),
	}
	db := &mockDBProvider{rows: rows}
	svc := NewRAGSyncService(db)

	_, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err == nil {
		t.Error("expected error, got nil")
	}
}

func TestMarkSyncedSuccess(t *testing.T) {
	tx := &mockTx{}
	db := &mockDBProvider{tx: tx}
	svc := NewRAGSyncService(db)

	err := svc.MarkSynced(context.Background(), []string{"1", "2"})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestMarkSyncedEmpty(t *testing.T) {
	db := &mockDBProvider{}
	svc := NewRAGSyncService(db)

	err := svc.MarkSynced(context.Background(), []string{})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestMarkSyncedTxError(t *testing.T) {
	db := &mockDBProvider{txErr: errors.New("tx error")}
	svc := NewRAGSyncService(db)

	err := svc.MarkSynced(context.Background(), []string{"1"})
	if err == nil {
		t.Error("expected error, got nil")
	}
}

func TestMarkSyncedExecError(t *testing.T) {
	tx := &mockTx{execErr: errors.New("exec error")}
	db := &mockDBProvider{tx: tx}
	svc := NewRAGSyncService(db)

	err := svc.MarkSynced(context.Background(), []string{"1"})
	if err == nil {
		t.Error("expected error, got nil")
	}
}

func TestMarkSyncedCommitError(t *testing.T) {
	tx := &mockTx{commitErr: errors.New("commit error")}
	db := &mockDBProvider{tx: tx}
	svc := NewRAGSyncService(db)

	err := svc.MarkSynced(context.Background(), []string{"1"})
	if err == nil {
		t.Error("expected error, got nil")
	}
}

func TestProcessIncomingSyncSuccess(t *testing.T) {
	tx := &mockTx{}
	db := &mockDBProvider{tx: tx}
	svc := NewRAGSyncService(db)

	err := svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{
		{ID: "1", Vector: []float32{1.0}},
	})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestProcessIncomingSyncEmpty(t *testing.T) {
	db := &mockDBProvider{}
	svc := NewRAGSyncService(db)

	err := svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestProcessIncomingSyncTxError(t *testing.T) {
	db := &mockDBProvider{txErr: errors.New("tx error")}
	svc := NewRAGSyncService(db)

	err := svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{{ID: "1"}})
	if err == nil {
		t.Error("expected error, got nil")
	}
}

func TestProcessIncomingSyncExecError(t *testing.T) {
	tx := &mockTx{execErr: errors.New("exec error")}
	db := &mockDBProvider{tx: tx}
	svc := NewRAGSyncService(db)

	err := svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{{ID: "1"}})
	if err == nil {
		t.Error("expected error, got nil")
	}
}
