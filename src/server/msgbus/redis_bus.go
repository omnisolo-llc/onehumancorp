package msgbus

import (
	"context"
	"fmt"

	goredis "github.com/redis/go-redis/v9"
)

// RedisBus is a Bus backed by Redis/Valkey pub/sub.  It is the recommended
// backend for large-scale cluster deployments.
type RedisBus struct {
	client *goredis.Client
}

func newRedisBus(cfg Config) (*RedisBus, error) {
	addr := cfg.RedisAddr
	if addr == "" {
		addr = "localhost:6379"
	}
	client := goredis.NewClient(&goredis.Options{
		Addr:     addr,
		Password: cfg.RedisPassword,
		DB:       cfg.RedisDB,
	})

	// Verify connectivity.
	ctx := context.Background()
	if err := client.Ping(ctx).Err(); err != nil {
		_ = client.Close()
		return nil, fmt.Errorf("msgbus/redis: ping %q: %w", addr, err)
	}
	return &RedisBus{client: client}, nil
}

// Publish publishes msg.Payload to msg.Topic using Redis PUBLISH.
func (b *RedisBus) Publish(ctx context.Context, msg Message) error {
	return b.client.Publish(ctx, msg.Topic, msg.Payload).Err()
}

// Subscribe registers handler for all messages on topic via Redis SUBSCRIBE.
// The subscription is managed in a background goroutine that runs until the
// returned cancel function is called.
func (b *RedisBus) Subscribe(topic string, handler Handler) (func(), error) {
	pubsub := b.client.Subscribe(context.Background(), topic)

	// Verify the subscription was set up.
	if _, err := pubsub.Receive(context.Background()); err != nil {
		_ = pubsub.Close()
		return nil, fmt.Errorf("msgbus/redis: subscribe %q: %w", topic, err)
	}

	ctx, cancel := context.WithCancel(context.Background())

	go func() {
		ch := pubsub.Channel()
		for {
			select {
			case <-ctx.Done():
				return
			case m, ok := <-ch:
				if !ok {
					return
				}
				handler(Message{Topic: m.Channel, Payload: []byte(m.Payload)})
			}
		}
	}()

	return func() {
		cancel()
		_ = pubsub.Close()
	}, nil
}

// Close closes the underlying Redis client.
func (b *RedisBus) Close() error { return b.client.Close() }
