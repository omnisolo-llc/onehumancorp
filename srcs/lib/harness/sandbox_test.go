package harness

import (
	"context"
	"strings"
	"testing"
	"os"
)

func TestHarnessManager(t *testing.T) {
	m := NewHarnessManager()
	wrapped := m.WrapCommand(context.Background(), "echo hello")
	if len(wrapped) == 0 || (!strings.Contains(wrapped[0], "bwrap") && !strings.Contains(wrapped[0], "sandbox-exec") && !strings.Contains(wrapped[0], "bash")) {
		t.Errorf("WrapCommand output incorrect: %v", wrapped)
	}

	m.WrapCommand(context.Background(), "echo <(ls)")
	v := m.GetViolations()
	if len(v) != 1 {
		t.Errorf("expected 1 violation, got %d", len(v))
	}
}

func TestHarnessManagerEnvScrubbing(t *testing.T) {
    m := NewHarnessManager()

    os.Setenv("OHC_API_KEY_TEST", "secret")
    os.Setenv("OTEL_TEST", "secret")

    stdout, _, err := m.Execute(context.Background(), "env")

    if err == nil {
        if strings.Contains(stdout, "OHC_API_KEY_TEST") || strings.Contains(stdout, "OTEL_TEST") {
            t.Errorf("Secrets leaked into subprocess env: %s", stdout)
        }
    }
}
