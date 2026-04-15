package mesh

import (
	"context"

	"github.com/redis/go-redis/v9"
)

type RedisMeshBroker struct {
	client *redis.Client
}

func NewRedisMeshBroker(client *redis.Client) *RedisMeshBroker {
	return &RedisMeshBroker{client: client}
}

func (b *RedisMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	return b.client.Publish(ctx, channel, payload).Err()
}

func (b *RedisMeshBroker) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	pubsub := b.client.Subscribe(ctx, channel)

	out := make(chan []byte, 100)
	go func() {
		defer pubsub.Close()
		defer close(out)

		ch := pubsub.Channel()
		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-ch:
				if !ok {
					return
				}
				select {
				case out <- []byte(msg.Payload):
				case <-ctx.Done():
					return
				}
			}
		}
	}()

	return out, nil
}
