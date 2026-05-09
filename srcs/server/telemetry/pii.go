package telemetry

import (
	"os"
	"strings"
)

// RedactInterfacePII removes sensitive values from a map of attributes.
func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	redacted := make(map[string]interface{})
	for k, v := range attrs {
		if IsSensitiveKey(k) {
			redacted[k] = "[REDACTED]"
		} else {
			redacted[k] = v
		}
	}
	return redacted
}

// IsSensitiveKey returns true if the key name suggests it contains sensitive data.
func IsSensitiveKey(key string) bool {
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

// isTelemetryEnabled checks if telemetry is enabled.
func isTelemetryEnabled() bool {
	// In standalone mode, do not sync telemetry to cloud unless explicitly enabled
	isStandalone := os.Getenv("OHC_STANDALONE") == "true" || os.Getenv("STANDALONE_MODE") == "true"
	if isStandalone {
		return os.Getenv("OHC_TELEMETRY_ENABLED") == "true"
	}
	return true
}
