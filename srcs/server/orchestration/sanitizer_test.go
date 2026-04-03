package orchestration

import (
	"testing"
)

func TestSanitizePayload(t *testing.T) {
	tests := []struct {
		name     string
		payload  string
		expected string
	}{
		{
			name:     "No private tags",
			payload:  "This is a public payload",
			expected: "This is a public payload",
		},
		{
			name:     "Single private tag",
			payload:  "Here is some [PRIVATE:sensitive data] that is hidden.",
			expected: "Here is some [REDACTED] that is hidden.",
		},
		{
			name:     "Multiple private tags",
			payload:  "[PRIVATE:secret1] and [PRIVATE:secret2]",
			expected: "[REDACTED] and [REDACTED]",
		},
		{
			name:     "Empty private tag",
			payload:  "Empty [PRIVATE:] tag",
			expected: "Empty [REDACTED] tag",
		},
		{
			name:     "JSON payload with private tags",
			payload:  `{"task": "analyze", "data": "[PRIVATE:user_ssn]"}`,
			expected: `{"task": "analyze", "data": "[REDACTED]"}`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := SanitizePayload(tt.payload)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if result != tt.expected {
				t.Errorf("expected %q, got %q", tt.expected, result)
			}
		})
	}
}
