package orchestration

import (
	"regexp"
)

var (
	privateRe = regexp.MustCompile(`\[PRIVATE:.*?\]`)
	emailRe   = regexp.MustCompile(`[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}`)
	ccRe      = regexp.MustCompile(`\b(?:\d[ -]*?){13,16}\b`)
)

// SanitizePayload removes explicit private markers from the payload.
// For this MVP, it uses a mock regex replacing [PRIVATE:*] tags with [REDACTED].
var SanitizePayloadFunc = func(payload string) (string, error) {
	sanitized := privateRe.ReplaceAllString(payload, "[REDACTED]")
	sanitized = emailRe.ReplaceAllString(sanitized, "[EMAIL_REDACTED]")
	sanitized = ccRe.ReplaceAllString(sanitized, "[CC_REDACTED]")

	return sanitized, nil
}

func SanitizePayload(payload string) (string, error) {
	return SanitizePayloadFunc(payload)
}
