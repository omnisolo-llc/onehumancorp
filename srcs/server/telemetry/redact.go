package telemetry

import "strings"

// RedactInterfacePII recursively redacts PII from maps and slices before serialization
func RedactInterfacePII(input map[string]interface{}) map[string]interface{} {
	if input == nil {
		return nil
	}
	output := make(map[string]interface{})
	for k, v := range input {
		output[k] = redactValue(k, v)
	}
	return output
}

func redactValue(key string, val interface{}) interface{} {
	kLower := strings.ToLower(key)

	if isPIIKey(kLower) {
		return "[REDACTED]"
	}

	switch v := val.(type) {
	case map[string]interface{}:
		return RedactInterfacePII(v)
	case []interface{}:
		var newSlice []interface{}
		for _, item := range v {
			newSlice = append(newSlice, redactValue("", item))
		}
		return newSlice
	default:
		return val
	}
}

func isPIIKey(key string) bool {
	piiKeys := []string{"email", "password", "token", "secret", "ssn", "credit_card", "phone", "address"}
	for _, pk := range piiKeys {
		if strings.Contains(key, pk) {
			return true
		}
	}
	return false
}
