package telemetry

import (
	"os"
	"testing"
)

func TestComplianceGuardrails(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer func() {
		os.Unsetenv("OHC_STANDALONE")
		os.Unsetenv("OHC_TELEMETRY_ENABLED")
	}()

	attrs := map[string]interface{}{
		"tenant_id":       "123-abc",
		"organization_id": "org-456",
		"session_id":      "sess-789",
		"payload":         "sensitive data",
		"email":           "user@example.com",
		"normal_field":    "normal data",
		"nested": map[string]interface{}{
			"password": "secret_password",
			"info":     "normal info",
		},
	}

	redacted := RedactInterfacePII(attrs)

	if redacted["tenant_id"] != "[REDACTED]" {
		t.Errorf("tenant_id was not redacted, got: %v", redacted["tenant_id"])
	}
	if redacted["organization_id"] != "[REDACTED]" {
		t.Errorf("organization_id was not redacted, got: %v", redacted["organization_id"])
	}
	if redacted["session_id"] != "[REDACTED]" {
		t.Errorf("session_id was not redacted, got: %v", redacted["session_id"])
	}
	if redacted["payload"] != "[REDACTED]" {
		t.Errorf("payload was not redacted, got: %v", redacted["payload"])
	}
	if redacted["email"] != "[REDACTED]" {
		t.Errorf("email was not redacted, got: %v", redacted["email"])
	}
	if redacted["normal_field"] != "normal data" {
		t.Errorf("normal_field was modified, got: %v", redacted["normal_field"])
	}

	nested := redacted["nested"].(map[string]interface{})
	if nested["password"] != "[REDACTED]" {
		t.Errorf("nested password was not redacted, got: %v", nested["password"])
	}
	if nested["info"] != "normal info" {
		t.Errorf("nested info was modified, got: %v", nested["info"])
	}
}

func TestLocalSovereigntyTelemetry(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Unsetenv("OHC_TELEMETRY_ENABLED")

	if isTelemetryEnabled() {
		t.Errorf("Expected telemetry to be disabled by default in standalone mode to ensure local sovereignty.")
	}

	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	if !isTelemetryEnabled() {
		t.Errorf("Expected telemetry to be enabled when OHC_TELEMETRY_ENABLED=true in standalone mode.")
	}
	os.Unsetenv("OHC_STANDALONE")
	os.Unsetenv("OHC_TELEMETRY_ENABLED")
}
