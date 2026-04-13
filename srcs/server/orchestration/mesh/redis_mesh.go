package mesh

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/google/uuid"
	"github.com/redis/go-redis/v9"
)

var releaseLockScript = redis.NewScript(`
	if redis.call("get", KEYS[1]) == ARGV[1] then
		return redis.call("del", KEYS[1])
	else
		return 0
	end
`)

type redisSubscription struct {
	pubsub *redis.PubSub
}

func (s *redisSubscription) Close() error {
	return s.pubsub.Close()
}

// RedisMesh implements TeammateMesh using Redis.
type RedisMesh struct {
	client     *redis.Client
	instanceID string
}

// NewRedisMesh creates a new RedisMesh instance.
func NewRedisMesh(redisURL string) (*RedisMesh, error) {
	opts, err := redis.ParseURL(redisURL)
	if err != nil {
		return nil, fmt.Errorf("invalid redis url: %w", err)
	}
	client := redis.NewClient(opts)

	// Verify connection
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	if err := client.Ping(ctx).Err(); err != nil {
		return nil, fmt.Errorf("failed to ping redis: %w", err)
	}

	return &RedisMesh{
		client:     client,
		instanceID: uuid.NewString(),
	}, nil
}

// Publish sends a message to all subscribers of the topic.
func (rm *RedisMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	return rm.client.Publish(ctx, topic, payload).Err()
}

// Subscribe listens to messages on a given topic.
func (rm *RedisMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	pubsub := rm.client.Subscribe(ctx, topic)

	// Verify subscription success
	_, err := pubsub.Receive(ctx)
	if err != nil {
		pubsub.Close()
		return nil, fmt.Errorf("redis subscribe error: %w", err)
	}

	ch := pubsub.Channel()

	go func() {
		defer pubsub.Close()
		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-ch:
				if !ok {
					return
				}
				handler([]byte(msg.Payload))
			}
		}
	}()

	return &redisSubscription{pubsub: pubsub}, nil
}

// AcquireLock attempts to acquire a lock using SET NX PX.
func (rm *RedisMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	// OHC-SIP: unique token must be used to ensure locks are only released by the owner
	lockKey := "lock:" + key
	ok, err := rm.client.SetNX(ctx, lockKey, rm.instanceID, ttl).Result()
	if err != nil {
		return false, fmt.Errorf("redis setnx error: %w", err)
	}
	return ok, nil
}

// ReleaseLock releases the lock using a Lua script to verify ownership.
func (rm *RedisMesh) ReleaseLock(ctx context.Context, key string) error {
	lockKey := "lock:" + key
	err := releaseLockScript.Run(ctx, rm.client, []string{lockKey}, rm.instanceID).Err()
	if err != nil {
		return fmt.Errorf("redis release lock error: %w", err)
	}
	return nil
}

// RegisterPresence sets the agent's status with a TTL.
func (rm *RedisMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	presenceKey := "presence:" + agentID

	presence := AgentPresence{
		AgentID: agentID,
		Status:  status,
		Updated: time.Now(),
	}

	data, err := json.Marshal(presence)
	if err != nil {
		return fmt.Errorf("failed to marshal presence: %w", err)
	}

	// 2 minute TTL to handle dead agents
	if err := rm.client.Set(ctx, presenceKey, data, 2*time.Minute).Err(); err != nil {
		return fmt.Errorf("redis set presence error: %w", err)
	}

	return nil
}

// GetActiveAgents retrieves all active agents from Redis.
func (rm *RedisMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	var agents []AgentPresence

	// Scan for all presence keys
	var cursor uint64
	for {
		var keys []string
		var err error
		keys, cursor, err = rm.client.Scan(ctx, cursor, "presence:*", 100).Result()
		if err != nil {
			return nil, fmt.Errorf("redis scan error: %w", err)
		}

		if len(keys) > 0 {
			values, err := rm.client.MGet(ctx, keys...).Result()
			if err != nil {
				slog.Error("redis mget error during presence fetch", "error", err)
				continue
			}

			for _, val := range values {
				if val == nil {
					continue
				}
				strVal, ok := val.(string)
				if !ok {
					continue
				}

				var p AgentPresence
				if err := json.Unmarshal([]byte(strVal), &p); err == nil {
					agents = append(agents, p)
				}
			}
		}

		if cursor == 0 {
			break
		}
	}

	return agents, nil
}
