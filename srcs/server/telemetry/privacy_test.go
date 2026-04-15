package telemetry_test

import (
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"testing"
)

func TestRedactPII(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "Email",
			input:    "Contact me at test@example.com",
			expected: "Contact me at [REDACTED_EMAIL]",
		},
		{
			name:     "Phone",
			input:    "Call 123-456-7890",
			expected: "Call [REDACTED_PHONE]",
		},
		{
			name:     "SSN",
			input:    "My SSN is 123-45-6789",
			expected: "My SSN is [REDACTED_SSN]",
		},
		{
			name:     "Credit Card",
			input:    "My card is 1234-5678-9012-3456",
			expected: "My card is [REDACTED_CREDIT_CARD]",
		},
		{
			name:     "Credit Card with spaces",
			input:    "My card is 1234 5678 9012 3456",
			expected: "My card is [REDACTED_CREDIT_CARD]",
		},
		{
			name:     "OpenAI Key",
			input:    "Key: sk-123456789012345678901234567890123456789012345678",
			expected: "Key: [REDACTED_OPENAI_KEY]",
		},
		{
			name:     "Anthropic Key",
			input:    "Key: sk-ant-api03-12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345",
			expected: "Key: [REDACTED_ANTHROPIC_KEY]",
		},
		{
			name:     "AWS Access Key",
			input:    "AKIAABCDEF1234567890",
			expected: "[REDACTED_AWS_ACCESS_KEY]",
		},
		{
			name:     "AWS Secret Key",
			input:    "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
			expected: "[REDACTED_AWS_SECRET_KEY]",
		},
		{
			name:     "False Positive Avoidance",
			input:    "this_is_a_very_long_string_without_boundaries_that_should_not_be_redacted_at_all",
			expected: "this_is_a_very_long_string_without_boundaries_that_should_not_be_redacted_at_all",
		},
		{
			name:     "Timestamp Avoidance",
			input:    "The timestamp is 1715694200000",
			expected: "The timestamp is 1715694200000",
		},
		{
			name:     "AWS Secret with special chars",
			input:    "Key: wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKE+.",
			expected: "Key: [REDACTED_AWS_SECRET_KEY].",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := telemetry.RedactPII(tt.input)
			if got != tt.expected {
				t.Errorf("RedactPII(%q) = %q, want %q", tt.input, got, tt.expected)
			}
		})
	}
}
