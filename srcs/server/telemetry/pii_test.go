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
			input:    "User data: [PRIVATE:user@example.com]",
			expected: "User data: [REDACTED]",
		},
		{
			name:     "string without PII",
			input:    "No PII here",
			expected: "No PII here",
		},
		{
			name: "map with PII",
			input: map[string]interface{}{
				"email": "Contact [PRIVATE:user@example.com]",
				"name":  "Alice",
			},
			expected: map[string]interface{}{
				"email": "Contact [REDACTED]",
				"name":  "Alice",
			},
		},
		{
			name: "nested structure",
			input: map[string]interface{}{
				"users": []interface{}{
					map[string]interface{}{"email": "[PRIVATE:a@example.com]"},
					map[string]interface{}{"email": "b@example.com"},
				},
			},
			expected: map[string]interface{}{
				"users": []interface{}{
					map[string]interface{}{"email": "[REDACTED]"},
					map[string]interface{}{"email": "b@example.com"},
				},
			},
		},
		{
			name:     "integer",
			input:    42,
			expected: 42,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := RedactInterfacePII(tt.input)
			if !reflect.DeepEqual(result, tt.expected) {
				t.Errorf("expected %v, got %v", tt.expected, result)
			}
		})
	}
}
