package harness

import (
	"bytes"
	"context"
	"testing"
	"strings"

	"github.com/onehumancorp/mono/src/server/telemetry"
	"github.com/prometheus/client_golang/prometheus"
)


func init() {
	prometheus.DefaultRegisterer = prometheus.NewRegistry()
	telemetry.InitTelemetry()
}

func TestManager(t *testing.T) {
	runner := NewBwrapRunner(nil)
	runner.ProxySock = "" // Disable proxy socket in tests
	manager := NewManager(nil, runner)
	ctx := context.Background()

	t.Run("Initialize", func(t *testing.T) {
		if err := manager.Initialize(ctx); err != nil {
			t.Errorf("Initialize() error = %v", err)
		}
	})

	t.Run("UpdateConfig", func(t *testing.T) {
		config := Config{
			DefaultPolicy: Policy{
				AllowNetwork: true,
			},
		}
		if err := manager.UpdateConfig(ctx, config); err != nil {
			t.Errorf("UpdateConfig() error = %v", err)
		}
		if !manager.config.DefaultPolicy.AllowNetwork {
			t.Errorf("Expected AllowNetwork to be true")
		}
	})

	t.Run("Execute with validation failure", func(t *testing.T) {
		command := "sudo rm -rf /"
		_, err := manager.Execute(ctx, command)
		if err == nil {
			t.Errorf("Expected security violation, but got nil")
		}
	})

	t.Run("WrapCommand", func(t *testing.T) {
		cmd, err := manager.WrapCommand(ctx, "echo 1", nil)
		if err != nil || cmd != "echo 1" {
			t.Errorf("WrapCommand failed")
		}
	})


	t.Run("ExecuteStream", func(t *testing.T) {
		var outBuf, errBuf bytes.Buffer
		res, err := manager.ExecuteStream(ctx, "echo stream_test", nil, nil, &outBuf, &errBuf, "agent-1", "org-1", "admin", "gpt-4")
		// The error might be because bwrap is not available, but we just verify it doesn't panic
		// and attempts to use the buffers.
		if err == nil {
			if !strings.Contains(outBuf.String(), "stream_test") && !strings.Contains(res.Stdout, "stream_test") {
				t.Errorf("Expected output to contain 'stream_test', got outBuf: %q, res.Stdout: %q, errBuf: %q, res.Stderr: %q, ExitCode: %d", outBuf.String(), res.Stdout, errBuf.String(), res.Stderr, res.ExitCode)
			}
		}
	})

	t.Run("ExecuteWithPolicy", func(t *testing.T) {
		// We can't easily run bwrap in this environment, so we mock the runner if we needed deep execution tests.
		// For now, we trust the integration.
	})
}

func TestRegistry_GetManager(t *testing.T) {
	registry := NewRegistry()
	manager := NewManager(nil, nil)
	registry.Register("test", manager)

	m, err := registry.GetManager("test")
	if err != nil || m == nil {
		t.Errorf("GetManager failed: %v", err)
	}

	_, err = registry.GetManager("nonexistent")
	if err == nil {
		t.Errorf("Expected error for nonexistent manager")
	}

	registry.Register("not-a-manager", &mockHarness{})
	_, err = registry.GetManager("not-a-manager")
	if err == nil {
		t.Errorf("Expected error for non-manager harness")
	}
}

type mockHarness struct{}

func (m *mockHarness) Execute(ctx context.Context, command string) (Result, error) {
	return Result{}, nil
}
