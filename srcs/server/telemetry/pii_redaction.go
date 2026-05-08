package telemetry

import "strings"

// RedactInterfacePII redacts sensitive PII fields from a map before marshaling to JSON.
func RedactInterfacePII(payload map[string]interface{}) map[string]interface{} {
	if payload == nil {
		return nil
	}

	redacted := make(map[string]interface{})
	for k, v := range payload {
		keyLower := strings.ToLower(k)
		if strings.Contains(keyLower, "email") ||
		   strings.Contains(keyLower, "password") ||
		   strings.Contains(keyLower, "token") ||
		   strings.Contains(keyLower, "secret") ||
		   strings.Contains(keyLower, "key") ||
		   strings.Contains(keyLower, "ssn") ||
		   strings.Contains(keyLower, "credit_card") ||
		   strings.Contains(keyLower, "pii") {
			redacted[k] = "[REDACTED]"
		} else {
			// recursively redact if it's a map
			if subMap, ok := v.(map[string]interface{}); ok {
				redacted[k] = RedactInterfacePII(subMap)
			} else if subSlice, ok := v.([]interface{}); ok {
				var redactedSlice []interface{}
				for _, item := range subSlice {
					if itemMap, ok := item.(map[string]interface{}); ok {
						item = RedactInterfacePII(itemMap)
					}
					redactedSlice = append(redactedSlice, item)
				}
				redacted[k] = redactedSlice
			} else {
				redacted[k] = v
			}
		}
	}
	return redacted
}
