package orchestration

import (
	"context"
	"encoding/json"
	"log/slog"
)

// MeshMessage represents a broadcast message over the Teammate Mesh API.
type MeshMessage struct {
	Type        string `json:"type"`
	TaskID      string `json:"task_id,omitempty"`
	Priority    string `json:"priority,omitempty"`
	Description string `json:"description,omitempty"`
	SenderID    string `json:"sender_id,omitempty"`
	Role        string `json:"role,omitempty"`
	Content     string `json:"content,omitempty"`
	Timestamp   string `json:"timestamp,omitempty"`
}

// BroadcastMeshMessage publishes a mesh message to all connected agents in the specified Centrifuge channel.
func (h *Hub) BroadcastMeshMessage(ctx context.Context, roomID string, msg MeshMessage) error {
	cn := h.CentrifugeNode()
	if cn == nil {
		slog.Warn("BroadcastMeshMessage: centrifuge node is not initialized")
		return nil
	}

	channel := "mesh:" + roomID
	data, err := json.Marshal(msg)
	if err != nil {
		return err
	}

	_, err = cn.node.Publish(channel, data)
	return err
}
