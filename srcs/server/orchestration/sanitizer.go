package orchestration

import (
	"regexp"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

var privateTagRegex = regexp.MustCompile(`\[PRIVATE:.*?\]`)

// SanitizePayload strips out explicit private markers like [PRIVATE:*] tags
// and delegates to telemetry.RedactPII for general PII redaction.
func SanitizePayload(payload string) (string, error) {
	sanitized := privateTagRegex.ReplaceAllString(payload, "[REDACTED]")
	return telemetry.RedactPII(sanitized), nil
}
