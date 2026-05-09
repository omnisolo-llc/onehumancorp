package telemetry

import (
	"context"
	"os"
	"strings"
)

var globalSyncEngine *TelemetrySyncEngine

// InitGlobalSyncEngine initializes the global telemetry sync engine.
// Should be called on application startup in standalone mode.
func InitGlobalSyncEngine(engine *TelemetrySyncEngine) {
	globalSyncEngine = engine
}

// bufferMetricHelper is an internal helper to buffer if the engine is initialized and standalone is active
func bufferMetricHelper(ctx context.Context, name string, value float64, attrs map[string]interface{}) {
	if globalSyncEngine != nil && isTelemetryEnabled() {
		redactedAttrs := RedactInterfacePII(attrs)
		// Only buffer if it's explicitly enabled for standalone mode
		isStandalone := os.Getenv("OHC_STANDALONE") == "true" || os.Getenv("STANDALONE_MODE") == "true"
		if isStandalone {
			_ = globalSyncEngine.BufferMetric(ctx, name, value, redactedAttrs)
		}
	}
}

// RedactInterfacePII redacts PII from interface maps
func RedactInterfacePII(val interface{}) map[string]interface{} {
	switch v := val.(type) {
	case map[string]interface{}:
		newMap := make(map[string]interface{})
		for k, val := range v {
			if isSensitiveKey(k) {
				newMap[k] = "[REDACTED]"
			} else {
				newMap[k] = val // Simplified for this fix
			}
		}
		return newMap
	}
	return map[string]interface{}{}
}

func isSensitiveKey(key string) bool {
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
		strings.Contains(k, "payload")
}
