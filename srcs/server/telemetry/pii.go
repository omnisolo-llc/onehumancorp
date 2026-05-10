package telemetry

import (
	"regexp"
)

var piiRegex = regexp.MustCompile(`\[PRIVATE:.*?\]`)

// RedactInterfacePII redacts PII data from interface{} fields, typically used before logging.
// It searches for strings matching the pattern [PRIVATE:data] and replaces them with [REDACTED].
func RedactInterfacePII(data interface{}) interface{} {
	switch v := data.(type) {
	case string:
		return piiRegex.ReplaceAllString(v, "[REDACTED]")
	case map[string]interface{}:
		redactedMap := make(map[string]interface{})
		for key, value := range v {
			redactedMap[key] = RedactInterfacePII(value)
		}
		return redactedMap
	case []interface{}:
		redactedSlice := make([]interface{}, len(v))
		for i, value := range v {
			redactedSlice[i] = RedactInterfacePII(value)
		}
		return redactedSlice
	default:
		return data
	}
}
