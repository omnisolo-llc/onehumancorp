package orchestration

import (
	"regexp"
)

var privateTagRegex = regexp.MustCompile(`\[PRIVATE:[^\]]*\]`)

// SanitizePayload strips out explicit private markers from the payload.
// For this MVP, it uses a mock regex replacing `[PRIVATE:*]` tags with `[REDACTED]`.
func SanitizePayload(payload string) (string, error) {
	sanitized := privateTagRegex.ReplaceAllString(payload, "[REDACTED]")
	return sanitized, nil
}
