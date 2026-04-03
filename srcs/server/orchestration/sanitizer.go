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
