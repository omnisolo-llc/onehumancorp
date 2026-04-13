package mesh

import (
	"context"
	"time"

	"github.com/redis/go-redis/v9"
)

type redisSubscription struct {
	pubsub *redis.PubSub
}

func (s *redisSubscription) Unsubscribe(ctx context.Context) error {
	return s.pubsub.Close()
}

// RedisTeammateMesh implements TeammateMesh for cloud environments.
type RedisTeammateMesh struct {
	client *redis.Client
}

// NewRedisTeammateMesh creates a new RedisTeammateMesh.
func NewRedisTeammateMesh(redisURL string) (*RedisTeammateMesh, error) {
	opts, err := redis.ParseURL(redisURL)
	if err != nil {
		return nil, err
	}

	client := redis.NewClient(opts)

	// Verify connection
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	if err := client.Ping(ctx).Err(); err != nil {
		return nil, err
	}

	return &RedisTeammateMesh{
		client: client,
	}, nil
}

// Publish sends a payload to a topic.
func (rm *RedisTeammateMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	return rm.client.Publish(ctx, topic, payload).Err()
}

// Subscribe listens to a topic.
func (rm *RedisTeammateMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	pubsub := rm.client.Subscribe(ctx, topic)

	// Wait for confirmation that subscription is created before returning
	if _, err := pubsub.Receive(ctx); err != nil {
		return nil, err
	}

	ch := pubsub.Channel()

	go func() {
		for msg := range ch {
			handler([]byte(msg.Payload))
		}
	}()

	return &redisSubscription{pubsub: pubsub}, nil
}

// AcquireLock attempts to acquire a lock for a given key.
func (rm *RedisTeammateMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	return rm.client.SetNX(ctx, key, "locked", ttl).Result()
}

// ReleaseLock releases the lock for a given key.
func (rm *RedisTeammateMesh) ReleaseLock(ctx context.Context, key string) error {
	return rm.client.Del(ctx, key).Err()
}

// RegisterPresence updates the presence for an agent.
func (rm *RedisTeammateMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	// Use string keys with TTL for auto-expiration instead of a hash without TTL
	key := "agent_presence:" + agentID
	return rm.client.Set(ctx, key, status, 30*time.Second).Err()
}

// GetActiveAgents returns a list of all active agents.
func (rm *RedisTeammateMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	keys, err := rm.client.Keys(ctx, "agent_presence:*").Result()
	if err != nil {
		return nil, err
	}

	var agents []AgentPresence
	for _, key := range keys {
		status, err := rm.client.Get(ctx, key).Result()
		if err == nil {
			// Extract agent ID from key
			agentID := key[len("agent_presence:"):]
			agents = append(agents, AgentPresence{
				AgentID: agentID,
				Status:  status,
			})
		}
	}

	return agents, nil
}
