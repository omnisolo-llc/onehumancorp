package mesh

import (
	"context"
	"encoding/json"
	"log"

	"github.com/redis/go-redis/v9"
)

// MeshMessage represents a payload sent over the Teammate Mesh.
type MeshMessage struct {
	SenderID string          `json:"sender_id"`
	Topic    string          `json:"topic"`
	Payload  json.RawMessage `json:"payload"`
}

// TeammateMesh is the Redis Pub/Sub powered communication layer.
type TeammateMesh struct {
	client *redis.Client
}

// NewTeammateMesh initializes a new TeammateMesh.
func NewTeammateMesh(client *redis.Client) *TeammateMesh {
	return &TeammateMesh{client: client}
}

// Publish sends a message to the specified topic.
func (m *TeammateMesh) Publish(ctx context.Context, msg MeshMessage) error {
	data, err := json.Marshal(msg)
	if err != nil {
		return err
	}
	return m.client.Publish(ctx, msg.Topic, data).Err()
}

// Subscribe listens to a topic and handles messages.
func (m *TeammateMesh) Subscribe(ctx context.Context, topic string, handler func(MeshMessage)) {
	sub := m.client.Subscribe(ctx, topic)
	ch := sub.Channel()

	go func() {
		defer sub.Close()
		for {
			select {
			case <-ctx.Done():
				return
			case redisMsg := <-ch:
				var msg MeshMessage
				if err := json.Unmarshal([]byte(redisMsg.Payload), &msg); err != nil {
					log.Printf("Failed to unmarshal mesh message: %v", err)
					continue
				}
				handler(msg)
			}
		}
	}()
}
