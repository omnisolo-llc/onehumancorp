package mcp_audit_sync

import (
	"context"
	"encoding/json"
	"testing"
)

type MockDB struct{}

func (m *MockDB) Exec(ctx context.Context, sql string, arguments ...interface{}) (interface{}, error) {
	return nil, nil
}

type MockTelemetry struct{}

func (m *MockTelemetry) IncrementCounter(name string, val int, labels map[string]string) {}

func TestSyncAuditLogsToCloud(t *testing.T) {
	mockDB := &MockDB{}
	mockTele := &MockTelemetry{}
	tool := NewAuditSyncTool(mockDB, mockTele)

	payload := AuditSyncPayload{
		TenantID:  "tenant-1",
		AgentID:   "agent-1",
		Action:    "read",
		Resource:  "doc-1",
		Status:    "success",
		Metadata:  "{}",
		Timestamp: 1622548800,
	}
	payloadBytes, _ := json.Marshal(payload)

	err := tool.SyncAuditLogsToCloud(context.Background(), string(payloadBytes))
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestSyncAuditLogsToCloud_InvalidPayload(t *testing.T) {
	mockDB := &MockDB{}
	mockTele := &MockTelemetry{}
	tool := NewAuditSyncTool(mockDB, mockTele)

	err := tool.SyncAuditLogsToCloud(context.Background(), "invalid json")
	if err == nil {
		t.Errorf("Expected error for invalid json")
	}
}

func TestSyncAuditLogsToCloud_MissingFields(t *testing.T) {
	mockDB := &MockDB{}
	mockTele := &MockTelemetry{}
	tool := NewAuditSyncTool(mockDB, mockTele)

	payload := AuditSyncPayload{
		TenantID:  "", // Missing
		AgentID:   "agent-1",
		Action:    "read",
		Resource:  "doc-1",
		Status:    "success",
		Metadata:  "{}",
		Timestamp: 1622548800,
	}
	payloadBytes, _ := json.Marshal(payload)

	err := tool.SyncAuditLogsToCloud(context.Background(), string(payloadBytes))
	if err == nil {
		t.Errorf("Expected error for missing fields")
	}
}
