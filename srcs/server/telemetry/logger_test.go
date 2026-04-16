package telemetry

import (
	"bytes"
	"log/slog"
	"strings"
	"testing"
)

func TestPIIRedactingHandler(t *testing.T) {
	var buf bytes.Buffer
	baseHandler := slog.NewJSONHandler(&buf, nil)
	handler := NewPIIRedactingHandler(baseHandler)
	logger := slog.New(handler)

	// Test message redaction
	logger.Info("User test@example.com logged in", "phone", "555-123-4567")

	output := buf.String()
	if strings.Contains(output, "test@example.com") {
		t.Errorf("Expected email to be redacted, got: %s", output)
	}
	if !strings.Contains(output, "[REDACTED_EMAIL]") {
		t.Errorf("Expected [REDACTED_EMAIL] in output, got: %s", output)
	}
	if strings.Contains(output, "555-123-4567") {
		t.Errorf("Expected phone to be redacted, got: %s", output)
	}
	if !strings.Contains(output, "[REDACTED_PHONE]") {
		t.Errorf("Expected [REDACTED_PHONE] in output, got: %s", output)
	}
}

func TestPIIRedactingHandler_WithAttrs(t *testing.T) {
	var buf bytes.Buffer
	baseHandler := slog.NewJSONHandler(&buf, nil)
	handler := NewPIIRedactingHandler(baseHandler)
	logger := slog.New(handler).With("secret", "sk-ant-api03-123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123")

	logger.Info("Some action")

	output := buf.String()
	if strings.Contains(output, "sk-ant-api03-123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123") {
		t.Errorf("Expected anthropic key to be redacted, got: %s", output)
	}
	if !strings.Contains(output, "[REDACTED_ANTHROPIC_KEY]") {
		t.Errorf("Expected [REDACTED_ANTHROPIC_KEY] in output, got: %s", output)
	}
}
