package mesh

import (
	"context"
	"encoding/json"

	"github.com/go-redis/redis/v8"
)

// RedisPubSub implements MeshPubSub using Redis
type RedisPubSub struct {
	client *redis.Client
}

// NewRedisPubSub creates a new RedisPubSub
func NewRedisPubSub(addr string) *RedisPubSub {
	client := redis.NewClient(&redis.Options{
		Addr: addr,
	})
	return &RedisPubSub{client: client}
}

func (r *RedisPubSub) Publish(ctx context.Context, topic string, message TeammateMeshEvent) error {
	data, err := json.Marshal(message)
	if err != nil {
		return err
	}
	return r.client.Publish(ctx, topic, data).Err()
}

func (r *RedisPubSub) Subscribe(ctx context.Context, topic string) (<-chan TeammateMeshEvent, error) {
	pubsub := r.client.Subscribe(ctx, topic)
	ch := make(chan TeammateMeshEvent)

	go func() {
		defer close(ch)
		defer pubsub.Close()

        channel := pubsub.Channel()
		for {
            select {
            case <-ctx.Done():
                return
            case msg, ok := <-channel:
                if !ok {
                    return
                }
                var event TeammateMeshEvent
                if err := json.Unmarshal([]byte(msg.Payload), &event); err == nil {
                    select {
                    case <-ctx.Done():
                        return
                    case ch <- event:
                    }
                }
            }
		}
	}()

	return ch, nil
}

func (r *RedisPubSub) Close() error {
	return r.client.Close()
}
