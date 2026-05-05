package harness

import (
	"context"
)

type AgentHarness struct {
	lifecycle HarnessLifecycle
	sync      *FileSyncBridge
}

func NewAgentHarness(lifecycle HarnessLifecycle, sync *FileSyncBridge) *AgentHarness {
	return &AgentHarness{
		lifecycle: lifecycle,
		sync:      sync,
	}
}

func (h *AgentHarness) Run(ctx context.Context, agentID, prompt string) (*AttemptResult, error) {
	return h.lifecycle.RunAttempt(ctx, agentID, prompt)
}

func (h *AgentHarness) Sync(ctx context.Context, path string, content []byte) error {
	return h.sync.SyncFile(ctx, path, content)
}
