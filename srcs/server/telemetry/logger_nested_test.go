package telemetry

import (
	"bytes"
	"log/slog"
	"strings"
	"testing"
)

func TestPIIRedactingHandler_WithNestedGroup(t *testing.T) {
	var buf bytes.Buffer
	baseHandler := slog.NewJSONHandler(&buf, nil)
	handler := NewPIIRedactingHandler(baseHandler)
	logger := slog.New(handler)

	logger.Info("User info", slog.Group("user", slog.Group("details", slog.String("email", "test@example.com"), slog.String("phone", "555-123-4567"))))

	output := buf.String()
	if strings.Contains(output, "test@example.com") {
		t.Errorf("Expected email to be redacted inside nested group, got: %s", output)
	}
	if !strings.Contains(output, "[REDACTED_EMAIL]") {
		t.Errorf("Expected [REDACTED_EMAIL] in output, got: %s", output)
	}
}
