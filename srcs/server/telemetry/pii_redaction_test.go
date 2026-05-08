package telemetry

import (
	"reflect"
	"testing"
)

func TestRedactInterfacePII(t *testing.T) {
	input := map[string]interface{}{
		"user_id": "123",
		"email": "test@example.com",
		"nested": map[string]interface{}{
			"secret_key": "abc",
			"data": "safe",
		},
		"list": []interface{}{
			map[string]interface{}{"email": "test2@example.com"},
			"not-a-map",
		},
	}
	expected := map[string]interface{}{
		"user_id": "123",
		"email": "[REDACTED]",
		"nested": map[string]interface{}{
			"secret_key": "[REDACTED]",
			"data": "safe",
		},
		"list": []interface{}{
			map[string]interface{}{"email": "[REDACTED]"},
			"not-a-map",
		},
	}

	result := RedactInterfacePII(input)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %v, got %v", expected, result)
	}
}
