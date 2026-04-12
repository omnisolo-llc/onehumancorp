package telemetry

import (
	"context"
	"encoding/json"
	"testing"
)

func TestNewMetricsAndBuffering(t *testing.T) {
	// Setup mock meter to initialize metrics
	mockM := &mockMeter{}
	err := InitWithMeter(mockM)
	if err != nil {
		t.Fatalf("InitWithMeter failed: %v", err)
	}

	// Mock BufferMetricFunc
	var bufferedMetrics = make(map[string]string)
	BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		bufferedMetrics[metricType] = payload
		return nil
	}
	defer func() { BufferMetricFunc = nil }()

	ctx := context.Background()

	t.Run("RecordSyncDaemonBatchSize", func(t *testing.T) {
		RecordSyncDaemonBatchSize(ctx, 42)
		payload, ok := bufferedMetrics["sync_daemon_batch_size"]
		if !ok {
			t.Fatal("expected sync_daemon_batch_size to be buffered")
		}
		var data map[string]interface{}
		if err := json.Unmarshal([]byte(payload), &data); err != nil {
			t.Fatalf("failed to unmarshal payload: %v", err)
		}
		if data["batch_size"].(float64) != 42 {
			t.Errorf("expected batch_size 42, got %v", data["batch_size"])
		}
	})

	t.Run("RecordAgentExecutionTrace", func(t *testing.T) {
		RecordAgentExecutionTrace(ctx, "agent-1", "role-1", "api-1", "event-1")
		payload, ok := bufferedMetrics["agent_execution_trace"]
		if !ok {
			t.Fatal("expected agent_execution_trace to be buffered")
		}
		var data map[string]interface{}
		if err := json.Unmarshal([]byte(payload), &data); err != nil {
			t.Fatalf("failed to unmarshal payload: %v", err)
		}
		if data["agent_id"] != "agent-1" || data["role"] != "role-1" || data["api"] != "api-1" || data["event_type"] != "event-1" {
			t.Errorf("unexpected payload data: %v", data)
		}
	})

	t.Run("LogAgentExecution calls RecordAgentExecutionTrace", func(t *testing.T) {
		// Clear previously buffered metric
		delete(bufferedMetrics, "agent_execution_trace")

		LogAgentExecution(ctx, "agent-2", "role-2", "api-2", "event-2", `{"key": "value"}`)

		payload, ok := bufferedMetrics["agent_execution_trace"]
		if !ok {
			t.Fatal("expected agent_execution_trace to be buffered via LogAgentExecution")
		}
		var data map[string]interface{}
		if err := json.Unmarshal([]byte(payload), &data); err != nil {
			t.Fatalf("failed to unmarshal payload: %v", err)
		}
		if data["agent_id"] != "agent-2" {
			t.Errorf("expected agent_id agent-2, got %v", data["agent_id"])
		}
	})
}
