package telemetry

import "strings"

func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	redacted := make(map[string]interface{})
	for k, v := range attrs {
		if str, ok := v.(string); ok && strings.Contains(str, "[PRIVATE:PII]") {
			redacted[k] = "[REDACTED]"
		} else {
			redacted[k] = v
		}
	}
	return redacted
}
