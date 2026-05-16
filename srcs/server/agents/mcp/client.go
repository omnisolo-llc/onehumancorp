package mcp

import (
	"fmt"
	"strings"
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

func isSensitiveKeyLocal(key string) bool {
	k := strings.ToLower(key)
	return strings.Contains(k, "password") ||
		strings.Contains(k, "secret") ||
		strings.Contains(k, "key") ||
		strings.Contains(k, "token") ||
		strings.Contains(k, "auth") ||
		strings.Contains(k, "cookie") ||
		strings.Contains(k, "credential") ||
		strings.Contains(k, "email") ||
		strings.Contains(k, "phone") ||
		strings.Contains(k, "ssn") ||
		strings.Contains(k, "address") ||
		strings.Contains(k, "name") ||
		strings.Contains(k, "pii") ||
		strings.Contains(k, "tenant_id") ||
		strings.Contains(k, "organization_id") ||
		strings.Contains(k, "session_id") ||
		strings.Contains(k, "payload") ||
		strings.Contains(k, "credit") ||
		strings.Contains(k, "card") ||
		strings.Contains(k, "cvv") ||
		strings.Contains(k, "dob") ||
		strings.Contains(k, "birth") ||
		strings.Contains(k, "passport") ||
		strings.Contains(k, "bank") ||
		strings.Contains(k, "account") ||
		strings.Contains(k, "stripe") ||
		strings.Contains(k, "billing")
}

func isEmailLocal(s string) bool {
	return strings.Contains(s, "@") && strings.Contains(s, ".")
}

func redactInterfacePIILocal(val interface{}) interface{} {
	switch v := val.(type) {
	case map[string]interface{}:
		newMap := make(map[string]interface{})
		for k, innerV := range v {
			if isSensitiveKeyLocal(k) {
				newMap[k] = "[REDACTED]"
			} else {
				newMap[k] = redactInterfacePIILocal(innerV)
			}
		}
		return newMap
	case []interface{}:
		newArr := make([]interface{}, len(v))
		for i, innerV := range v {
			newArr[i] = redactInterfacePIILocal(innerV)
		}
		return newArr
	case string:
		if isEmailLocal(v) {
			return "[EMAIL_REDACTED]"
		}
		return v
	default:
		return v
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

	redactedLabels, _ := redactInterfacePIILocal(labels).(map[string]interface{})
	err := t.telemetry.BufferMetric(metricName, metricType, value, redactedLabels)
	if err != nil {
		return nil, fmt.Errorf("failed to buffer metric: %w", err)
	}

	return map[string]interface{}{"status": "success"}, nil
}
