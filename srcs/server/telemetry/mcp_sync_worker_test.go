package telemetry

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"onehumancorp/srcs/server/db"
)

type MockRows struct {
	data [][]interface{}
	idx int
}

func (r *MockRows) Close() error { return nil }
func (r *MockRows) Next() bool {
	if r.idx < len(r.data) {
		r.idx++
		return true
	}
	return false
}
func (r *MockRows) Scan(dest ...interface{}) error {
	row := r.data[r.idx-1]
	*dest[0].(*int) = row[0].(int)
	*dest[1].(*string) = row[1].(string)
	*dest[2].(*float64) = row[2].(float64)
	*dest[3].(*string) = row[3].(string)
	*dest[4].(*string) = row[4].(string)
	return nil
}
func (r *MockRows) Err() error { return nil }


type MockProvider struct {
	Rows *MockRows
	QueryErr error
	ExecErr error
	ExecCalls []interface{}
}

func (m *MockProvider) Query(query string, args ...interface{}) (db.Rows, error) {
	if m.QueryErr != nil {
		return nil, m.QueryErr
	}
	return m.Rows, nil
}

func (m *MockProvider) Exec(query string, args ...interface{}) (db.Result, error) {
	if m.ExecErr != nil {
		return nil, m.ExecErr
	}
	for _, arg := range args {
		m.ExecCalls = append(m.ExecCalls, arg)
	}
	return nil, nil
}

func TestMcpSyncWorker_SyncMetrics(t *testing.T) {
	mockRows := &MockRows{
		data: [][]interface{}{
			{1, "test_metric", 1.0, "{}", "2024-05-04T00:00:00Z"},
			{2, "another_metric", 2.5, "{\"key\": \"value\"}", "2024-05-04T00:01:00Z"},
		},
	}

	provider := &MockProvider{
		Rows: mockRows,
	}

	mockServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer mockServer.Close()

	worker := NewMcpSyncWorker(provider, time.Second, mockServer.URL, mockServer.Client())

	worker.syncMetrics(context.Background())

	if len(provider.ExecCalls) != 2 {
		t.Fatalf("expected 2 exec calls, got %d", len(provider.ExecCalls))
	}

	if provider.ExecCalls[0] != 1 || provider.ExecCalls[1] != 2 {
		t.Fatalf("expected exec calls for IDs 1 and 2, got %v", provider.ExecCalls)
	}
}

func TestMcpSyncWorker_SyncMetrics_NetworkFailure(t *testing.T) {
	mockRows := &MockRows{
		data: [][]interface{}{
			{1, "test_metric", 1.0, "{}", "2024-05-04T00:00:00Z"},
		},
	}

	provider := &MockProvider{
		Rows: mockRows,
	}

	mockServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer mockServer.Close()

	worker := NewMcpSyncWorker(provider, time.Second, mockServer.URL, mockServer.Client())

	worker.syncMetrics(context.Background())

	if len(provider.ExecCalls) != 0 {
		t.Fatalf("expected 0 exec calls on network failure, got %d", len(provider.ExecCalls))
	}
}

func TestMcpSyncWorker_Start(t *testing.T) {
	provider := &MockProvider{
		Rows: &MockRows{data: [][]interface{}{}},
	}
	worker := NewMcpSyncWorker(provider, 10*time.Millisecond, "http://localhost", nil)
	ctx, cancel := context.WithCancel(context.Background())

	done := make(chan struct{})
	go func() {
		worker.Start(ctx)
		close(done)
	}()

	time.Sleep(25 * time.Millisecond)
	cancel()
	<-done
}
