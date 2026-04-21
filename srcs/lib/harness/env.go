package harness

import "strings"

func ScrubEnv(env []string) []string {
	var scrubbed []string
	blockedPrefixes := []string{"OHC_API_KEY", "OTEL_"}
	for _, e := range env {
		blocked := false
		for _, prefix := range blockedPrefixes {
			if strings.HasPrefix(e, prefix) {
				blocked = true
				break
			}
		}
		if !blocked {
			scrubbed = append(scrubbed, e)
		}
	}
	return scrubbed
}
