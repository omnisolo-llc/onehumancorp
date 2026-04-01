package telemetry

import (
	"regexp"
)

var (
	emailRegex   = regexp.MustCompile(`[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}`)
	phoneRegex   = regexp.MustCompile(`\b\d{3}[-.]?\d{3}[-.]?\d{4}\b`)
	ssnRegex     = regexp.MustCompile(`\b\d{3}-\d{2}-\d{4}\b`)
)

func RedactPII(input string) string {
	s := emailRegex.ReplaceAllString(input, "[REDACTED_EMAIL]")
	s = phoneRegex.ReplaceAllString(s, "[REDACTED_PHONE]")
	s = ssnRegex.ReplaceAllString(s, "[REDACTED_SSN]")
	return s
}

func RedactInterfacePII(val interface{}) interface{} {
	switch v := val.(type) {
	case string:
		return RedactPII(v)
	case map[string]interface{}:
		for k, val := range v {
			v[k] = RedactInterfacePII(val)
		}
		return v
	case []interface{}:
		for i, val := range v {
			v[i] = RedactInterfacePII(val)
		}
		return v
	case []string:
		res := make([]string, len(v))
		for i, str := range v {
			res[i] = RedactPII(str)
		}
		return res
	case []map[string]interface{}:
		for i, m := range v {
			for k, val := range m {
				m[k] = RedactInterfacePII(val)
			}
			v[i] = m
		}
		return v
	default:
		return val
	}
}
