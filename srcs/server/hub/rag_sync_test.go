package hub_test

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"database/sql"
	"go.opentelemetry.io/otel/metric"
)

// Minimal mock db.Provider to test SQL logic structure without a real DB
type mockProvider struct {
	db.Provider
	queryCalls int
	execCalls  int
	lastExec   string
}

// Mock Rows for FetchPendingSyncs
type mockRows struct {
	records []hub.RAGSyncRecord
	idx     int
}

func (m *mockRows) Next() bool {
	if m.idx < len(m.records) {
		m.idx++
		return true
	}
	return false
}

func (m *mockRows) Scan(dest ...interface{}) error {
	rec := m.records[m.idx-1]

	// Assuming dest pointers map exactly to what FetchPendingSyncs passes
	*dest[0].(*string) = rec.ID
	*dest[1].(*string) = rec.Context

	// Handle vector
	vStr := dest[2].(*sql.NullString)
	if len(rec.Vector) > 0 {
		b, _ := json.Marshal(rec.Vector)
		vStr.String = string(b)
		vStr.Valid = true
	} else {
		vStr.Valid = false
	}

	*dest[3].(*hub.SyncStatus) = rec.SyncStatus

	lastSync := dest[4].(*sql.NullTime)
	lastSync.Time = rec.LastSyncAt
	lastSync.Valid = !rec.LastSyncAt.IsZero()

	return nil
}

func (m *mockRows) Close() {}
func (m *mockRows) Columns() ([]string, error) { return nil, nil }
func (m *mockRows) Err() error { return nil }

func (m *mockProvider) Query(ctx context.Context, query string, args ...interface{}) (db.Rows, error) {
	m.queryCalls++
	return &mockRows{
		records: []hub.RAGSyncRecord{
			{ID: "1", Context: "test 1", Vector: []float32{1.0, 2.0}, SyncStatus: hub.SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: hub.SyncStatusError},
		},
		idx: 0,
	}, nil
}

func (m *mockProvider) Exec(ctx context.Context, query string, args ...interface{}) (int64, error) {
	m.execCalls++
	m.lastExec = query
	return 1, nil // mock success
}

func TestRAGSyncService_Implementation(t *testing.T) {
	// Initialize minimal telemetry dependencies so tests don't crash when counters increment
	// This uses the mock telemetry built in the project.
	telemetry.RAGRecordsSyncedTotal = &mockInt64Counter{}
	telemetry.RAGSyncErrorsTotal = &mockInt64Counter{}

	mockDB := &mockProvider{}
	svc := hub.NewRAGSyncService(mockDB)

	ctx := context.Background()

	// 1. Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if mockDB.queryCalls != 1 {
		t.Errorf("expected 1 query call, got %d", mockDB.queryCalls)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}
	if records[0].Vector[0] != 1.0 {
		t.Errorf("vector unmarshal failed, got %v", records[0].Vector)
	}

	// 2. Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}
	if mockDB.execCalls != 1 {
		t.Errorf("expected 1 exec call, got %d", mockDB.execCalls)
	}

	// 3. Test ProcessIncomingSync
	incoming := []hub.RAGSyncRecord{
		{ID: "3", Context: "incoming 1", Vector: []float32{3.0}},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}
	if mockDB.execCalls != 2 {
		t.Errorf("expected 2 total exec calls, got %d", mockDB.execCalls)
	}
}


type mockInt64Counter struct {
	metric.Int64Counter
}

func (m *mockInt64Counter) Add(ctx context.Context, incr int64, options ...metric.AddOption) {}
