package orchestration

import (
	"regexp"
)

var privateTagRe = regexp.MustCompile(`\[PRIVATE:[^\]]*\]`)

// SanitizePayload strips out explicit private markers from the payload.
// For this MVP, it replaces [PRIVATE:*] tags with a redacted marker.
func SanitizePayload(payload string) (string, error) {
	sanitized := privateTagRe.ReplaceAllString(payload, "[REDACTED]")
	return sanitized, nil
}
