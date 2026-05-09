package telemetry

// RedactInterfacePII redacts PII from an interface map.
func RedactInterfacePII(attrs map[string]interface{}) map[string]interface{} {
	if attrs == nil {
		return nil
	}
	redacted, _ := RedactInterfacePIIRecursive(attrs).(map[string]interface{})
	return redacted
}
