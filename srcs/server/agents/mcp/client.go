package mcp

import (
	"context"
	"encoding/json"
	"errors"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type ExecutionResult struct {
	ToolID           string          `json:"tool_id"`
	Status           string          `json:"status"`
	ResultData       json.RawMessage `json:"result_data"`
	HybridEscalation bool            `json:"hybrid_escalation"`
	Escalation       bool            `json:"escalation"`
	ExecutedAt       time.Time       `json:"executed_at"`
}

// Global registry for telemetry bridges
var (
	registeredTelemetryBridges = make(map[string]bool)
	bridgesMu                  sync.RWMutex
)

// Ensure proper formatting of execution results
func FormatExecutionResult(toolID string, status string, resultData []byte, escalation bool) *ExecutionResult {
	return &ExecutionResult{
		ToolID:           toolID,
		Status:           status,
		ResultData:       resultData,
		HybridEscalation: escalation,
		Escalation:       escalation,
		ExecutedAt:       time.Now().UTC(),
	}
}

// RegisterTelemetryMCPBridge registers the telemetry bridge for visual correlations.
func RegisterTelemetryMCPBridge(endpoint string) error {
	if endpoint == "" {
		return errors.New("endpoint cannot be empty")
	}
	bridgesMu.Lock()
	defer bridgesMu.Unlock()
	registeredTelemetryBridges[endpoint] = true
	return nil
}

// IsTelemetryMCPBridgeRegistered checks if a bridge is registered.
func IsTelemetryMCPBridgeRegistered(endpoint string) bool {
	bridgesMu.RLock()
	defer bridgesMu.RUnlock()
	return registeredTelemetryBridges[endpoint]
}

type HybridContextTool struct{}

func (t *HybridContextTool) Execute(ctx context.Context, payload map[string]interface{}) (*ExecutionResult, error) {
	if telemetry.BufferMetricFunc != nil {
		metricType := "hybrid_ui_context"
		if mt, ok := payload["metric_type"].(string); ok && mt != "" {
			metricType = mt
		}

		payloadBytes, err := json.Marshal(payload)
		if err != nil {
			return nil, errors.New("failed to marshal payload")
		}

		_ = telemetry.BufferMetricFunc(ctx, metricType, string(payloadBytes))
	}

	resultBytes, err := json.Marshal(map[string]string{"status": "recorded"})
	if err != nil {
		return nil, errors.New("failed to marshal result")
	}

	return FormatExecutionResult("hybrid_context", "success", resultBytes, true), nil
}
