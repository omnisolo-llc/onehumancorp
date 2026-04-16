package mesh

import (
	"context"
	"encoding/json"
	"fmt"
)

// HandoffPayload represents the intent payload for a mission handoff
type HandoffPayload struct {
	MissionID string `json:"missionID"`
	Action    string `json:"action"`
}

// HandoffAdapter abstracts broadcasting and subscribing to mission handoffs
type HandoffAdapter interface {
	BroadcastHandoff(ctx context.Context, missionID string) error
	SubscribeHandoffs(ctx context.Context) (<-chan string, error)
}

// HybridHandoffAdapter implements HandoffAdapter using a TeammateMeshService
type HybridHandoffAdapter struct {
	Service TeammateMeshService
}

// NewHybridHandoffAdapter creates a new HybridHandoffAdapter
func NewHybridHandoffAdapter(service TeammateMeshService) *HybridHandoffAdapter {
	return &HybridHandoffAdapter{Service: service}
}

// BroadcastHandoff constructs a JSON payload for handoff and broadcasts it
func (a *HybridHandoffAdapter) BroadcastHandoff(ctx context.Context, missionID string) error {
	payload := HandoffPayload{
		MissionID: missionID,
		Action:    "handoff",
	}

	data, err := json.Marshal(payload)
	if err != nil {
		return fmt.Errorf("failed to marshal handoff payload: %w", err)
	}

	return a.Service.BroadcastIntent(ctx, string(data))
}

// SubscribeHandoffs returns a channel that receives raw handoff intents
func (a *HybridHandoffAdapter) SubscribeHandoffs(ctx context.Context) (<-chan string, error) {
	return a.Service.Subscribe(ctx)
}
