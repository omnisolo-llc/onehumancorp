package mcp

import (
	"errors"
	"os"
	"testing"
)

// MockOrchestrator is a mock implementation of the Orchestrator interface.
type MockOrchestrator struct {
	registerToolFunc func(toolName, endpoint string) error
	registeredTools  map[string]string
}

func (m *MockOrchestrator) RegisterTool(toolName, endpoint string) error {
	if m.registerToolFunc != nil {
		return m.registerToolFunc(toolName, endpoint)
	}
	if m.registeredTools == nil {
		m.registeredTools = make(map[string]string)
	}
	m.registeredTools[toolName] = endpoint
	return nil
}

func TestTelemetryMCPBridge_Register_CloudMode(t *testing.T) {
	// Ensure OHC_STANDALONE is not "true"
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	mockOrchestrator := &MockOrchestrator{}
	bridge := NewTelemetryMCPBridge(mockOrchestrator)

	err := bridge.Register()
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	endpoint, ok := mockOrchestrator.registeredTools["telemetry-mcp-bridge"]
	if !ok {
		t.Fatalf("tool was not registered")
	}
	if endpoint != "http://telemetry-mcp-bridge:9090" {
		t.Errorf("expected cloud endpoint, got %s", endpoint)
	}
}

func TestTelemetryMCPBridge_Register_StandaloneMode(t *testing.T) {
	// Set OHC_STANDALONE to "true"
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	mockOrchestrator := &MockOrchestrator{}
	bridge := NewTelemetryMCPBridge(mockOrchestrator)

	err := bridge.Register()
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	endpoint, ok := mockOrchestrator.registeredTools["telemetry-mcp-bridge"]
	if !ok {
		t.Fatalf("tool was not registered")
	}
	if endpoint != "local://telemetry-bridge" {
		t.Errorf("expected standalone endpoint, got %s", endpoint)
	}
}

func TestTelemetryMCPBridge_Register_NilOrchestrator(t *testing.T) {
	bridge := NewTelemetryMCPBridge(nil)

	err := bridge.Register()
	if err == nil {
		t.Fatalf("expected an error when orchestrator is nil")
	}
	if err.Error() != "orchestrator cannot be nil" {
		t.Errorf("unexpected error message: %s", err.Error())
	}
}

func TestTelemetryMCPBridge_Register_OrchestratorError(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")
	expectedErr := errors.New("registration failed")

	mockOrchestrator := &MockOrchestrator{
		registerToolFunc: func(toolName, endpoint string) error {
			return expectedErr
		},
	}
	bridge := NewTelemetryMCPBridge(mockOrchestrator)

	err := bridge.Register()
	if err == nil {
		t.Fatalf("expected error from orchestrator to be returned")
	}
}
