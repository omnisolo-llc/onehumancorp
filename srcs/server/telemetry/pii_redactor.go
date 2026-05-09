package telemetry

import (
	"strings"
)

// RedactInterfacePII redacts PII from map payloads, mirroring the Rust implementation.
func RedactInterfacePII(val interface{}) interface{} {
	return redactValue(val)
}

func redactValue(val interface{}) interface{} {
	switch v := val.(type) {
	case map[string]interface{}:
		newMap := make(map[string]interface{})
		for k, val := range v {
			if IsSensitiveKey(k) {
				newMap[k] = "[REDACTED]"
			} else {
				newMap[k] = redactValue(val)
			}
		}
		return newMap
	case []interface{}:
		newArr := make([]interface{}, len(v))
		for i, item := range v {
			newArr[i] = redactValue(item)
		}
		return newArr
	case string:
		if IsEmail(v) {
			return "[EMAIL_REDACTED]"
		}
		return v
	default:
		return v
	}
}

// IsSensitiveKey checks if a string key looks like it might contain sensitive data.
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

// IsEmail checks if a string contains '@' and '.', suggesting it might be an email.
func IsEmail(s string) bool {
	return strings.Contains(s, "@") && strings.Contains(s, ".")
}
