package mcp

import (
	"errors"
	"testing"
)

type MockTelemetryClient struct {
	Metrics []MockMetric
	Err     error
}

type MockMetric struct {
	Name   string
	Type   string
	Value  float64
	Labels map[string]interface{}
}

func (m *MockTelemetryClient) BufferMetric(metricName string, metricType string, value float64, labels map[string]interface{}) error {
	if m.Err != nil {
		return m.Err
	}
	m.Metrics = append(m.Metrics, MockMetric{
		Name:   metricName,
		Type:   metricType,
		Value:  value,
		Labels: labels,
	})
	return nil
}

func TestHybridContextTool_Execute_Success(t *testing.T) {
	mockTelemetry := &MockTelemetryClient{}
	tool := NewHybridContextTool(mockTelemetry)

	args := map[string]interface{}{
		"metric_name": "test_metric",
		"metric_type": "gauge",
		"value":       42.0,
		"labels": map[string]interface{}{
			"env": "test",
		},
	}

	res, err := tool.Execute(args)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if status, ok := res["status"].(string); !ok || status != "success" {
		t.Fatalf("expected status 'success', got %v", res["status"])
	}

	if len(mockTelemetry.Metrics) != 1 {
		t.Fatalf("expected 1 metric, got %d", len(mockTelemetry.Metrics))
	}

	metric := mockTelemetry.Metrics[0]
	if metric.Name != "test_metric" {
		t.Errorf("expected metric name 'test_metric', got '%s'", metric.Name)
	}
	if metric.Type != "gauge" {
		t.Errorf("expected metric type 'gauge', got '%s'", metric.Type)
	}
	if metric.Value != 42.0 {
		t.Errorf("expected metric value 42.0, got %f", metric.Value)
	}
}

func TestHybridContextTool_Execute_Defaults(t *testing.T) {
	mockTelemetry := &MockTelemetryClient{}
	tool := NewHybridContextTool(mockTelemetry)

	args := map[string]interface{}{}

	_, err := tool.Execute(args)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(mockTelemetry.Metrics) != 1 {
		t.Fatalf("expected 1 metric, got %d", len(mockTelemetry.Metrics))
	}

	metric := mockTelemetry.Metrics[0]
	if metric.Name != "hybrid_action" {
		t.Errorf("expected metric name 'hybrid_action', got '%s'", metric.Name)
	}
	if metric.Type != "event" {
		t.Errorf("expected metric type 'event', got '%s'", metric.Type)
	}
	if metric.Value != 1.0 {
		t.Errorf("expected metric value 1.0, got %f", metric.Value)
	}
}

func TestHybridContextTool_Execute_Error(t *testing.T) {
	mockTelemetry := &MockTelemetryClient{
		Err: errors.New("telemetry error"),
	}
	tool := NewHybridContextTool(mockTelemetry)

	args := map[string]interface{}{}

	_, err := tool.Execute(args)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}

	if err.Error() != "failed to buffer metric: telemetry error" {
		t.Errorf("unexpected error message: %v", err)
	}
}
