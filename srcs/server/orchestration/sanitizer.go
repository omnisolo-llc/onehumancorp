package orchestration

import (
	"regexp"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

var privateRegex = regexp.MustCompile(`\[PRIVATE:.*?\]`)

// SanitizePayload strips out explicit private markers using a regex
// and applies standard PII redaction from the telemetry package.
func SanitizePayload(payload string) (string, error) {
	// First, remove MVP [PRIVATE:*] tags
	sanitized := privateRegex.ReplaceAllString(payload, "")

	// Second, apply standard PII redaction
	sanitized = telemetry.RedactPII(sanitized)

	return sanitized, nil
}

// SanitizePayloadMap recursively sanitizes a map interface
func SanitizePayloadMap(data interface{}) interface{} {
	switch v := data.(type) {
	case string:
		sanitized, _ := SanitizePayload(v)
		return sanitized
	case map[string]interface{}:
		res := make(map[string]interface{}, len(v))
		for key, val := range v {
			res[key] = SanitizePayloadMap(val)
		}
		return res
	case []interface{}:
		res := make([]interface{}, len(v))
		for i, val := range v {
			res[i] = SanitizePayloadMap(val)
		}
		return res
	default:
		return v
	}
}
