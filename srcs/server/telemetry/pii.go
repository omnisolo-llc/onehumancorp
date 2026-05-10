package telemetry

// RedactInterfacePII redacts PII from a map of interfaces.
func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	if attrs == nil {
		return nil
	}
	redacted := make(map[string]interface{})
	for k, v := range attrs {
		if k == "email" || k == "phone" || k == "password" || k == "ssn" {
			redacted[k] = "[REDACTED]"
		} else {
			redacted[k] = v
		}
	}
	return redacted
}
