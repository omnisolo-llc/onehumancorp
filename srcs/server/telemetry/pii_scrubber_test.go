package telemetry

import (
	"bytes"
	"context"
	"log/slog"
	"strings"
	"testing"
)

func TestPIIScrubberHandler(t *testing.T) {
	var buf bytes.Buffer
	baseHandler := slog.NewTextHandler(&buf, nil)
	scrubberHandler := &PIIScrubberHandler{Handler: baseHandler}
	logger := slog.New(scrubberHandler)

	ctx := context.Background()

	// Test Message Redaction
	t.Run("Redact Message", func(t *testing.T) {
		buf.Reset()
		logger.InfoContext(ctx, "User email is leaking@example.com")
		output := buf.String()
		if strings.Contains(output, "leaking@example.com") {
			t.Errorf("Expected email to be redacted in message, got: %s", output)
		}
		if !strings.Contains(output, "[REDACTED_EMAIL]") {
			t.Errorf("Expected [REDACTED_EMAIL] in message, got: %s", output)
		}
	})

	// Test Attribute Redaction
	t.Run("Redact String Attribute", func(t *testing.T) {
		buf.Reset()
		logger.ErrorContext(ctx, "An error occurred", slog.String("contact", "phone 123-456-7890"))
		output := buf.String()
		if strings.Contains(output, "123-456-7890") {
			t.Errorf("Expected phone number to be redacted in attribute, got: %s", output)
		}
		if !strings.Contains(output, "[REDACTED_PHONE]") {
			t.Errorf("Expected [REDACTED_PHONE] in attribute, got: %s", output)
		}
	})

	// Test Any Value Map Redaction
	t.Run("Redact Any Attribute Map", func(t *testing.T) {
		buf.Reset()
		payload := map[string]interface{}{
			"ssn":   "123-45-6789",
			"email": "test@acme.com",
		}
		logger.WarnContext(ctx, "Suspicious payload", slog.Any("payload", payload))
		output := buf.String()
		if strings.Contains(output, "123-45-6789") || strings.Contains(output, "test@acme.com") {
			t.Errorf("Expected SSN and email to be redacted in Any attribute map, got: %s", output)
		}
		if !strings.Contains(output, "[REDACTED_SSN]") || !strings.Contains(output, "[REDACTED_EMAIL]") {
			t.Errorf("Expected [REDACTED_SSN] and [REDACTED_EMAIL] in attribute, got: %s", output)
		}
	})

	// Test WithAttrs Redaction
	t.Run("WithAttrs Redaction", func(t *testing.T) {
		buf.Reset()
		loggerWithAttrs := logger.With(slog.String("user", "bob@example.com"))
		loggerWithAttrs.InfoContext(ctx, "Login attempt")
		output := buf.String()
		if strings.Contains(output, "bob@example.com") {
			t.Errorf("Expected email to be redacted in WithAttrs, got: %s", output)
		}
		if !strings.Contains(output, "[REDACTED_EMAIL]") {
			t.Errorf("Expected [REDACTED_EMAIL] in WithAttrs, got: %s", output)
		}
	})

	// Test WithGroup Redaction
	t.Run("WithGroup Redaction", func(t *testing.T) {
		buf.Reset()
		loggerWithGroup := logger.WithGroup("test_group")
		loggerWithGroup.InfoContext(ctx, "System test", slog.String("contact", "info@example.com"))
		output := buf.String()
		if strings.Contains(output, "info@example.com") {
			t.Errorf("Expected email to be redacted in WithGroup, got: %s", output)
		}
		if !strings.Contains(output, "[REDACTED_EMAIL]") {
			t.Errorf("Expected [REDACTED_EMAIL] in WithGroup, got: %s", output)
		}
	})

	// Test Inline Group Redaction
	t.Run("Inline Group Redaction", func(t *testing.T) {
		buf.Reset()
		logger.InfoContext(ctx, "Group test", slog.Group("nested",
			slog.String("secret_email", "nested@example.com"),
			slog.Group("deep",
				slog.String("phone", "123-456-7890"),
			),
		))
		output := buf.String()
		if strings.Contains(output, "nested@example.com") {
			t.Errorf("Expected email to be redacted in inline group, got: %s", output)
		}
		if !strings.Contains(output, "[REDACTED_EMAIL]") {
			t.Errorf("Expected [REDACTED_EMAIL] in inline group, got: %s", output)
		}
		if strings.Contains(output, "123-456-7890") {
			t.Errorf("Expected phone to be redacted in deep group, got: %s", output)
		}
		if !strings.Contains(output, "[REDACTED_PHONE]") {
			t.Errorf("Expected [REDACTED_PHONE] in deep group, got: %s", output)
		}
	})
}
