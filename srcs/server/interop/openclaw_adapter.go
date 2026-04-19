package interop

import (
	"context"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/agent"
	"github.com/onehumancorp/mono/srcs/server/agent/harness"
	"github.com/onehumancorp/mono/srcs/server/harness/authz"
)

// OpenClawAdapter integrates an external OpenClaw agent instance.
type OpenClawAdapter struct {
	Identity string
	executor *agent.Executor
}

func NewOpenClawAdapter(identity string, authorizer *authz.CapabilityAuthorizer) (*OpenClawAdapter, error) {
	if err := ValidateSPIFFEID(identity); err != nil {
		return nil, fmt.Errorf("invalid identity for OpenClawAdapter: %w", err)
	}

	var interceptor *authz.CapabilityInterceptor
	if authorizer != nil {
		interceptor = authz.NewCapabilityInterceptor(authorizer)
	}
	return &OpenClawAdapter{
		Identity: identity,
		executor: agent.NewExecutor(harness.NewIsolationHarness(), interceptor),
	}, nil
}

// Ensure interface compliance.
var _ UniversalAdapter = (*OpenClawAdapter)(nil)

// SyncState functionality.
func (a *OpenClawAdapter) SyncState(ctx context.Context, state *State) error {
	// Mock K8s/LangGraph state sync
	if state == nil {
		return fmt.Errorf("state cannot be nil")
	}
	// Simulate adding OpenClaw specific metadata
	if state.Data == nil {
		state.Data = make(map[string]interface{})
	}
	state.Data["openclaw_synced"] = true
	state.Data["last_identity"] = a.Identity

	// Ensure shared state via LangGraph is synchronized
	LogCheckpoint(state, a.Identity)
	return nil
}

// ExecuteCommand functionality.
func (a *OpenClawAdapter) ExecuteCommand(ctx context.Context, cmd string) (string, error) {
	if cmd == "" {
		return "", fmt.Errorf("empty command")
	}

	// OpenClaw sessions would have unique session IDs based on Identity, dummy it for now.
	sessionID := a.Identity + "-session"
	out, err := a.executor.ExecuteCommand(ctx, sessionID, cmd)
	if err != nil {
		return "", err
	}

	return fmt.Sprintf("OpenClaw executed: %s\nOutput: %s", cmd, string(out)), nil
}
