package telemetry

import (
	"reflect"
	"testing"
)

func TestRedactInterfacePII(t *testing.T) {
	tests := []struct {
		name     string
		input    interface{}
		expected interface{}
	}{
		{
			name:     "string with PII",
			input:    "Contact me at user@example.com or 123-456-7890. SSN: 123-45-6789.",
			expected: "Contact me at [REDACTED_EMAIL] or [REDACTED_PHONE]. SSN: [REDACTED_SSN].",
		},
		{
			name: "map with PII",
			input: map[string]interface{}{
				"email": "user@test.com",
				"data": map[string]interface{}{
					"phone": "987-654-3210",
				},
			},
			expected: map[string]interface{}{
				"email": "[REDACTED_EMAIL]",
				"data": map[string]interface{}{
					"phone": "[REDACTED_PHONE]",
				},
			},
		},
		{
			name: "slice with PII",
			input: []interface{}{
				"test@email.com",
				map[string]interface{}{"ssn": "999-99-9999"},
			},
			expected: []interface{}{
				"[REDACTED_EMAIL]",
				map[string]interface{}{"ssn": "[REDACTED_SSN]"},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := RedactInterfacePII(tt.input)
			if !reflect.DeepEqual(result, tt.expected) {
				t.Errorf("RedactInterfacePII() = %v, want %v", result, tt.expected)
			}
		})
	}
}
