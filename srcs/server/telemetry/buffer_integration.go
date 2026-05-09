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

func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	redacted := make(map[string]interface{})
	for k, v := range attrs {
		keyLower := strings.ToLower(k)
		if strings.Contains(keyLower, "email") || strings.Contains(keyLower, "password") ||
		   strings.Contains(keyLower, "secret") || strings.Contains(keyLower, "token") ||
		   strings.Contains(keyLower, "key") {
			redacted[k] = "[REDACTED]"
		} else {
			redacted[k] = v
		}
	}
	return redacted
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
