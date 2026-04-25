package mesh

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"errors"
	"time"

	"github.com/redis/go-redis/v9"
)

type redisSubscription struct {
	pubsub *redis.PubSub
	cancel context.CancelFunc
}

func (s *redisSubscription) Close() error {
	s.cancel()
	return s.pubsub.Close()
}

type RedisMesh struct {
	client *redis.Client
}

func NewRedisMesh(client *redis.Client) *RedisMesh {
	return &RedisMesh{
		client: client,
	}
}

func (m *RedisMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	return m.client.Publish(ctx, topic, payload).Err()
}

func (m *RedisMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	pubsub := m.client.Subscribe(ctx, topic)

	// Wait for confirmation that subscription is created before proceeding
	_, err := pubsub.Receive(ctx)
	if err != nil {
		pubsub.Close()
		return nil, err
	}

	subCtx, cancel := context.WithCancel(ctx)
	ch := pubsub.Channel()

	go func() {
		for {
			select {
			case msg, ok := <-ch:
				if !ok {
					return
				}
				handler([]byte(msg.Payload))
			case <-subCtx.Done():
				return
			}
		}
	}()

	return &redisSubscription{
		pubsub: pubsub,
		cancel: cancel,
	}, nil
}

func (m *RedisMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (string, bool, error) {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	token := hex.EncodeToString(b)

	ok, err := m.client.SetNX(ctx, "lock:"+key, token, ttl).Result()
	if err != nil {
		return "", false, err
	}
	if !ok {
		return "", false, nil
	}

	return token, true, nil
}

func (m *RedisMesh) ReleaseLock(ctx context.Context, key string, token string) error {
	// Lua script to ensure atomic check-and-delete
	const script = `
		if redis.call("get", KEYS[1]) == ARGV[1] then
			return redis.call("del", KEYS[1])
		else
			return 0
		end
	`

	res, err := m.client.Eval(ctx, script, []string{"lock:" + key}, token).Result()
	if err != nil {
		return err
	}

	if count, ok := res.(int64); ok && count == 1 {
		return nil
	}

	return errors.New("lock not found or invalid token")
}
