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
}
