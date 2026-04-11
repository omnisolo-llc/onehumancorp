package hub

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockRows struct {
	records []RAGSyncRecord
	idx     int
}

func (m *mockRows) Next() bool {
	m.idx++
	return m.idx <= len(m.records)
}

func (m *mockRows) Scan(dest ...any) error {
	r := m.records[m.idx-1]
	*dest[0].(*string) = r.ID
	*dest[1].(*string) = r.Context
	var vBytes []byte
	if r.Vector != nil {
		vBytes, _ = json.Marshal(r.Vector)
	}
	*dest[2].(*[]byte) = vBytes
	status := string(r.SyncStatus)
	*dest[3].(**string) = &status
	t := r.LastSyncAt
	*dest[4].(**time.Time) = &t
	return nil
}

func (m *mockRows) Close()                      {}
func (m *mockRows) Columns() ([]string, error)  { return nil, nil }
func (m *mockRows) Err() error                  { return nil }

type mockProvider struct {
	records []RAGSyncRecord
	execs   int
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	m.execs++
	return 1, nil
}

func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return &mockRows{records: m.records, idx: 0}, nil
}

func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return nil
}

func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) { return nil, nil }
func (m *mockProvider) Close()                                   {}
func (m *mockProvider) IsSQLite() bool                           { return false }
func (m *mockProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) {
	return nil, nil
}

func TestDefaultRAGSyncService(t *testing.T) {
	mockDB := &mockProvider{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test1", Vector: []float32{1.1, 2.2}, SyncStatus: SyncStatusPending, LastSyncAt: time.Now()},
		},
	}
	service := &DefaultRAGSyncService{Provider: mockDB}

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if len(records[0].Vector) != 2 || records[0].Vector[0] != 1.1 {
		t.Fatalf("expected vector parsing to succeed, got %v", records[0].Vector)
	}

	// Test MarkSynced
	err = service.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if mockDB.execs != 1 {
		t.Fatalf("expected MarkSynced to execute 1 query")
	}

	// Test ProcessIncomingSync
	err = service.ProcessIncomingSync(context.Background(), []RAGSyncRecord{
		{ID: "2", Context: "test2", Vector: []float32{3.3}, SyncStatus: SyncStatusPending},
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if mockDB.execs != 2 {
		t.Fatalf("expected ProcessIncomingSync to execute 1 query")
	}
}
