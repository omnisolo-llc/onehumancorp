package proxy

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"os"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockRows struct {
	items []map[string]interface{}
	idx   int
}

func (m *mockRows) Next() bool {
	if m.idx < len(m.items) {
		m.idx++
		return true
	}
	return false
}

func (m *mockRows) Scan(dest ...any) error {
	item := m.items[m.idx-1]
	id := dest[0].(*string)
	tool := dest[1].(*string)
	args := dest[2].(*string)

	*id = item["id"].(string)
	*tool = item["tool_name"].(string)
	*args = item["arguments"].(string)
	return nil
}

func (m *mockRows) Close() {}
func (m *mockRows) Columns() ([]string, error) { return nil, nil }
func (m *mockRows) Err() error { return nil }


type mockDBProvider struct {
	db.Provider
	execCount int
	queryRows *mockRows
	queryErr  error
	execErr   error
}

func (m *mockDBProvider) IsSQLite() bool {
	return true
}

func (m *mockDBProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	m.execCount++
	return 1, m.execErr
}

func (m *mockDBProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	if m.queryErr != nil {
		return nil, m.queryErr
	}
	return m.queryRows, nil
}

func TestMcpSyncProxy_Buffer(t *testing.T) {
	mockDB := &mockDBProvider{}
	proxy := NewMcpSyncProxy(mockDB, "http://dummy")

	id, err := proxy.Buffer(context.Background(), "test-tool", map[string]interface{}{"key": "value"})
	if err != nil {
		t.Fatalf("Buffer failed: %v", err)
	}
	if id == "" {
		t.Errorf("Expected non-empty ID")
	}
	if mockDB.execCount != 1 {
		t.Errorf("Expected 1 Exec call, got %d", mockDB.execCount)
	}
}

func TestMcpSyncProxy_Sync(t *testing.T) {
	os.Setenv("SPIFFE_IDENTITY_TOKEN", "fake-token")
	defer os.Unsetenv("SPIFFE_IDENTITY_TOKEN")

	var receivedToken string
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		receivedToken = r.Header.Get("Authorization")
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	argsBytes, _ := json.Marshal(map[string]interface{}{"key": "value"})
	mockRows := &mockRows{
		items: []map[string]interface{}{
			{"id": "1", "tool_name": "test-tool", "arguments": string(argsBytes)},
			{"id": "2", "tool_name": "test-tool-2", "arguments": string(argsBytes)},
		},
	}
	mockDB := &mockDBProvider{queryRows: mockRows}

	proxy := NewMcpSyncProxy(mockDB, ts.URL)
	count, err := proxy.Sync(context.Background())
	if err != nil {
		t.Fatalf("Sync failed: %v", err)
	}

	if count != 2 {
		t.Errorf("Expected 2 items synced, got %d", count)
	}

	if receivedToken != "Bearer fake-token" {
		t.Errorf("Expected SPIFFE token to be sent, got %s", receivedToken)
	}
}
