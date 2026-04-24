package orchestration

import (
	"context"
)

type AgentContext struct {
	AgentID         string
	AgentType       string
	ParentSessionID string
	Env             map[string]string
}

type agentContextKey struct{}

func WithAgentContext(ctx context.Context, ac *AgentContext) context.Context {
	return context.WithValue(ctx, agentContextKey{}, ac)
}

func GetAgentContext(ctx context.Context) (*AgentContext, bool) {
	ac, ok := ctx.Value(agentContextKey{}).(*AgentContext)
	return ac, ok
}
