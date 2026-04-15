package telemetry_test

import (
    "bytes"
    "context"
    "log/slog"
    "testing"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestPIIRedactingHandler(t *testing.T) {
    var buf bytes.Buffer
    baseHandler := slog.NewTextHandler(&buf, nil)
    redactingHandler := telemetry.NewPIIRedactingHandler(baseHandler)
    logger := slog.New(redactingHandler)

    ctx := context.Background()
    logger.InfoContext(ctx, "Test message contact test@example.com",
        "email", "user@example.com",
        "phone", "123-456-7890",
        "safe", "safe text",
        "group", slog.GroupValue(
            slog.String("nested_email", "admin@example.com"),
        ),
    )

    output := buf.String()

    if bytes.Contains(buf.Bytes(), []byte("test@example.com")) {
        t.Errorf("Expected message to be redacted, got %q", output)
    }
    if bytes.Contains(buf.Bytes(), []byte("user@example.com")) {
        t.Errorf("Expected email attr to be redacted, got %q", output)
    }
    if bytes.Contains(buf.Bytes(), []byte("123-456-7890")) {
        t.Errorf("Expected phone attr to be redacted, got %q", output)
    }
    if bytes.Contains(buf.Bytes(), []byte("admin@example.com")) {
        t.Errorf("Expected nested email attr to be redacted, got %q", output)
    }
    if !bytes.Contains(buf.Bytes(), []byte("safe text")) {
        t.Errorf("Expected safe text to be present, got %q", output)
    }
    if !bytes.Contains(buf.Bytes(), []byte("[REDACTED_EMAIL]")) {
        t.Errorf("Expected [REDACTED_EMAIL] in output, got %q", output)
    }
    if !bytes.Contains(buf.Bytes(), []byte("[REDACTED_PHONE]")) {
        t.Errorf("Expected [REDACTED_PHONE] in output, got %q", output)
    }
}
