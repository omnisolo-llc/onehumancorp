package mesh

import (
	"context"
	"encoding/json"
	"time"

	"github.com/redis/go-redis/v9"
)

type redisSubscription struct {
	pubsub *redis.PubSub
}

func (s *redisSubscription) Unsubscribe() error {
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

	// Wait for confirmation that subscription is created before returning
	_, err := pubsub.Receive(ctx)
	if err != nil {
		pubsub.Close()
		return nil, err
	}

	go func() {
		ch := pubsub.Channel()
		for msg := range ch {
			handler([]byte(msg.Payload))
		}
	}()

	return &redisSubscription{pubsub: pubsub}, nil
}

func (m *RedisMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	// Use SETNX semantic
	return m.client.SetNX(ctx, "lock:"+key, "1", ttl).Result()
}

func (m *RedisMesh) ReleaseLock(ctx context.Context, key string) error {
	return m.client.Del(ctx, "lock:"+key).Err()
}

func (m *RedisMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	presence := AgentPresence{AgentID: agentID, Status: status}
	data, err := json.Marshal(presence)
	if err != nil {
		return err
	}
	// Expire presence after 30 seconds
	return m.client.Set(ctx, "presence:" + agentID, data, 30*time.Second).Err()
}

func (m *RedisMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	keys, err := m.client.Keys(ctx, "presence:*").Result()
	if err != nil {
		return nil, err
	}

	var active []AgentPresence
	for _, key := range keys {
		data, err := m.client.Get(ctx, key).Result()
		if err == nil {
			var p AgentPresence
			if err := json.Unmarshal([]byte(data), &p); err == nil {
				active = append(active, p)
			}
		}
	}
	return active, nil
}
