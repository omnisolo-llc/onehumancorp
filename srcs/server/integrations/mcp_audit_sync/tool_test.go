package mcp_audit_sync

import (
	"context"
	"testing"
	)

// MockDB is a simple mock database for testing.
type MockDB struct {
	ExecFunc func(ctx context.Context, sql string, arguments ...interface{}) (interface{}, error)
}

func (m *MockDB) Exec(ctx context.Context, sql string, arguments ...interface{}) (interface{}, error) {
	if m.ExecFunc != nil {
		return m.ExecFunc(ctx, sql, arguments...)
	}
	return nil, nil
}

func (m *MockDB) Query(ctx context.Context, sql string, args ...interface{}) (interface{}, error) {
	return nil, nil
}

func (m *MockDB) QueryRow(ctx context.Context, sql string, args ...interface{}) interface{} {
	return nil
}

func (m *MockDB) Begin(ctx context.Context) (interface{}, error) {
	return nil, nil
}

func (m *MockDB) Close() {}

// MockTelemetry is a simple mock telemetry for testing.
type MockTelemetry struct {
	IncrementCounterFunc func(name string, value int64, tags map[string]string)
}

func (m *MockTelemetry) IncrementCounter(name string, value int64, tags map[string]string) {
	if m.IncrementCounterFunc != nil {
		m.IncrementCounterFunc(name, value, tags)
	}
}
func (m *MockTelemetry) SetGauge(name string, value float64, tags map[string]string) {}
func (m *MockTelemetry) RecordHistogram(name string, value float64, tags map[string]string) {}

func TestSyncAuditLogsToCloud_Success(t *testing.T) {
	mockDB := &MockDB{
		ExecFunc: func(ctx context.Context, sql string, arguments ...interface{}) (interface{}, error) {
			return nil, nil
		},
	}

	telemetryCalled := false
	mockTele := &MockTelemetry{
		IncrementCounterFunc: func(name string, value int64, tags map[string]string) {
			telemetryCalled = true
		},
	}

	tool := NewAuditSyncTool(mockDB, mockTele)

	payload := `{"tenant_id": "t1", "agent_id": "a1", "action": "test", "resource": "res", "status": "ok", "metadata": "{}", "timestamp": 1234567890}`
	err := tool.SyncAuditLogsToCloud(context.Background(), payload)

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	if !telemetryCalled {
		t.Errorf("Expected telemetry to be called")
	}
}

func TestSyncAuditLogsToCloud_InvalidJSON(t *testing.T) {
	tool := NewAuditSyncTool(&MockDB{}, nil)

	payload := `{invalid_json}`
	err := tool.SyncAuditLogsToCloud(context.Background(), payload)

	if err == nil {
		t.Fatalf("Expected error for invalid JSON")
	}
}

func TestSyncAuditLogsToCloud_MissingFields(t *testing.T) {
	tool := NewAuditSyncTool(&MockDB{}, nil)

	payload := `{"tenant_id": "t1"}` // Missing other required fields
	err := tool.SyncAuditLogsToCloud(context.Background(), payload)

	if err == nil {
		t.Fatalf("Expected error for missing required fields")
	}
}
