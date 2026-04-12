package mesh

import (
	"context"
	"encoding/json"
	"time"

	"github.com/redis/go-redis/v9"
)

type RedisMesh struct {
	client *redis.Client
}

func NewRedisMesh(redisURL string) (*RedisMesh, error) {
	opts, err := redis.ParseURL(redisURL)
	if err != nil {
		return nil, err
	}
	client := redis.NewClient(opts)
	return &RedisMesh{client: client}, nil
}

type redisSubscription struct {
	pubsub *redis.PubSub
}

func (s *redisSubscription) Unsubscribe() error {
	return s.pubsub.Close()
}

func (m *RedisMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	return m.client.Publish(ctx, topic, payload).Err()
}

func (m *RedisMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	pubsub := m.client.Subscribe(ctx, topic)
	go func() {
		ch := pubsub.Channel()
		for msg := range ch {
			handler([]byte(msg.Payload))
		}
	}()
	return &redisSubscription{pubsub: pubsub}, nil
}

func (m *RedisMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	return m.client.SetNX(ctx, "lock:"+key, "1", ttl).Result()
}

func (m *RedisMesh) ReleaseLock(ctx context.Context, key string) error {
	return m.client.Del(ctx, "lock:"+key).Err()
}

func (m *RedisMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	presence := AgentPresence{
		AgentID:  agentID,
		Status:   status,
		LastSeen: time.Now(),
	}
	data, err := json.Marshal(presence)
	if err != nil {
		return err
	}
	return m.client.Set(ctx, "presence:"+agentID, data, 10*time.Minute).Err()
}

func (m *RedisMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	keys, err := m.client.Keys(ctx, "presence:*").Result()
	if err != nil {
		return nil, err
	}

	var agents []AgentPresence
	for _, key := range keys {
		data, err := m.client.Get(ctx, key).Bytes()
		if err != nil {
			continue
		}
		var p AgentPresence
		if err := json.Unmarshal(data, &p); err == nil {
			agents = append(agents, p)
		}
	}
	return agents, nil
}
