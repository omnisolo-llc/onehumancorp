package mcp

import (
	"path/filepath"

	"os"
	"strings"
	"context"
	"github.com/onehumancorp/mono/src/server_old/telemetry"
	"encoding/json"
	"errors"
	"sync"
	"time"
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
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return nil, err
	}
	if telemetry.BufferMetricFunc != nil {
		_ = telemetry.BufferMetricFunc(ctx, "hybrid_ui_context", string(payloadBytes))
	}
	return FormatExecutionResult("hybrid_context", "success", payloadBytes, false), nil
}


type LocalFSSyncTool struct{}

// Execute performs local file system operations based on the given payload.
func (t *LocalFSSyncTool) Execute(ctx context.Context, payload map[string]interface{}) (*ExecutionResult, error) {
    action, _ := payload["Action"].(string)
    path, _ := payload["Path"].(string)

    cleanPath := filepath.Clean(path)
    if !strings.HasPrefix(cleanPath, ".agent-task/") || strings.Contains(cleanPath, "..") {
        return nil, errors.New("sandbox violation: path must start with .agent-task/")
    }

    payloadBytes, err := json.Marshal(payload)
    if err != nil {
        return nil, err
    }

    if telemetry.BufferMetricFunc != nil {
        _ = telemetry.BufferMetricFunc(ctx, "local_fs_sync", string(payloadBytes))
    }

    var resultData []byte

    switch action {
    case "read":
        data, err := os.ReadFile(cleanPath)
        if err != nil {
            return nil, err
        }
        resultData = data
    case "write":
        content, _ := payload["Content"].(string)
        err := os.WriteFile(cleanPath, []byte(content), 0644)
        if err != nil {
            return nil, err
        }
        resultData = []byte(`{"status":"written"}`)
    case "sync":
        _, err := os.Stat(cleanPath)
        if err != nil {
            return nil, err
        }
        resultData = []byte(`{"status":"synced"}`)
    default:
        return nil, errors.New("invalid action")
    }

    return FormatExecutionResult("local_fs_sync", "success", resultData, false), nil
}
