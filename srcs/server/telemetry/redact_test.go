package telemetry

import (
	"reflect"
	"testing"
)

func TestRedactInterfacePII(t *testing.T) {
	input := map[string]interface{}{
		"user_id": "12345",
		"email": "user@example.com",
		"nested": map[string]interface{}{
			"token": "secret_token_123",
			"safe_key": "safe_value",
		},
		"tags": []interface{}{"a", "b"},
		"secrets_list": []interface{}{
			map[string]interface{}{"password": "pwd"},
		},
	}

	expected := map[string]interface{}{
		"user_id": "12345",
		"email": "[REDACTED]",
		"nested": map[string]interface{}{
			"token": "[REDACTED]",
			"safe_key": "safe_value",
		},
		"tags": []interface{}{"a", "b"},
		"secrets_list": "[REDACTED]",
	}

	result := RedactInterfacePII(input)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %v, got %v", expected, result)
	}
}
