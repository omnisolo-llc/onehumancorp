package agents

import (
	"context"
)

// AgentManager defines the interface for managing the lifecycle of AI agents.
type AgentManager interface {
	// SpawnAgent starts a new agent instance based on the provider type and configuration.
	SpawnAgent(ctx context.Context, agent Agent, config string) error

	// TerminateAgent stops a running agent instance.
	TerminateAgent(ctx context.Context, agentID string) error

	// GetAgentStatus retrieves the current status of an agent instance.
	GetAgentStatus(ctx context.Context, agentID string) (Status, error)
}
