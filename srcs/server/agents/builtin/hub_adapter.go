package builtin

import (
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type OrchestrationHubAdapter struct {
	hub *orchestration.Hub
}

func NewOrchestrationHubAdapter(hub *orchestration.Hub) Hub {
	return &OrchestrationHubAdapter{hub: hub}
}

func (a *OrchestrationHubAdapter) RegisterAgent(agent HubAgent) {
	a.hub.RegisterAgent(orchestration.Agent{
		ID:             agent.ID,
		Name:           agent.Name,
		Role:           agent.Role,
		OrganizationID: agent.OrganizationID,
		Status:         orchestration.Status(agent.Status),
		ProviderType:   agent.ProviderType,
		Region:         agent.Region,
		Managed:        agent.Managed,
	})
}

func (a *OrchestrationHubAdapter) Subscribe(agentID string) (<-chan struct{}, func()) {
	return a.hub.Subscribe(agentID)
}

func (a *OrchestrationHubAdapter) Inbox(agentID string) []HubMessage {
	msgs := a.hub.Inbox(agentID)
	out := make([]HubMessage, 0, len(msgs))
	for _, msg := range msgs {
		out = append(out, HubMessage{
			ID:        msg.ID,
			FromAgent: msg.FromAgent,
			ToAgent:   msg.ToAgent,
			Type:      msg.Type,
			Content:   msg.Content,
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
