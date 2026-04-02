package orchestration

import (
	"context"
	"encoding/json"
	"log/slog"
	"time"
)

// MeshMessage represents a realtime Teammate Mesh protocol message.
type MeshMessage struct {
	Type        string `json:"type"`
	TaskID      string `json:"task_id,omitempty"`
	Priority    string `json:"priority,omitempty"`
	Description string `json:"description,omitempty"`
	SenderID    string `json:"sender_id,omitempty"`
	Role        string `json:"role,omitempty"`
	Content     string `json:"content,omitempty"`
}

// PublishToMesh broadcasts a message to the entire swarm mesh.
// In cloud mode (OHC_MULTITENANT=true), this uses Redis Pub/Sub.
// In standalone mode, it leverages the internal Centrifuge event bus.
func (h *Hub) PublishToMesh(ctx context.Context, roomID string, msg MeshMessage) error {
	payload, err := json.Marshal(msg)
	if err != nil {
		return err
	}

	// Check if Redis is enabled via environment variables.
	// For OHC Multi-Tenant Cloud mode, we expect Redis to be configured.
	// Assuming Redis is injected or available via CentrifugeNode, we use Centrifuge.
	// We'll wrap it in standard Hub message.

	hubMsg := Message{
		ID:         "mesh-" + time.Now().UTC().Format("20060102150405"),
		FromAgent:  msg.SenderID,
		ToAgent:    "all", // broadcast
		Type:       "TASK_BROADCAST",
		Content:    string(payload),
		MeetingID:  roomID,
		OccurredAt: time.Now().UTC(),
	}

	slog.Info("Teammate Mesh: broadcasting message", "room", roomID, "type", msg.Type)
	return h.Publish(hubMsg)
}
