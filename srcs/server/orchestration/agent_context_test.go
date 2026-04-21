package orchestration

import (
	"context"
	"sync"
	"testing"
)

func TestAgentContext_Isolation(t *testing.T) {
	ctx := context.Background()

	ac1 := &AgentContext{
		AgentID:         "agent-1",
		AgentType:       "subagent",
		ParentSessionID: "session-A",
	}

	ac2 := &AgentContext{
		AgentID:         "agent-2",
		AgentType:       "teammate",
		ParentSessionID: "session-B",
	}

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		defer wg.Done()
		ctx1 := WithAgentContext(ctx, ac1)
		retrieved, ok := GetAgentContext(ctx1)
		if !ok || retrieved.AgentID != "agent-1" || retrieved.ParentSessionID != "session-A" {
			t.Errorf("goroutine 1 failed to retrieve correct context")
		}
	}()

	go func() {
		defer wg.Done()
		ctx2 := WithAgentContext(ctx, ac2)
		retrieved, ok := GetAgentContext(ctx2)
		if !ok || retrieved.AgentID != "agent-2" || retrieved.ParentSessionID != "session-B" {
			t.Errorf("goroutine 2 failed to retrieve correct context")
		}
	}()

	wg.Wait()
}

func TestAgentContext_NotFound(t *testing.T) {
	ctx := context.Background()
	_, ok := GetAgentContext(ctx)
	if ok {
		t.Errorf("expected GetAgentContext to return false for empty context")
	}
}
