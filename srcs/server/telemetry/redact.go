package telemetry

// RedactInterfacePII redacts PII from map payloads to ensure data privacy.
func RedactInterfacePII(payload map[string]interface{}) map[string]interface{} {
	if payload == nil {
		return nil
	}
	out := make(map[string]interface{})
	for k, v := range payload {
		// basic redaction of typical PII keys
		if k == "email" || k == "password" || k == "token" || k == "secret" || k == "ip_address" || k == "phone" {
			out[k] = "[REDACTED]"
			continue
		}

		switch typed := v.(type) {
		case map[string]interface{}:
			out[k] = RedactInterfacePII(typed)
		case []interface{}:
			var newSlice []interface{}
			for _, item := range typed {
				if m, ok := item.(map[string]interface{}); ok {
					newSlice = append(newSlice, RedactInterfacePII(m))
				} else {
					newSlice = append(newSlice, item)
				}
			}
			out[k] = newSlice
		default:
			out[k] = typed
		}
	}
	return out
}
