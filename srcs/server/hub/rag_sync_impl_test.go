package hub_test

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
)

type mockDBProvider struct {
	db.Provider
	execCount int
}

func (m *mockDBProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	m.execCount++
	return 1, nil
}

type mockRows struct {
	count int
}

func (m *mockRows) Next() bool {
	m.count--
	return m.count >= 0
}

func (m *mockRows) Scan(dest ...any) error {
	*dest[0].(*string) = "1"
	*dest[1].(*string) = "test context"
	*dest[2].(*string) = string(hub.SyncStatusPending)
	return nil
}

func (m *mockRows) Close() {}
func (m *mockRows) Columns() ([]string, error) { return nil, nil }
func (m *mockRows) Err() error { return nil }

func (m *mockDBProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return &mockRows{count: 1}, nil
}

func TestDefaultRAGSyncService_FetchPendingSyncs(t *testing.T) {
	mockDB := &mockDBProvider{}
	svc := hub.NewDefaultRAGSyncService(mockDB, nil, "")

	records, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" || records[0].Context != "test context" {
		t.Errorf("unexpected record values")
	}
}

func TestDefaultRAGSyncService_ProcessIncomingSync(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	mockDB := &mockDBProvider{}
	svc := hub.NewDefaultRAGSyncService(mockDB, ts.Client(), ts.URL)

	err := svc.ProcessIncomingSync(context.Background(), []hub.RAGSyncRecord{
		{ID: "1", Context: "test"},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mockDB.execCount != 1 {
		t.Errorf("expected MarkSynced to be called (Exec once), got %d", mockDB.execCount)
	}
}
