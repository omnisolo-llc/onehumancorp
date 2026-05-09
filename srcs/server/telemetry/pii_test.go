package telemetry

import (
	"testing"
)

func TestRedactInterfacePII(t *testing.T) {
	attrs := map[string]interface{}{
		"email": "test@example.com",
		"other": "value",
		"nested": map[string]interface{}{
			"password": "secret",
		},
		"tenant_id": "tenant-123",
		"array_of_evil": []interface{}{
			map[string]interface{}{"name": "John Doe", "email": "john@doe.com"},
		},
		"safe_field": "This should not be redacted",
		"raw_email":  "malicious@example.com",
	}
	redacted := RedactInterfacePII(attrs)

	if redacted["email"] != "[REDACTED]" {
		t.Errorf("Expected email to be redacted, got %v", redacted["email"])
	}
	if redacted["other"] != "value" {
		t.Errorf("Expected other to be value, got %v", redacted["other"])
	}

	nested := redacted["nested"].(map[string]interface{})
	if nested["password"] != "[REDACTED]" {
		t.Errorf("Expected nested password to be redacted, got %v", nested["password"])
	}

	if redacted["tenant_id"] != "[REDACTED]" {
		t.Errorf("Expected tenant_id to be redacted, got %v", redacted["tenant_id"])
	}

	if redacted["raw_email"] != "[EMAIL_REDACTED]" && redacted["raw_email"] != "[REDACTED]" {
		t.Errorf("Expected raw_email to be redacted, got %v", redacted["raw_email"])
	}

	arrayOfEvil := redacted["array_of_evil"].([]interface{})
	if len(arrayOfEvil) != 1 {
		t.Errorf("Expected array length 1, got %d", len(arrayOfEvil))
	}
	firstEvil := arrayOfEvil[0].(map[string]interface{})
	if firstEvil["name"] != "[REDACTED]" {
		t.Errorf("Expected name to be redacted, got %v", firstEvil["name"])
	}
}
