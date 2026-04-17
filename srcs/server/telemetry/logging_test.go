package telemetry

import (
	"bytes"
	"context"
	"log/slog"
	"os"
	"strings"
	"testing"
)

func TestInitUnifiedLogging(t *testing.T) {
	// Capture stdout
	oldStdout := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	InitUnifiedLogging("test-mode")

	slog.Info("test message")

	w.Close()
	os.Stdout = oldStdout

	var buf bytes.Buffer
	buf.ReadFrom(r)
	out := buf.String()

	if !strings.Contains(out, `"msg":"test message"`) {
		t.Errorf("expected msg in log output, got %s", out)
	}
	if !strings.Contains(out, `"deployment_mode":"test-mode"`) {
		t.Errorf("expected deployment_mode in log output, got %s", out)
	}
}

func TestLogCloudEvent(t *testing.T) {
	// Capture stdout
	oldStdout := os.Stdout
	r, w, _ := os.Pipe()
	os.Stdout = w

	InitUnifiedLogging("test-mode")

	LogCloudEvent(context.Background(), "cloud-event", map[string]interface{}{"key": "value"})

	w.Close()
	os.Stdout = oldStdout

	var buf bytes.Buffer
	buf.ReadFrom(r)
	out := buf.String()

	if !strings.Contains(out, `"msg":"cloud-event"`) {
		t.Errorf("expected msg in log output, got %s", out)
	}
	if !strings.Contains(out, `"details":"{\"key\":\"value\"}"`) {
		t.Errorf("expected details in log output, got %s", out)
	}
}
