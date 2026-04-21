package mcp

import (
	"context"
	"testing"
	"net/http"
	"net/http/httptest"

	"github.com/google/uuid"
    "github.com/onehumancorp/mono/srcs/server/db"
)

type mockRow struct {}
func (m *mockRow) Scan(dest ...any) error { return nil }

type mockRows struct {
    count int
}
func (m *mockRows) Close() {}
func (m *mockRows) Err() error { return nil }
func (m *mockRows) Next() bool {
    if m.count > 0 {
        m.count--
        return true
    }
    return false
}
func (m *mockRows) Scan(dest ...any) error {
    if len(dest) == 3 {
        if idPtr, ok := dest[0].(*string); ok {
            *idPtr = "123e4567-e89b-12d3-a456-426614174000"
        }
        if toolPtr, ok := dest[1].(*string); ok {
            *toolPtr = "test-tool"
        }
        if payloadPtr, ok := dest[2].(*string); ok {
            *payloadPtr = "{\"key\":\"value\"}"
        }
    }
    return nil
}
func (m *mockRows) Columns() ([]string, error) { return nil, nil }


type mockDBProvider struct {
    execCalls int
    queryCalls int
}

func (m *mockDBProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
    m.execCalls++
    return 1, nil
}
func (m *mockDBProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
    m.queryCalls++
    return &mockRows{count: 1}, nil
}
func (m *mockDBProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return &mockRow{} }
func (m *mockDBProvider) Begin(ctx context.Context) (db.Tx, error) { return nil, nil }
func (m *mockDBProvider) Close() {}
func (m *mockDBProvider) Ping(ctx context.Context) error { return nil }
func (m *mockDBProvider) IsSQLite() bool { return true }
func (m *mockDBProvider) AcquireTask(ctx context.Context, organizationID, agentID string) (*db.TaskRecord, error) { return nil, nil }


func TestMcpSyncProxy_BufferIntegrationState(t *testing.T) {
    ctx := context.Background()
    mockDB := &mockDBProvider{}
    proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:8080")

    payload := map[string]interface{}{"key": "value"}
    id, err := proxy.BufferIntegrationState(ctx, "test-tool", payload)

    if err != nil {
        t.Fatalf("Expected no error, got %v", err)
    }

    if id == "" {
        t.Errorf("Expected valid UUID, got empty string")
    }

    if _, err := uuid.Parse(id); err != nil {
        t.Errorf("Expected valid UUID, got %v", id)
    }

    if mockDB.execCalls != 1 {
        t.Errorf("Expected 1 Exec call, got %d", mockDB.execCalls)
    }
}

func TestMcpSyncProxy_SyncPendingStates(t *testing.T) {
    ctx := context.Background()
    mockDB := &mockDBProvider{}

    server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        w.WriteHeader(http.StatusOK)
    }))
    defer server.Close()

    proxy := NewMcpSyncProxy(mockDB, nil, server.URL)

    err := proxy.SyncPendingStates(ctx)

    if err != nil {
        t.Fatalf("Expected no error, got %v", err)
    }

    if mockDB.queryCalls != 1 {
        t.Errorf("Expected 1 Query call, got %d", mockDB.queryCalls)
    }

    if mockDB.execCalls != 1 {
        t.Errorf("Expected 1 Exec call, got %d", mockDB.execCalls)
    }
}
