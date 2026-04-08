package telemetry

import (
	"bytes"
	"log/slog"
	"strings"
	"testing"
)

func TestPIIScrubberHandler_Message(t *testing.T) {
	var buf bytes.Buffer
	baseHandler := slog.NewJSONHandler(&buf, nil)
	scrubber := NewPIIScrubberHandler(baseHandler)
	logger := slog.New(scrubber)

	logger.Info("User logged in with email test@example.com and phone 123-456-7890")

	output := buf.String()
	if strings.Contains(output, "test@example.com") {
		t.Errorf("Expected email to be scrubbed, but got: %s", output)
	}
	if strings.Contains(output, "123-456-7890") {
		t.Errorf("Expected phone to be scrubbed, but got: %s", output)
	}
	if !strings.Contains(output, "[REDACTED_EMAIL]") {
		t.Errorf("Expected output to contain [REDACTED_EMAIL], but got: %s", output)
	}
	if !strings.Contains(output, "[REDACTED_PHONE]") {
		t.Errorf("Expected output to contain [REDACTED_PHONE], but got: %s", output)
	}
}

func TestPIIScrubberHandler_Attributes(t *testing.T) {
	var buf bytes.Buffer
	baseHandler := slog.NewJSONHandler(&buf, nil)
	scrubber := NewPIIScrubberHandler(baseHandler)
	logger := slog.New(scrubber)

	logger.Info("User registered", "email", "secret@example.com", "phone", "987-654-3210")

	output := buf.String()
	if strings.Contains(output, "secret@example.com") {
		t.Errorf("Expected email attribute to be scrubbed, but got: %s", output)
	}
	if strings.Contains(output, "987-654-3210") {
		t.Errorf("Expected phone attribute to be scrubbed, but got: %s", output)
	}
	if !strings.Contains(output, "[REDACTED_EMAIL]") {
		t.Errorf("Expected output to contain [REDACTED_EMAIL], but got: %s", output)
	}
	if !strings.Contains(output, "[REDACTED_PHONE]") {
		t.Errorf("Expected output to contain [REDACTED_PHONE], but got: %s", output)
	}
}

func TestPIIScrubberHandler_Group(t *testing.T) {
	var buf bytes.Buffer
	baseHandler := slog.NewJSONHandler(&buf, nil)
	scrubber := NewPIIScrubberHandler(baseHandler)
	logger := slog.New(scrubber).WithGroup("user")

	logger.Info("Details", "email", "group@example.com")

	output := buf.String()
	if strings.Contains(output, "group@example.com") {
		t.Errorf("Expected grouped email attribute to be scrubbed, but got: %s", output)
	}
	if !strings.Contains(output, "[REDACTED_EMAIL]") {
		t.Errorf("Expected output to contain [REDACTED_EMAIL], but got: %s", output)
	}
}

func TestPIIScrubberHandler_WithAttrs(t *testing.T) {
	var buf bytes.Buffer
	baseHandler := slog.NewJSONHandler(&buf, nil)
	scrubber := NewPIIScrubberHandler(baseHandler)
	logger := slog.New(scrubber).With("ssn", "123-45-6789")

	logger.Info("Processing")

	output := buf.String()
	if strings.Contains(output, "123-45-6789") {
		t.Errorf("Expected WithAttrs SSN attribute to be scrubbed, but got: %s", output)
	}
	if !strings.Contains(output, "[REDACTED_SSN]") {
		t.Errorf("Expected output to contain [REDACTED_SSN], but got: %s", output)
	}
}
