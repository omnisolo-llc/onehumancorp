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
type subagentContextKey struct{}
type teammateContextKey struct{}

func WithAgentContext(ctx context.Context, ac *AgentContext) context.Context {
	return context.WithValue(ctx, agentContextKey{}, ac)
}

func GetAgentContext(ctx context.Context) (*AgentContext, bool) {
	ac, ok := ctx.Value(agentContextKey{}).(*AgentContext)
	return ac, ok
}

// SubagentContext bounds.
type SubagentContext struct {
	AgentContext
	Priority int
}

func WithSubagentContext(ctx context.Context, sc *SubagentContext) context.Context {
	return context.WithValue(ctx, subagentContextKey{}, sc)
}

func GetSubagentContext(ctx context.Context) (*SubagentContext, bool) {
	sc, ok := ctx.Value(subagentContextKey{}).(*SubagentContext)
	return sc, ok
}

// TeammateAgentContext bounds.
type TeammateAgentContext struct {
	AgentContext
	TeamID string
}

func WithTeammateAgentContext(ctx context.Context, tc *TeammateAgentContext) context.Context {
	return context.WithValue(ctx, teammateContextKey{}, tc)
}

func GetTeammateAgentContext(ctx context.Context) (*TeammateAgentContext, bool) {
	tc, ok := ctx.Value(teammateContextKey{}).(*TeammateAgentContext)
	return tc, ok
}
