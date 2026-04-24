package telemetry_test

import (
	"bytes"
	"log/slog"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/src/server/telemetry"
)

func TestMultiTenantLoggingPIIRedaction(t *testing.T) {
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
		{
			name:     "Leakage Audit Guardrail 1",
			input:    "multi-tenant user data with PII like john.doe@acme.com in logs",
			expected: "multi-tenant user data with PII like [REDACTED_EMAIL] in logs",
		},
		{
			name:     "Leakage Audit Guardrail 2",
			input:    "Cloud DB query leaked phone number +1-800-555-0199",
			expected: "Cloud DB query leaked phone number [REDACTED_PHONE]",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var buf bytes.Buffer
			baseHandler := slog.NewJSONHandler(&buf, nil)
			handler := telemetry.NewPIIRedactingHandler(baseHandler)
			logger := slog.New(handler)

			logger.Info(tt.input)
			output := buf.String()

			if !strings.Contains(output, tt.expected) {
				t.Errorf("Expected output to contain %q, got %q", tt.expected, output)
			}

			// Optional: verify that the unredacted input is NOT in the output if it's supposed to be redacted
			if tt.input != tt.expected && strings.Contains(output, tt.input) {
				t.Errorf("Expected output to NOT contain unredacted input %q, got %q", tt.input, output)
			}
		})
	}
}
