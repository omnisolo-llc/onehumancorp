package telemetry

import (
	"reflect"
	"testing"
)

func TestRedactInterfacePII(t *testing.T) {
	tests := []struct {
		name     string
		input    map[string]interface{}
		expected map[string]interface{}
	}{
		{
			name:     "Nil input",
			input:    nil,
			expected: nil,
		},
		{
			name:     "Empty map",
			input:    make(map[string]interface{}),
			expected: make(map[string]interface{}),
		},
		{
			name: "Map without PII",
			input: map[string]interface{}{
				"service": "test",
				"count":   10,
			},
			expected: map[string]interface{}{
				"service": "test",
				"count":   10,
			},
		},
		{
			name: "Map with PII fields",
			input: map[string]interface{}{
				"email":    "user@example.com",
				"phone":    "123-456-7890",
				"password": "secret_password",
				"ssn":      "000-00-0000",
				"user_id":  "12345",
			},
			expected: map[string]interface{}{
				"email":    "[REDACTED]",
				"phone":    "[REDACTED]",
				"password": "[REDACTED]",
				"ssn":      "[REDACTED]",
				"user_id":  "12345",
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := RedactInterfacePII(tt.input)
			if !reflect.DeepEqual(got, tt.expected) {
				t.Errorf("RedactInterfacePII() = %v, want %v", got, tt.expected)
			}
		})
	}
}
