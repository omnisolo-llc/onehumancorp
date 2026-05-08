package telemetry

import (
	"reflect"
	"testing"
)

func TestRedactInterfacePII(t *testing.T) {
	input := map[string]interface{}{
		"email": "test@example.com",
		"other": "value",
		"nested": map[string]interface{}{
			"secret": "my-secret",
			"safe":   "data",
		},
		"list": []interface{}{
			map[string]interface{}{
				"token": "tok_123",
				"clean": 1,
			},
			"stringval",
		},
	}
	expected := map[string]interface{}{
		"email": "[REDACTED]",
		"other": "value",
		"nested": map[string]interface{}{
			"secret": "[REDACTED]",
			"safe":   "data",
		},
		"list": []interface{}{
			map[string]interface{}{
				"token": "[REDACTED]",
				"clean": 1,
			},
			"stringval",
		},
	}
	result := RedactInterfacePII(input)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %v, got %v", expected, result)
	}
}
