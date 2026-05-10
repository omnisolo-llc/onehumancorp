package telemetry

import "strings"

// RedactInterfacePII redacts PII from an interface map.
func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	if attrs == nil {
		return nil
	}
	redacted := make(map[string]interface{}, len(attrs))
	for k, v := range attrs {
		keyLower := strings.ToLower(k)
		if strings.Contains(keyLower, "password") ||
			strings.Contains(keyLower, "secret") ||
			strings.Contains(keyLower, "key") ||
			strings.Contains(keyLower, "token") ||
			strings.Contains(keyLower, "auth") ||
			strings.Contains(keyLower, "cookie") ||
			strings.Contains(keyLower, "credential") ||
			strings.Contains(keyLower, "email") ||
			strings.Contains(keyLower, "phone") ||
			strings.Contains(keyLower, "ssn") ||
			strings.Contains(keyLower, "address") ||
			strings.Contains(keyLower, "name") ||
			strings.Contains(keyLower, "pii") ||
			strings.Contains(keyLower, "tenant_id") ||
			strings.Contains(keyLower, "organization_id") ||
			strings.Contains(keyLower, "session_id") ||
			strings.Contains(keyLower, "payload") ||
			strings.Contains(keyLower, "credit") ||
			strings.Contains(keyLower, "card") ||
			strings.Contains(keyLower, "cvv") ||
			strings.Contains(keyLower, "dob") ||
			strings.Contains(keyLower, "birth") ||
			strings.Contains(keyLower, "passport") ||
			strings.Contains(keyLower, "bank") ||
			strings.Contains(keyLower, "account") ||
			strings.Contains(keyLower, "stripe") ||
			strings.Contains(keyLower, "billing") ||
			strings.Contains(keyLower, "ip_address") ||
			strings.Contains(keyLower, "mac_address") ||
			strings.Contains(keyLower, "geolocation") {
			redacted[k] = "[REDACTED]"
		} else {
			if nestedMap, ok := v.(map[string]interface{}); ok {
				redacted[k] = RedactInterfacePII(nestedMap)
			} else {
				redacted[k] = v
			}
		}
	}
	return redacted
}
