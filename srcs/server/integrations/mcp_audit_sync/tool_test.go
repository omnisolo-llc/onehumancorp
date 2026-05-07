package mcp_audit_sync

import (
	"context"
	"encoding/json"
	"testing"
)

type MockDB struct {
	ExecFunc func(ctx context.Context, sql string, arguments ...interface{}) (interface{}, error)
}

func (m *MockDB) Query(ctx context.Context, sql string, arguments ...interface{}) (interface{}, error) {
	return nil, nil
}

func (m *MockDB) QueryRow(ctx context.Context, sql string, arguments ...interface{}) interface{} {
	return nil
}

func (m *MockDB) Exec(ctx context.Context, sql string, arguments ...interface{}) (interface{}, error) {
	if m.ExecFunc != nil {
		return m.ExecFunc(ctx, sql, arguments...)
	}
	return nil, nil
}

func (m *MockDB) Close() {}

// MockTelemetry is a simple mock telemetry for testing.
type MockTelemetry struct {
	IncrementCounterFunc func(name string, value float64, tags map[string]string)
}

func (m *MockTelemetry) IncrementCounter(name string, value float64, tags map[string]string) {
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
		IncrementCounterFunc: func(name string, value float64, tags map[string]string) {
			telemetryCalled = true
		},
	}

	tool := NewAuditSyncTool(mockDB, mockTele)

	payload := AuditSyncPayload{
		TenantID:  "tenant-1",
		AgentID:   "agent-1",
		Action:    "LOGIN",
		Resource:  "system",
		Status:    "SUCCESS",
		Metadata:  "{}",
		Timestamp: 1672531200,
	}
	payloadBytes, _ := json.Marshal(payload)

	err := tool.SyncAuditLogsToCloud(context.Background(), string(payloadBytes))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if !telemetryCalled {
		t.Fatalf("expected telemetry to be called")
	}
}

func TestSyncAuditLogsToCloud_InvalidPayload(t *testing.T) {
	tool := NewAuditSyncTool(&MockDB{}, &MockTelemetry{})

	err := tool.SyncAuditLogsToCloud(context.Background(), "invalid json")
	if err == nil {
		t.Fatalf("expected error for invalid json, got nil")
	}

	err = tool.SyncAuditLogsToCloud(context.Background(), "{}")
	if err == nil {
		t.Fatalf("expected error for missing fields, got nil")
	}
}
