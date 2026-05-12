package orchestration

import (
    "onehumancorp/srcs/server/orchestration/harness"
)

type HarnessResolver struct {
    defaultHarness harness.AgentHarness
}

func NewHarnessResolver() *HarnessResolver {
    return &HarnessResolver{
        defaultHarness: &MockHarness{}, // Using a mock for now
    }
}

func (r *HarnessResolver) Resolve(agentID string) (harness.AgentHarness, error) {
    return r.defaultHarness, nil
}

type MockHarness struct{}

func (m *MockHarness) RunAttempt(cmd string) (*harness.AttemptResult, error) {
    return &harness.AttemptResult{
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
