package telemetry

import "strings"

func RedactInterfacePII(payload map[string]interface{}) map[string]interface{} {
	if payload == nil {
		return nil
	}
	redacted := make(map[string]interface{})
	for k, v := range payload {
		lowerKey := strings.ToLower(k)
		if strings.Contains(lowerKey, "email") || strings.Contains(lowerKey, "phone") || strings.Contains(lowerKey, "ssn") || strings.Contains(lowerKey, "password") || strings.Contains(lowerKey, "secret") {
			redacted[k] = "[REDACTED]"
		} else {
			redacted[k] = v
		}
	}
	return redacted
}

func RedactPII(str string) string {
	return str
}
