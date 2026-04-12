package mesh

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/redis/go-redis/v9"
)

type RedisMesh struct {
	client    *redis.Client
	lockOwner string
}

type redisSubscription struct {
	pubsub *redis.PubSub
}

func (s *redisSubscription) Close() error {
	return s.pubsub.Close()
}

func NewRedisTeammateMesh(redisURL string) (*RedisMesh, error) {
	opts, err := redis.ParseURL(redisURL)
	if err != nil {
		return nil, err
	}
	client := redis.NewClient(opts)
	// Test connection
	if err := client.Ping(context.Background()).Err(); err != nil {
		return nil, err
	}
	return &RedisMesh{
		client:    client,
		lockOwner: uuid.New().String(),
	}, nil
}

func (rm *RedisMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	return rm.client.Publish(ctx, topic, payload).Err()
}

func (rm *RedisMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	pubsub := rm.client.Subscribe(ctx, topic)
	// Wait for confirmation that subscription is created before returning
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

	return &redisSubscription{pubsub: pubsub}, nil
}

func (rm *RedisMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	return rm.client.SetNX(ctx, "lock:"+key, rm.lockOwner, ttl).Result()
}

func (rm *RedisMesh) ReleaseLock(ctx context.Context, key string) error {
	script := `
		if redis.call("get", KEYS[1]) == ARGV[1] then
			return redis.call("del", KEYS[1])
		else
			return 0
		end
	`
	_, err := rm.client.Eval(ctx, script, []string{"lock:" + key}, rm.lockOwner).Result()
	return err
}

func (rm *RedisMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	// Use a Hash or String with TTL. Since we need to get active agents easily,
	// let's use a String with TTL and a key pattern "presence:<agentID>".
	// Alternatively, add to a ZSET with score = time.now().Unix() + ttl
	return rm.client.Set(ctx, "presence:"+agentID, status, 30*time.Second).Err()
}

func (rm *RedisMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	var agents []AgentPresence
	var keys []string
	var cursor uint64
	for {
		scanKeys, nextCursor, err := rm.client.Scan(ctx, cursor, "presence:*", 100).Result()
		if err != nil {
			return nil, err
		}
		keys = append(keys, scanKeys...)
		cursor = nextCursor
		if cursor == 0 {
			break
		}
	}

	for _, key := range keys {
		status, err := rm.client.Get(ctx, key).Result()
		if err == nil {
			agentID := key[len("presence:"):]
			agents = append(agents, AgentPresence{
				AgentID: agentID,
				Status:  status,
			})
		}
	}

	return agents, nil
}
