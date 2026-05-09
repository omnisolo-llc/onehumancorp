package orchestration

import (
	"context"

	"github.com/redis/go-redis/v9"
)

// RedisMesh implements MeshHub for cloud-native operation using Redis Pub/Sub.
type RedisMesh struct {
	client *redis.Client
}

// NewRedisMesh creates a new RedisMesh.
func NewRedisMesh(client *redis.Client) *RedisMesh {
	return &RedisMesh{client: client}
}

// Publish publishes a message to a channel.
func (m *RedisMesh) Publish(ctx context.Context, channel string, data []byte) error {
	return m.client.Publish(ctx, channel, data).Err()
}

// Subscribe subscribes to a channel and handles messages.
func (m *RedisMesh) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	pubsub := m.client.Subscribe(ctx, channel)

	// Wait for confirmation that subscription is created before returning
	_, err := pubsub.Receive(ctx)
	if err != nil {
		return err
	}

	go func() {
		defer pubsub.Close()
		ch := pubsub.Channel()
		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-ch:
				if !ok {
					return
				}
				handler([]byte(msg.Payload))
			}
		}
	}()

	return nil
}