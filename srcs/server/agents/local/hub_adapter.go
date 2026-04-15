//go:build bazel

package local

import (
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// OrchestrationHubAdapter wraps orchestration.Hub to satisfy the local.Hub interface.
// It is compiled only when the orchestration package is available (Bazel build).
type OrchestrationHubAdapter struct {
	hub *orchestration.Hub
}

// NewOrchestrationHubAdapter wraps an orchestration.Hub for use with a local Runner.
func NewOrchestrationHubAdapter(hub *orchestration.Hub) Hub {
	return &OrchestrationHubAdapter{hub: hub}
}

func (a *OrchestrationHubAdapter) RegisterAgent(agent HubAgent) {
	a.hub.RegisterAgent(orchestration.Agent{
		ID:           agent.ID,
		Name:         agent.Name,
		Role:         agent.Role,
		Status:       orchestration.Status(agent.Status),
		ProviderType: agent.ProviderType,
	})
}

func (a *OrchestrationHubAdapter) Subscribe(agentID string) (<-chan struct{}, func()) {
	return a.hub.Subscribe(agentID)
}

func (a *OrchestrationHubAdapter) Inbox(agentID string) []HubMessage {
	msgs := a.hub.Inbox(agentID)
	out := make([]HubMessage, 0, len(msgs))
	for _, m := range msgs {
		out = append(out, HubMessage{
			ID:        m.ID,
			FromAgent: m.FromAgent,
			ToAgent:   m.ToAgent,
			Type:      m.Type,
			Content:   m.Content,
		})
	}
	return out
}

func (a *OrchestrationHubAdapter) Publish(msg HubMessage) error {
	return a.hub.Publish(orchestration.Message{
		ID:         msg.ID,
		FromAgent:  msg.FromAgent,
		ToAgent:    msg.ToAgent,
		Type:       msg.Type,
		Content:    msg.Content,
		OccurredAt: time.Now().UTC(),
	})
}

// StartDefaultRunner creates and starts a default local agent runner connected to
// the given Hub. It returns the runner so the caller can stop it later.
// This is the canonical wiring point called from main.go when no external
// provider is configured.
func StartDefaultRunner(hub *orchestration.Hub, cfg AgentConfig) (*Runner, error) {
	if hub == nil {
		return nil, fmt.Errorf("StartDefaultRunner: hub must not be nil")
	}

	// Attempt to wrap LLM with CachedLLMClient if DB is available in Hub
	if cfg.LLM == nil {
		baseLLM := defaultLLMClient()
		if hub.DB() != nil {
			cfg.LLM = NewCachedLLMClient(baseLLM, hub.DB(), hub.Redis())
		} else {
			cfg.LLM = baseLLM
		}
	}

	// Persist task output to the database when a DB is available.
	if cfg.DBProvider == nil && hub.DB() != nil {
		cfg.DBProvider = hub.DB()
	}

	adapter := NewOrchestrationHubAdapter(hub)
	runner := NewRunner(adapter, "", "", "", cfg)
	return runner, nil
}
