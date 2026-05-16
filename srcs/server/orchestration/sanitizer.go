package orchestration

import (
	"regexp"
)

// SanitizePayload removes explicit private markers from the payload.
// For this MVP, it uses a mock regex replacing [PRIVATE:*] tags with [REDACTED].
var SanitizePayloadFunc = func(payload string) (string, error) {
	re := regexp.MustCompile(`\[PRIVATE:.*?\]`)
	sanitized := re.ReplaceAllString(payload, "[REDACTED]")
	return sanitized, nil
}

func SanitizePayload(payload string) (string, error) {
	return SanitizePayloadFunc(payload)
}
