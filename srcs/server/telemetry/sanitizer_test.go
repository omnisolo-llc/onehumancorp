package telemetry

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRedactInterfacePII(t *testing.T) {
	tests := []struct {
		name     string
		input    map[string]interface{}
		expected map[string]interface{}
	}{
		{
			name:     "nil input",
			input:    nil,
			expected: nil,
		},
		{
			name:     "no pii",
			input:    map[string]interface{}{"event": "login", "user_id": 123},
			expected: map[string]interface{}{"event": "login", "user_id": 123},
		},
		{
			name:     "contains pii keys",
			input:    map[string]interface{}{"email": "test@example.com", "PHONE_NUMBER": "12345", "status": "active"},
			expected: map[string]interface{}{"email": "[REDACTED]", "PHONE_NUMBER": "[REDACTED]", "status": "active"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := RedactInterfacePII(tt.input)
			assert.Equal(t, tt.expected, result)
		})
	}
}
