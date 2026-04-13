package mesh

import (
	"context"
	"github.com/redis/go-redis/v9"
)

type RedisPubSub struct {
	client *redis.Client
}

func NewRedisPubSub(client *redis.Client) *RedisPubSub {
	return &RedisPubSub{
		client: client,
	}
}

func (r *RedisPubSub) Publish(ctx context.Context, topic string, message []byte) error {
	return r.client.Publish(ctx, topic, message).Err()
}

func (r *RedisPubSub) Subscribe(ctx context.Context, topic string) (<-chan []byte, func() error, error) {
	pubsub := r.client.Subscribe(ctx, topic)

	// Ensure connection is established
	_, err := pubsub.Receive(ctx)
	if err != nil {
		return nil, nil, err
	}

	ch := make(chan []byte, 100)
	redisCh := pubsub.Channel()

	go func() {
		defer close(ch)
		for msg := range redisCh {
			ch <- []byte(msg.Payload)
		}
	}()

	unsubscribe := func() error {
		err := pubsub.Close()
		return err
	}

	return ch, unsubscribe, nil
}
