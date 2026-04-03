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
			name:     "No tags",
			payload:  "This is a normal payload",
			expected: "This is a normal payload",
		},
		{
			name:     "One tag",
			payload:  "This is a [PRIVATE:secret123] payload",
			expected: "This is a [REDACTED] payload",
		},
		{
			name:     "Multiple tags",
			payload:  "[PRIVATE:first] This is a [PRIVATE:second] payload",
			expected: "[REDACTED] This is a [REDACTED] payload",
		},
		{
			name:     "Tag with space",
			payload:  "This is a [PRIVATE:secret 123] payload",
			expected: "This is a [REDACTED] payload",
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
