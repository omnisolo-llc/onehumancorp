package harness

import (
	"context"
	"errors"
	"strings"
	"testing"
)

type MockExecutor struct {
	ShouldError bool
	Env         []string
}

func (m *MockExecutor) Execute(ctx context.Context, cmd string, customEnv []string) (string, error) {
	m.Env = customEnv
	if m.ShouldError {
		return "", errors.New("mock execution error")
	}
	return "mock output", nil
}

func TestOrchestrator_SpawnSubAgent(t *testing.T) {
	executor := &MockExecutor{ShouldError: false}
	orchestrator := &Orchestrator{
		ProxyPort: 8080,
		Executor:  executor,
	}

	err := orchestrator.SpawnSubAgent(context.Background(), "echo test")
	if err != nil {
		t.Errorf("Expected nil error, got %v", err)
	}

	proxyEnv := "HTTP_PROXY=http://127.0.0.1:8080"
	found := false
	for _, e := range executor.Env {
		if e == proxyEnv {
			found = true
			break
		}
	}
	if !found {
		t.Errorf("Expected HTTP_PROXY to be injected into customEnv")
	}
}

func TestOrchestrator_SpawnSubAgent_Error(t *testing.T) {
	orchestrator := &Orchestrator{
		ProxyPort: 8080,
		Executor: &MockExecutor{ShouldError: true},
	}

	err := orchestrator.SpawnSubAgent(context.Background(), "echo test")
	if err == nil {
		t.Errorf("Expected error from mock executor, got nil")
	}
}

func TestOrchestrator_SpawnSubAgent_DefaultExecutor(t *testing.T) {
	orchestrator := &Orchestrator{
		ProxyPort: 8080,
		Executor: nil,
	}

	err := orchestrator.SpawnSubAgent(context.Background(), "echo test")
	if err != nil {
		if !strings.Contains(err.Error(), "failed to execute sub-agent") {
			t.Errorf("Expected execution error, got: %v", err)
		}
	}
}
