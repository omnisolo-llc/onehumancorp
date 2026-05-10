package telemetry

import (
	"testing"
)

func TestRedactInterfacePII(t *testing.T) {
	attrs := map[string]interface{}{
		"email": "test@example.com",
		"other": "value",
		"ip_address": "127.0.0.1",
		"mac_address": "00:00:00:00:00:00",
		"geolocation": "0,0",
		"nested": map[string]interface{}{
			"password": "secret",
		},
	}
	redacted := RedactInterfacePII(attrs)

	if redacted["email"] != "[REDACTED]" {
		t.Errorf("Expected email to be redacted, got %v", redacted["email"])
	}
	if redacted["ip_address"] != "[REDACTED]" {
		t.Errorf("Expected ip_address to be redacted, got %v", redacted["ip_address"])
	}
	if redacted["mac_address"] != "[REDACTED]" {
		t.Errorf("Expected mac_address to be redacted, got %v", redacted["mac_address"])
	}
	if redacted["geolocation"] != "[REDACTED]" {
		t.Errorf("Expected geolocation to be redacted, got %v", redacted["geolocation"])
	}
	if redacted["other"] != "value" {
		t.Errorf("Expected other to be value, got %v", redacted["other"])
	}

	nested := redacted["nested"].(map[string]interface{})
	if nested["password"] != "[REDACTED]" {
		t.Errorf("Expected nested password to be redacted, got %v", nested["password"])
	}
}
