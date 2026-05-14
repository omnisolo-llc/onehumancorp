package orchestration

import (
	"fmt"
	"sync"
	apiharness "onehumancorp/src/server/api/harness"
)

type HarnessResolver struct {
	mu             sync.RWMutex
	defaultHarness apiharness.AgentHarness
	registry       map[string]apiharness.AgentHarness
}

func NewHarnessResolver() *HarnessResolver {
	return &HarnessResolver{
		defaultHarness: &MockHarness{}, // Using a mock for now
		registry:       make(map[string]apiharness.AgentHarness),
	}
}

func (r *HarnessResolver) Register(id string, harness apiharness.AgentHarness) {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.registry[id] = harness
}

func (r *HarnessResolver) Resolve(agentID string) (apiharness.AgentHarness, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if harness, ok := r.registry[agentID]; ok {
		return harness, nil
	}

	if r.defaultHarness != nil {
		return r.defaultHarness, nil
	}

	return nil, fmt.Errorf("no harness registered for %s and no default harness", agentID)
}

type MockHarness struct{
	Called bool
}

func (m *MockHarness) RunAttempt(cmd string) (*apiharness.AttemptResult, error) {
	m.Called = true
	return &apiharness.AttemptResult{
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
