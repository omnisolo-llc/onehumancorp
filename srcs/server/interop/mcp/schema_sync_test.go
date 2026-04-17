package mcp

import (
	"context"
	"errors"
	"github.com/onehumancorp/mono/srcs/server/db"
	"net/http"
	"net/http/httptest"
	"testing"
)

// A mocked DB provider that can optionally simulate errors
type mockErrorDBProvider struct {
	mockDBProvider
	failExec  bool
	failQuery bool
}

func (m *mockErrorDBProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if m.failExec {
		return 0, errors.New("simulated exec error")
	}
	return m.mockDBProvider.Exec(ctx, sql, arguments...)
}

func (m *mockErrorDBProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	if m.failQuery {
		return nil, errors.New("simulated query error")
	}
	return m.mockDBProvider.Query(ctx, sql, optionsAndArgs...)
}

func TestSchemaSyncTool_Execute(t *testing.T) {
	ctx := context.Background()
	mockDB := &mockDBProvider{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
	tool := NewSchemaSyncTool(proxy)

	// Test invalid direction
	err := tool.Execute(ctx, []string{"v1"}, "invalid")
	if err == nil {
		t.Errorf("Expected error for invalid direction, got nil")
	}

	// Test valid push
	err = tool.Execute(ctx, []string{"v1"}, "push")
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Test valid pull
	err = tool.Execute(ctx, []string{"v1"}, "pull")
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if mockDB.execCalls != 4 {
		t.Errorf("Expected 4 Exec calls, got %d", mockDB.execCalls)
	}
	if mockDB.queryCalls != 2 {
		t.Errorf("Expected 2 Query call, got %d", mockDB.queryCalls)
	}
}

func TestSchemaSyncTool_Execute_Errors(t *testing.T) {
	ctx := context.Background()

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	t.Run("BufferIntegrationState Error", func(t *testing.T) {
		mockDB := &mockErrorDBProvider{failExec: true}
		proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
		tool := NewSchemaSyncTool(proxy)

		err := tool.Execute(ctx, []string{"v1"}, "push")
		if err == nil {
			t.Errorf("Expected error due to buffer failure, got nil")
		}
	})

	t.Run("SyncPendingStates Error", func(t *testing.T) {
		// Mock exec succeeds for buffering, but query fails for syncing
		mockDB := &mockErrorDBProvider{failQuery: true}
		proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
		tool := NewSchemaSyncTool(proxy)

		err := tool.Execute(ctx, []string{"v1"}, "push")
		if err == nil {
			t.Errorf("Expected error due to sync failure, got nil")
		}
	})
}
