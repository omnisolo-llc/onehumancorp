package telemetry

import (
	"context"
	"os"
)

var globalSyncEngine *TelemetrySyncEngine

// InitGlobalSyncEngine initializes the global telemetry sync engine.
// Should be called on application startup in standalone mode.
func InitGlobalSyncEngine(engine *TelemetrySyncEngine) {
	globalSyncEngine = engine
}

// redactInterfacePII is a helper function to redact sensitive information recursively.
func redactInterfacePII(val interface{}) interface{} {
	switch v := val.(type) {
	case map[string]interface{}:
		newMap := make(map[string]interface{})
		for k, innerV := range v {
			if isSensitiveKey(k) {
				newMap[k] = "[REDACTED]"
			} else {
				newMap[k] = redactInterfacePII(innerV)
			}
		}
		return newMap
	case []interface{}:
		newArr := make([]interface{}, len(v))
		for i, innerV := range v {
			newArr[i] = redactInterfacePII(innerV)
		}
		return newArr
	case string:
		if isEmail(v) {
			return "[EMAIL_REDACTED]"
		}
		return v
	default:
		return v
	}
}

// bufferMetricHelper is an internal helper to buffer if the engine is initialized and standalone is active
func bufferMetricHelper(ctx context.Context, name string, value float64, attrs map[string]interface{}) {
	if globalSyncEngine != nil && isTelemetryEnabled() {
		// Only buffer if it's explicitly enabled for standalone mode
		isStandalone := os.Getenv("OHC_STANDALONE") == "true" || os.Getenv("STANDALONE_MODE") == "true"
		if isStandalone {
			redactedAttrs, _ := redactInterfacePII(attrs).(map[string]interface{})
			_ = globalSyncEngine.BufferMetric(ctx, name, value, redactedAttrs)
		}
	}
}
