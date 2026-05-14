package harness

import (
	"strings"
	"testing"
)

type MockHarness struct{}

func (m *MockHarness) RunAttempt(cmd string) (*AttemptResult, error) {
	return &AttemptResult{
		Stdout:   "mock stdout",
		ExitCode: 0,
	}, nil
}

func (m *MockHarness) Compact() error {
	return nil
}

func (m *MockHarness) Reset() error {
	return nil
}

func TestMockHarness(t *testing.T) {
	var harness AgentHarness = &MockHarness{}
	res, err := harness.RunAttempt("ls")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if res.Stdout != "mock stdout" {
		t.Errorf("expected 'mock stdout', got %s", res.Stdout)
	}
}

func TestNewAssistantAgentHarness(t *testing.T) {
	// Empty config
	harness, err := NewAssistantAgentHarness("")
	if err != nil {
		t.Fatalf("expected no error for empty policy, got %v", err)
	}
	if harness == nil {
		t.Fatalf("expected harness, got nil")
	}

	// Valid config
	validPolicy := `{"disabled_commands": ["rm"]}`
	harness, err = NewAssistantAgentHarness(validPolicy)
	if err != nil {
		t.Fatalf("expected no error for valid policy, got %v", err)
	}
	if harness == nil {
		t.Fatalf("expected harness, got nil")
	}

	// Invalid config
	invalidPolicy := `{"disabled_commands": ["rm"`
	harness, err = NewAssistantAgentHarness(invalidPolicy)
	if err == nil {
		t.Fatalf("expected error for invalid policy, got nil")
	}
	if harness != nil {
		t.Fatalf("expected no harness, got %v", harness)
	}
}

func TestAssistantAgentHarness_RunAttempt(t *testing.T) {
	harness, err := NewAssistantAgentHarness("")
	if err != nil {
		t.Fatalf("failed to create harness: %v", err)
	}

	res, err := harness.RunAttempt("echo hello")
	if err != nil {
		t.Fatalf("RunAttempt failed: %v", err)
	}

	// if bwrap isn't installed locally, res will return failure, so we should only
	// test that res is returned.
	if res == nil {
		t.Fatalf("expected res, got nil")
	}
}

func TestAssistantAgentHarness_RunAttempt_PolicyDenied(t *testing.T) {
	policy := `{"disabled_commands": ["rm"]}`
	harness, err := NewAssistantAgentHarness(policy)
	if err != nil {
		t.Fatalf("failed to create harness: %v", err)
	}

	res, err := harness.RunAttempt("rm -rf /")
	if err == nil {
		t.Fatalf("expected error for denied command, got nil")
	}
	if res != nil {
		t.Errorf("expected nil result, got %+v", res)
	}

	if !strings.Contains(err.Error(), "denied") {
		t.Errorf("expected error to contain 'denied', got %v", err)
	}
}

func TestAssistantAgentHarness_Compact_Reset(t *testing.T) {
	harness, err := NewAssistantAgentHarness("")
	if err != nil {
		t.Fatalf("failed to create harness: %v", err)
	}

	if err := harness.Compact(); err != nil {
		t.Errorf("Compact failed: %v", err)
	}

	if err := harness.Reset(); err != nil {
		t.Errorf("Reset failed: %v", err)
	}
}
