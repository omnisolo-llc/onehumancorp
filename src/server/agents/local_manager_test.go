package agents

import (
	"context"
	"testing"
)

func TestLocalManager_SpawnAgent(t *testing.T) {
	// Use 'echo' as a dummy binary for testing
	manager := NewLocalManager("echo", false)

	agent := Agent{
		ID:   "agent-1",
		Role: "admin",
	}

	err := manager.SpawnAgent(context.Background(), agent, "")
	if err != nil {
		t.Fatalf("SpawnAgent failed: %v", err)
	}

	status, err := manager.GetAgentStatus(context.Background(), "agent-1")
	if err != nil {
		t.Fatalf("GetAgentStatus failed: %v", err)
	}

	t.Logf("Agent status: %v", status)

	err = manager.TerminateAgent(context.Background(), "agent-1")
	if err != nil {
		t.Logf("TerminateAgent failed (might have already finished): %v", err)
	}
}
