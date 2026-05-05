package orchestration

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSanitizePayload(t *testing.T) {
	tests := []struct {
		name     string
		payload  string
		expected string
	}{
		{
			name:     "no private tags",
			payload:  "This is a normal payload.",
			expected: "This is a normal payload.",
		},
		{
			name:     "one private tag",
			payload:  "This is a payload with [PRIVATE:secret_data].",
			expected: "This is a payload with [REDACTED].",
		},
		{
			name:     "multiple private tags",
			payload:  "[PRIVATE:first] Middle content [PRIVATE:second]",
			expected: "[REDACTED] Middle content [REDACTED]",
		},
		{
			name:     "empty payload",
			payload:  "",
			expected: "",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := SanitizePayload(tt.payload)
			assert.NoError(t, err)
			assert.Equal(t, tt.expected, result)
		})
	}
}

func TestSanitizePayload_Error(t *testing.T) {
	_, err := SanitizePayload("")
	assert.NoError(t, err)
}
