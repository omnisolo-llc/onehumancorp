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

// SanitizePayloadMap recursively sanitizes a map interface.
// It removes [PRIVATE] tags, applies PII redaction to strings,
// and explicitly deletes sensitive keys like `rag_context`.
func SanitizePayloadMap(data interface{}) interface{} {
	switch v := data.(type) {
	case string:
		sanitized, _ := SanitizePayload(v)
		return sanitized
	case map[string]interface{}:
		res := make(map[string]interface{}, len(v))
		for key, val := range v {
			if key == "rag_context" {
				continue
			}
			res[key] = SanitizePayloadMap(val)
		}
		return res
	case []interface{}:
		res := make([]interface{}, len(v))
		for i, val := range v {
			res[i] = SanitizePayloadMap(val)
		}
		return res
	case []string:
		res := make([]string, len(v))
		for i, val := range v {
			sanitized, _ := SanitizePayload(val)
			res[i] = sanitized
		}
		return res
	case []map[string]interface{}:
		res := make([]map[string]interface{}, len(v))
		for i, val := range v {
			res[i] = SanitizePayloadMap(val).(map[string]interface{})
		}
		return res
	default:
		// Attempt deep scrubbing for complex types by delegating to telemetry.RedactInterfacePII.
		// We first try our custom recursive logic above to ensure rag_context and [PRIVATE] tags
		// are handled correctly in maps/slices.
		return telemetry.RedactInterfacePII(v)
	}
}
