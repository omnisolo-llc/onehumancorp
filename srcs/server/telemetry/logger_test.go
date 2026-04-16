package telemetry

import (
	"bytes"

	"log/slog"
	"testing"
)

func TestPIIRedactingHandler(t *testing.T) {
	var buf bytes.Buffer
	baseHandler := slog.NewTextHandler(&buf, nil)
	handler := NewPIIRedactingHandler(baseHandler)
	logger := slog.New(handler)

	logger.Info("User logged in with email info@example.com", "email", "info@example.com", "nested", map[string]interface{}{
		"phone": "999-888-7777",
	})

	output := buf.String()
	if bytes.Contains([]byte(output), []byte("info@example.com")) {
		t.Errorf("PII email leaked: %s", output)
	}
	if bytes.Contains([]byte(output), []byte("999-888-7777")) {
		t.Errorf("PII phone leaked: %s", output)
	}
	if !bytes.Contains([]byte(output), []byte("[REDACTED_EMAIL]")) {
		t.Errorf("Email not redacted correctly: %s", output)
	}
	if !bytes.Contains([]byte(output), []byte("[REDACTED_PHONE]")) {
		t.Errorf("Phone not redacted correctly: %s", output)
	}
}
