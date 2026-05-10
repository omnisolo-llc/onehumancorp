package harness

import (
    "testing"
)

type MockHarness struct{}

func (m *MockHarness) RunAttempt(cmd string) (*AttemptResult, error) {
    return &AttemptResult{
        Stdout:   "mock stdout",
        ExitCode: 0,
    }, nil
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
