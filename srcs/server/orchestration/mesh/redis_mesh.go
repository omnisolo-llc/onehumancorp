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
	opt, err := redis.ParseURL(redisURL)
	if err != nil {
		return nil, err
	}
	client := redis.NewClient(opt)
	return &RedisMesh{client: client}, nil
}

func (rm *RedisMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	return rm.client.Publish(ctx, topic, payload).Err()
}

type redisSubscription struct {
	pubsub *redis.PubSub
}

func (rs *redisSubscription) Close() error {
	return rs.pubsub.Close()
}

func (rm *RedisMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	pubsub := rm.client.Subscribe(ctx, topic)
	go func() {
		for msg := range pubsub.Channel() {
			handler([]byte(msg.Payload))
		}
	}()
	return &redisSubscription{pubsub: pubsub}, nil
}

func (rm *RedisMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	return rm.client.SetNX(ctx, "lock:"+key, "1", ttl).Result()
}

func (rm *RedisMesh) ReleaseLock(ctx context.Context, key string) error {
	return rm.client.Del(ctx, "lock:"+key).Err()
}

func (rm *RedisMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	presence := AgentPresence{
		AgentID:   agentID,
		Status:    status,
		UpdatedAt: time.Now(),
	}
	data, err := json.Marshal(presence)
	if err != nil {
		return err
	}
	return rm.client.Set(ctx, "presence:"+agentID, data, 30*time.Second).Err()
}

func (rm *RedisMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	var keys []string
	var cursor uint64
	for {
		var err error
		var batch []string
		batch, cursor, err = rm.client.Scan(ctx, cursor, "presence:*", 100).Result()
		if err != nil {
			return nil, err
		}
		keys = append(keys, batch...)
		if cursor == 0 {
			break
		}
	}

	var agents []AgentPresence
	for _, key := range keys {
		val, err := rm.client.Get(ctx, key).Result()
		if err == nil {
			var p AgentPresence
			if err := json.Unmarshal([]byte(val), &p); err == nil {
				agents = append(agents, p)
			}
		}
	}
	return agents, nil
}
