package mesh

import (
	"context"

	"github.com/redis/go-redis/v9"
)

type RedisPubSub struct {
	client *redis.Client
}

func NewRedisPubSub(client *redis.Client) *RedisPubSub {
	return &RedisPubSub{client: client}
}

func (r *RedisPubSub) Publish(ctx context.Context, topic string, message []byte) error {
	return r.client.Publish(ctx, topic, message).Err()
}

func (r *RedisPubSub) Subscribe(ctx context.Context, topic string) (<-chan []byte, error) {
	pubsub := r.client.Subscribe(ctx, topic)
	ch := make(chan []byte, 100)

	go func() {
		defer close(ch)
		defer pubsub.Close()
		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-pubsub.Channel():
				if !ok {
					return
				}
				ch <- []byte(msg.Payload)
			}
		}
	}()

	return ch, nil
}
