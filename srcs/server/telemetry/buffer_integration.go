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

// isSensitiveKey returns true if a key name matches any known PII/sensitive tokens.
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
		strings.Contains(k, "billing") ||
		strings.Contains(k, "ip_address") ||
		strings.Contains(k, "mac_address") ||
		strings.Contains(k, "geolocation")
}

// isEmail checks for the basic structure of an email in a string value.
func isEmail(s string) bool {
	return strings.Contains(s, "@") && strings.Contains(s, ".")
}

// bufferMetricHelper is an internal helper to buffer if the engine is initialized and standalone is active
func bufferMetricHelper(ctx context.Context, name string, value float64, attrs map[string]interface{}) {
	if globalSyncEngine != nil && isTelemetryEnabled() {
		// Only buffer if it's explicitly enabled for standalone mode
		isStandalone := os.Getenv("OHC_STANDALONE") == "true" || os.Getenv("STANDALONE_MODE") == "true"
		if isStandalone {
			redactedAttrs := RedactInterfacePII(attrs)
			_ = globalSyncEngine.BufferMetric(ctx, name, value, redactedAttrs)
		}
	}
}
