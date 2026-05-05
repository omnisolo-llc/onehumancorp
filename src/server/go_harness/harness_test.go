package harness

import (
	"context"
	"os"
	"testing"
)

func TestAgentHarness(t *testing.T) {
	backend := NewDockerBackend()
	lifecycle := NewDefaultHarnessLifecycle(backend)
	bridge := NewFileSyncBridge()

	harness := NewAgentHarness(lifecycle, bridge)

	res, err := harness.Run(context.Background(), "agent-1", "echo test")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if res.Stdout != "Mock Docker Execution: echo test" {
		t.Errorf("expected Mock Docker Execution: echo test, got %s", res.Stdout)
	}

	err = harness.Sync(context.Background(), "/tmp/test", []byte("hello"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}

func TestDockerBackend(t *testing.T) {
	backend := NewDockerBackend()
	res, err := backend.ExecuteCommand(context.Background(), "echo test")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if res.Stdout != "Mock Docker Execution: echo test" {
		t.Errorf("expected Mock Docker Execution: echo test, got %s", res.Stdout)
	}
}

func TestLocalBackend(t *testing.T) {
	validator := NewASTValidator()
	backend := NewLocalBackend(validator)

	res, err := backend.ExecuteCommand(context.Background(), "echo test")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if res.Stdout != "test\n" {
		t.Errorf("expected test\\n, got %s", res.Stdout)
	}
}

func TestK8sBackend(t *testing.T) {
	backend := NewK8sBackend()

	os.Setenv("OHCMultitenant", "false")
	_, err := backend.ExecuteCommand(context.Background(), "echo test")
	if err == nil {
		t.Fatalf("expected error when disabled")
	}

	os.Setenv("OHCMultitenant", "true")
	res, err := backend.ExecuteCommand(context.Background(), "echo test")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if res.Stdout != "Mock K8s Execution: echo test" {
		t.Errorf("expected Mock K8s Execution: echo test, got %s", res.Stdout)
	}
}

func TestHarnessLifecycle(t *testing.T) {
	backend := NewDockerBackend()
	lifecycle := NewDefaultHarnessLifecycle(backend)

	res, err := lifecycle.RunAttempt(context.Background(), "agent-1", "echo test")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if res.Stdout != "Mock Docker Execution: echo test" {
		t.Errorf("expected Mock Docker Execution: echo test, got %s", res.Stdout)
	}
}

func TestFileSyncBridge(t *testing.T) {
	bridge := NewFileSyncBridge()
	err := bridge.SyncFile(context.Background(), "/tmp/test", []byte("hello"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	err = bridge.SyncFile(context.Background(), "/tmp/test", []byte("hello"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}
