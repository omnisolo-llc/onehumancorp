package telemetry

// RedactInterfacePII redacts PII from telemetry attributes
func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	redacted := make(map[string]interface{})
	for k, v := range attrs {
		if k == "email" || k == "password" || k == "phone" || k == "ssn" {
			redacted[k] = "[REDACTED]"
		} else {
			redacted[k] = v
		}
	}
	return redacted
}
