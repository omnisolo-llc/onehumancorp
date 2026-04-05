package telemetry

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// MockProvider implements db.Provider for testing
type MockProvider struct {
    db.Provider
    queryCount int
}

func (m *MockProvider) Query(ctx context.Context, query string, args ...interface{}) (db.Rows, error) {
    m.queryCount++
    return &MockRows{count: 0, limit: 1}, nil
}

func (m *MockProvider) Exec(ctx context.Context, query string, args ...interface{}) (int64, error) {
    return 1, nil
}

type MockRows struct {
    count int
    limit int
}

func (m *MockRows) Next() bool {
    if m.count < m.limit {
        m.count++
        return true
    }
    return false
}

func (m *MockRows) Scan(dest ...interface{}) error {
    if id, ok := dest[0].(*string); ok {
        *id = "test-id"
    }
    if mt, ok := dest[1].(*string); ok {
        *mt = "test-metric"
    }
    if pl, ok := dest[2].(*string); ok {
        *pl = "{}"
    }
    if ca, ok := dest[3].(*time.Time); ok {
        *ca = time.Now()
    }
    return nil
}

func (m *MockRows) Close() {}
func (m *MockRows) Err() error { return nil }
func (m *MockRows) Columns() ([]string, error) { return nil, nil }

func TestSyncDaemon(t *testing.T) {
    server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        if r.Header.Get("X-OHC-Conflict-Resolution") != "force-local" {
            t.Errorf("Missing required header")
        }
        body, _ := io.ReadAll(r.Body)
        var parsed []map[string]interface{}
        if err := json.Unmarshal(body, &parsed); err != nil {
            t.Errorf("Invalid JSON payload: %v", err)
        }
        w.WriteHeader(http.StatusOK)
    }))
    defer server.Close()

    provider := &MockProvider{}
    daemon := NewSyncDaemon(provider, server.URL)

    ctx, cancel := context.WithCancel(context.Background())
    defer cancel()

    // Mock the required global telemetry metrics before executing
	SyncLatency = nil
	SyncPayloadSize = nil
	syncDaemonBatchSize = nil

    // Test a single sync execution explicitly instead of daemon loop to prevent test hang
    // Run it in a goroutine that can be cancelled to avoid hanging just in case
    go daemon.syncMetrics(ctx)
    time.Sleep(100 * time.Millisecond) // Give it time to execute query and break

    if provider.queryCount == 0 {
        t.Errorf("Expected database to be queried")
    }
}
