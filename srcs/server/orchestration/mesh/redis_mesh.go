package mesh

import (
	"context"
	"encoding/json"
	"time"

	"github.com/redis/go-redis/v9"
)

type RedisSubscription struct {
	pubsub *redis.PubSub
}

func (s *RedisSubscription) Unsubscribe() error {
	return s.pubsub.Close()
}

type RedisMesh struct {
	client *redis.Client
}

func NewRedisMesh(client *redis.Client) *RedisMesh {
	return &RedisMesh{client: client}
}

func (m *RedisMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	return m.client.Publish(ctx, topic, payload).Err()
}

func (m *RedisMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	pubsub := m.client.Subscribe(ctx, topic)

	// Ensure subscription is actually created before returning
	_, err := pubsub.Receive(ctx)
	if err != nil {
		pubsub.Close()
		return nil, err
	}

	ch := pubsub.Channel()
	go func() {
		for msg := range ch {
			handler([]byte(msg.Payload))
		}
	}()
	return &RedisSubscription{pubsub: pubsub}, nil
}

func (m *RedisMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	lockKey := "mesh:lock:" + key
	return m.client.SetNX(ctx, lockKey, "locked", ttl).Result()
}

func (m *RedisMesh) ReleaseLock(ctx context.Context, key string) error {
	lockKey := "mesh:lock:" + key
	return m.client.Del(ctx, lockKey).Err()
}

func (m *RedisMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	p := AgentPresence{AgentID: agentID, Status: status}
	data, err := json.Marshal(p)
	if err != nil {
		return err
	}

	key := "mesh:presence:" + agentID
	return m.client.Set(ctx, key, data, 30*time.Second).Err()
}

func (m *RedisMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	var active []AgentPresence
	iter := m.client.Scan(ctx, 0, "mesh:presence:*", 0).Iterator()
	for iter.Next(ctx) {
		key := iter.Val()
		val, err := m.client.Get(ctx, key).Result()
		if err != nil {
			continue // key might have expired
		}
		var p AgentPresence
		if err := json.Unmarshal([]byte(val), &p); err == nil {
			active = append(active, p)
		}
	}
	if err := iter.Err(); err != nil {
		return nil, err
	}
	return active, nil
}
