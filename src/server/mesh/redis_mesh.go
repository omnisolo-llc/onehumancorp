package mesh

import (
	"context"

	"github.com/redis/go-redis/v9"
)

// RedisMesh implements TeammateMesh using Redis Pub/Sub.
type RedisMesh struct {
	client *redis.Client
}

// NewRedisMesh creates a new RedisMesh.
func NewRedisMesh(client *redis.Client) *RedisMesh {
	return &RedisMesh{
		client: client,
	}
}

// Publish sends a message to the specified channel.
func (r *RedisMesh) Publish(channel string, message []byte) error {
	return r.client.Publish(context.Background(), channel, message).Err()
}

// Subscribe returns a channel that receives messages from the specified channel.
func (r *RedisMesh) Subscribe(channel string) (<-chan []byte, error) {
	pubsub := r.client.Subscribe(context.Background(), channel)

	// Ensure the subscription is established.
	_, err := pubsub.Receive(context.Background())
	if err != nil {
		pubsub.Close()
		return nil, err
	}

	ch := make(chan []byte, 100)
	go func() {
		defer pubsub.Close()
		defer close(ch)

		redisCh := pubsub.Channel()
		for msg := range redisCh {
			select {
			case ch <- []byte(msg.Payload):
			default:
				// Non-blocking send to avoid hanging the bridge goroutine
				// if the consumer is slow or stops reading.
			}
		}
	}()

	return ch, nil
}
