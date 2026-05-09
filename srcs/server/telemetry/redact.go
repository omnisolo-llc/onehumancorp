package telemetry

// RedactInterfacePII redacts PII from telemetry attributes
func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	if attrs == nil {
		return nil
	}
	redacted := make(map[string]interface{})
	for k, v := range attrs {
		if k == "email" || k == "password" || k == "phone" || k == "ssn" || k == "name" || k == "token" {
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
