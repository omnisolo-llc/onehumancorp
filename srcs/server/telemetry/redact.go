package telemetry

import "strings"

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

// RedactInterfacePII redacts PII from an interface map.
func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	if attrs == nil {
		return nil
	}
	redacted := make(map[string]interface{}, len(attrs))
	for k, v := range attrs {
		// Basic PII redaction logic using shared guardrails
		if isSensitiveKey(k) {
			redacted[k] = "[REDACTED]"
		} else {
			// For nested maps, we could recurse
			if nestedMap, ok := v.(map[string]interface{}); ok {
				redacted[k] = RedactInterfacePII(nestedMap)
			} else {
				if strVal, isStr := v.(string); isStr && isEmail(strVal) {
					redacted[k] = "[EMAIL_REDACTED]"
				} else {
					redacted[k] = v
				}
			}
		}
	}
	return redacted
}
