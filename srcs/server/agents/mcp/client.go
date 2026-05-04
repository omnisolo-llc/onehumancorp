package mcp

import (
	"fmt"
)

type TelemetryClient interface {
	BufferMetric(metricName string, metricType string, value float64, labels map[string]interface{}) error
}

type HybridContextTool struct {
	telemetry TelemetryClient
}

func NewHybridContextTool(telemetry TelemetryClient) *HybridContextTool {
	return &HybridContextTool{
		telemetry: telemetry,
	}
}

func (t *HybridContextTool) Execute(arguments map[string]interface{}) (map[string]interface{}, error) {
	metricName := "hybrid_action"
	if name, ok := arguments["metric_name"].(string); ok {
		metricName = name
	}

	metricType := "event"
	if typ, ok := arguments["metric_type"].(string); ok {
		metricType = typ
	}

	value := 1.0
	if val, ok := arguments["value"].(float64); ok {
		value = val
	}

	labels := make(map[string]interface{})
	if lbls, ok := arguments["labels"].(map[string]interface{}); ok {
		labels = lbls
	}

	err := t.telemetry.BufferMetric(metricName, metricType, value, labels)
	if err != nil {
		return nil, fmt.Errorf("failed to buffer metric: %w", err)
	}

	return map[string]interface{}{"status": "success"}, nil
}
