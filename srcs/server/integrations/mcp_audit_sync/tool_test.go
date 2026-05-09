package mcp_audit_sync

import (
	"context"
	"testing"
	"github.com/DATA-DOG/go-sqlmock"
)

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
    db, mock, err := sqlmock.New()
    if err != nil {
        t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
    }
    defer db.Close()

	telemetryCalled := false
	mockTele := &MockTelemetry{
		IncrementCounterFunc: func(name string, value int64, tags map[string]string) {
			telemetryCalled = true
		},
	}

	tool := NewAuditSyncTool(db, mockTele)

    mock.ExpectExec("INSERT INTO mcp_audit_sync_log").WillReturnResult(sqlmock.NewResult(1, 1))

	payload := `{"tenant_id": "t1", "agent_id": "a1", "action": "test", "resource": "res", "status": "ok", "metadata": "{}", "timestamp": 1234567890}`
	err = tool.SyncAuditLogsToCloud(context.Background(), payload)

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	if !telemetryCalled {
		t.Errorf("Expected telemetry to be called")
	}
}

func TestSyncAuditLogsToCloud_InvalidJSON(t *testing.T) {
	tool := NewAuditSyncTool(nil, nil)

	payload := `{invalid_json}`
	err := tool.SyncAuditLogsToCloud(context.Background(), payload)

	if err == nil {
		t.Fatalf("Expected error for invalid JSON")
	}
}

func TestSyncAuditLogsToCloud_MissingFields(t *testing.T) {
	tool := NewAuditSyncTool(nil, nil)

	payload := `{"tenant_id": "t1"}` // Missing other required fields
	err := tool.SyncAuditLogsToCloud(context.Background(), payload)

	if err == nil {
		t.Fatalf("Expected error for missing required fields")
	}
}
