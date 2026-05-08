package telemetry

// RedactInterfacePII redacts PII from an interface map.
func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	if attrs == nil {
		return nil
	}
	redacted := make(map[string]interface{}, len(attrs))
	for k, v := range attrs {
		// Basic PII redaction logic
		if k == "email" || k == "phone" || k == "name" || k == "password" || k == "token" {
			redacted[k] = "[REDACTED]"
		} else {
			// For nested maps, we could recurse, but for now just copy
			if nestedMap, ok := v.(map[string]interface{}); ok {
				redacted[k] = RedactInterfacePII(nestedMap)
			} else {
				redacted[k] = v
			}
		}
	}
	return redacted
}
