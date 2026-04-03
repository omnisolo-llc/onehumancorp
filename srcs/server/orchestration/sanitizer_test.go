package orchestration

import (
	"testing"
)

func TestSanitizePayload(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "No private markers",
			input:    "This is a normal string.",
			expected: "This is a normal string.",
		},
		{
			name:     "With private markers",
			input:    "This is [PRIVATE:secret data]a string.",
			expected: "This is a string.",
		},
		{
			name:     "With multiple private markers",
			input:    "[PRIVATE:start]Hello[PRIVATE:middle]World[PRIVATE:end]",
			expected: "HelloWorld",
		},
		{
			name:     "With PII data",
			input:    "My email is test@example.com.",
			expected: "My email is [REDACTED_EMAIL].",
		},
		{
			name:     "With both private markers and PII data",
			input:    "[PRIVATE:do not read] My email is secret@example.com and phone is 555-555-1234.",
			expected: " My email is [REDACTED_EMAIL] and phone is [REDACTED_PHONE].",
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			result, err := SanitizePayload(tc.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if result != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, result)
			}
		})
	}
}
