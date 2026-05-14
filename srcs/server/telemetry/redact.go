package telemetry

// RedactInterfacePII redacts PII from an interface map.
func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	if attrs == nil {
		return nil
	}
	redacted := make(map[string]interface{}, len(attrs))
	for k, v := range attrs {
		// Advanced PII redaction logic using our shared token list
		if isSensitiveKey(k) {
			redacted[k] = "[REDACTED]"
		} else {
			// Recurse for nested maps
			if nestedMap, ok := v.(map[string]interface{}); ok {
				redacted[k] = RedactInterfacePII(nestedMap)
			} else {
				redacted[k] = v
			}
		}
	}
	return redacted
}
