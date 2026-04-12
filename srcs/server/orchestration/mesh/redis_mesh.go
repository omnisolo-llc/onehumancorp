package mesh

import (
	"context"
	"time"

	"github.com/redis/go-redis/v9"
)

type redisSubscription struct {
	pubsub *redis.PubSub
}

func (s *redisSubscription) Close() error {
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
	return m.client.HSet(ctx, "presence", agentID, status).Err()
}

func (m *RedisMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	res, err := m.client.HGetAll(ctx, "presence").Result()
	if err != nil {
		return nil, err
	}

	var agents []AgentPresence
	for id, status := range res {
		agents = append(agents, AgentPresence{
			AgentID: id,
			Status:  status,
		})
	}
	return agents, nil
}
