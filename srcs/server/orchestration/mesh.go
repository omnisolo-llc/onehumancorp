package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/centrifugal/centrifuge"
)

type TeammateMesh struct {
	node *centrifuge.Node
}

func NewTeammateMesh(node *centrifuge.Node) *TeammateMesh {
	return &TeammateMesh{
		node: node,
	}
}

type MeshMessage struct {
	SenderID  string    `json:"sender_id"`
	Role      string    `json:"role"`
	Content   string    `json:"content"`
	Timestamp time.Time `json:"timestamp"`
}

// Broadcast sends a message to all agents subscribed to a specific meeting room.
func (m *TeammateMesh) Broadcast(ctx context.Context, roomID string, msg MeshMessage) error {
	msg.Timestamp = time.Now().UTC()
	data, err := json.Marshal(msg)
	if err != nil {
		return fmt.Errorf("marshal message: %w", err)
	}

	channel := fmt.Sprintf("mesh:room:%s", roomID)

	_, err = m.node.Publish(channel, data)
	if err != nil {
		return fmt.Errorf("publish message: %w", err)
	}

	slog.Info("mesh: broadcast message", "room", roomID, "sender", msg.SenderID)
	return nil
}
