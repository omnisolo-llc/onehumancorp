package telemetry

// RedactInterfacePII redacts PII from an interface map recursively.
func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	if attrs == nil {
		return nil
	}

	redacted, ok := redactInterfacePIIHelper(attrs).(map[string]interface{})
	if !ok {
		return attrs
	}
	return redacted
}

// redactInterfacePIIHelper is a helper function to redact sensitive information recursively.
func redactInterfacePIIHelper(val interface{}) interface{} {
	switch v := val.(type) {
	case map[string]interface{}:
		newMap := make(map[string]interface{})
		for k, innerV := range v {
			if isSensitiveKey(k) {
				newMap[k] = "[REDACTED]"
			} else {
				newMap[k] = redactInterfacePIIHelper(innerV)
			}
		}
		return newMap
	case []interface{}:
		newArr := make([]interface{}, len(v))
		for i, innerV := range v {
			newArr[i] = redactInterfacePIIHelper(innerV)
		}
		return newArr
	case string:
		if isEmail(v) {
			return "[EMAIL_REDACTED]"
		}
		return v
	default:
		return v
	}
}
