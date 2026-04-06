package builtin

import (
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/local"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type OrchestrationHubAdapter struct {
	hub *orchestration.Hub
}

func NewOrchestrationHubAdapter(hub *orchestration.Hub) local.Hub {
	return &OrchestrationHubAdapter{hub: hub}
}

func (a *OrchestrationHubAdapter) RegisterAgent(agent local.HubAgent) {
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

func (a *OrchestrationHubAdapter) Inbox(agentID string) []local.HubMessage {
	msgs := a.hub.Inbox(agentID)
	out := make([]local.HubMessage, 0, len(msgs))
	for _, m := range msgs {
		out = append(out, local.HubMessage{
			ID:        m.ID,
			FromAgent: m.FromAgent,
			ToAgent:   m.ToAgent,
			Type:      m.Type,
			Content:   m.Content,
		})
	}
	return out
}

func (a *OrchestrationHubAdapter) Publish(msg local.HubMessage) error {
	return a.hub.Publish(orchestration.Message{
		ID:         msg.ID,
		FromAgent:  msg.FromAgent,
		ToAgent:    msg.ToAgent,
		Type:       msg.Type,
		Content:    msg.Content,
		OccurredAt: time.Now().UTC(),
	})
}
