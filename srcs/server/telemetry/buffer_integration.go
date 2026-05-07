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

// bufferMetricHelper is an internal helper to buffer if the engine is initialized and standalone is active
func bufferMetricHelper(ctx context.Context, name string, value float64, attrs map[string]interface{}) {
	if globalSyncEngine != nil && isTelemetryEnabled() {
		// Only buffer if it's explicitly enabled for standalone mode
		isStandalone := os.Getenv("OHC_STANDALONE") == "true" || os.Getenv("STANDALONE_MODE") == "true"
		if isStandalone {
			_ = globalSyncEngine.BufferMetric(ctx, name, value, attrs)
		}
	}
}
