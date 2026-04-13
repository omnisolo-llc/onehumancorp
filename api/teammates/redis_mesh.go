package teammates

import (
	"context"
	"time"

	"github.com/redis/go-redis/v9"
)

type RedisSubscription struct {
	pubsub *redis.PubSub
}

func (s *RedisSubscription) Close() error {
	return s.pubsub.Close()
}

type RedisMesh struct {
	client *redis.Client
}

func NewRedisMesh(client *redis.Client) *RedisMesh {
	return &RedisMesh{client: client}
}

func (r *RedisMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	return r.client.Publish(ctx, topic, payload).Err()
}

func (r *RedisMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	pubsub := r.client.Subscribe(ctx, topic)
	_, err := pubsub.Receive(ctx)
	if err != nil {
		return nil, err
	}

	go func() {
		ch := pubsub.Channel()
		for msg := range ch {
			handler([]byte(msg.Payload))
		}
	}()

	return &RedisSubscription{pubsub: pubsub}, nil
}

func (r *RedisMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	return r.client.SetNX(ctx, "lock:"+key, "1", ttl).Result()
}

func (r *RedisMesh) ReleaseLock(ctx context.Context, key string) error {
	return r.client.Del(ctx, "lock:"+key).Err()
}

func (r *RedisMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	return r.client.HSet(ctx, "presence", agentID, status).Err()
}

func (r *RedisMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	res, err := r.client.HGetAll(ctx, "presence").Result()
	if err != nil {
		return nil, err
	}
	var agents []AgentPresence
	for id, status := range res {
		agents = append(agents, AgentPresence{AgentID: id, Status: status})
	}
	return agents, nil
}
