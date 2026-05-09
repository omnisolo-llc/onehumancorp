package telemetry

import "strings"

// RedactInterfacePII securely redacts specific PII keys from telemetry attributes
// before they are buffered or logged.
func RedactInterfacePII(input map[string]interface{}) map[string]interface{} {
	if input == nil {
		return nil
	}

	redacted := make(map[string]interface{})

	piiKeys := map[string]bool{
		"email": true,
		"phone": true,
		"password": true,
		"secret": true,
		"token": true,
		"ssn": true,
		"credit_card": true,
		"dob": true,
	}

	for k, v := range input {
		keyLower := strings.ToLower(k)
		isPII := false
		for piiKey := range piiKeys {
			if strings.Contains(keyLower, piiKey) {
				isPII = true
				break
			}
		}

		if isPII {
			redacted[k] = "[REDACTED]"
		} else {
			redacted[k] = v
		}
	}

	return redacted
}
