package telemetry

import (
	"testing"
	"github.com/stretchr/testify/assert"
)

func TestRedactInterfacePII(t *testing.T) {
	input := map[string]interface{}{
		"normal_field": "value",
		"email":        "test@example.com",
		"password":     "secret123",
		"nested": map[string]interface{}{
			"token": "abc-123",
			"other": "safe",
		},
	}

	expected := map[string]interface{}{
		"normal_field": "value",
		"email":        "[REDACTED]",
		"password":     "[REDACTED]",
		"nested": map[string]interface{}{
			"token": "[REDACTED]",
			"other": "safe",
		},
	}

	result := RedactInterfacePII(input)
	assert.Equal(t, expected, result)
}
