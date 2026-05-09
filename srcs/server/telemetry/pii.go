package telemetry

// RedactInterfacePII redacts PII from an interface map.
func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	if attrs == nil {
		return nil
	}
	redacted := make(map[string]interface{}, len(attrs))
	for k, v := range attrs {
		// Basic PII redaction logic
		if isSensitiveKey(k) {
			redacted[k] = "[REDACTED]"
		} else {
			if nestedMap, ok := v.(map[string]interface{}); ok {
				redacted[k] = RedactInterfacePII(nestedMap)
			} else if nestedArr, ok := v.([]interface{}); ok {
				newArr := make([]interface{}, len(nestedArr))
				for i, elem := range nestedArr {
					if elemMap, ok := elem.(map[string]interface{}); ok {
						newArr[i] = RedactInterfacePII(elemMap)
					} else if s, ok := elem.(string); ok && isEmail(s) {
						newArr[i] = "[EMAIL_REDACTED]"
					} else {
						newArr[i] = elem
					}
				}
				redacted[k] = newArr
			} else {
				if s, ok := v.(string); ok && isEmail(s) {
					redacted[k] = "[EMAIL_REDACTED]"
				} else {
					redacted[k] = v
				}
			}
		}
	}
	return redacted
}
