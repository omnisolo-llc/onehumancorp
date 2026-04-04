package utils

import (
	"bytes"
	"encoding/json"
	"strings"
)

// MinifyJSONString takes a string, checks if it is valid JSON, and returns
// the minified version of it (whitespace removed). If it's not valid JSON,
// it returns the original string.
func MinifyJSONString(input string) string {
	trimmed := strings.TrimSpace(input)
	if trimmed == "" {
		return input
	}

	// Quick check to see if it even looks like JSON
	if !(strings.HasPrefix(trimmed, "{") && strings.HasSuffix(trimmed, "}")) &&
		!(strings.HasPrefix(trimmed, "[") && strings.HasSuffix(trimmed, "]")) {
		return input
	}

	var buf bytes.Buffer
	err := json.Compact(&buf, []byte(trimmed))
	if err != nil {
		return input // Not valid JSON, return original
	}

	return buf.String()
}
