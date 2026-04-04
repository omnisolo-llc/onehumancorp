package utils

import (
	"bytes"
	"encoding/json"
	"strings"
)

// MinifyJSONString takes a string, checks if it is potentially valid JSON (starts with { or [),
// and if it is, attempts to minify it. If it fails or is not JSON, it returns the original string.
// This saves LLM tokens when passing embedded JSON structures.
func MinifyJSONString(input string) string {
	trimmed := strings.TrimSpace(input)
	if len(trimmed) == 0 {
		return input
	}
	if (strings.HasPrefix(trimmed, "{") && strings.HasSuffix(trimmed, "}")) ||
		(strings.HasPrefix(trimmed, "[") && strings.HasSuffix(trimmed, "]")) {
		var minified bytes.Buffer
		if err := json.Compact(&minified, []byte(trimmed)); err == nil {
			return minified.String()
		}
	}
	return input
}
