package mesh

import (
	"context"
	"fmt"
	"time"

	"github.com/redis/go-redis/v9"
)

type RedisMesh struct {
	client *redis.Client
}

func NewRedisMesh(redisURL string) (*RedisMesh, error) {
	opt, err := redis.ParseURL(redisURL)
	if err != nil {
		return nil, fmt.Errorf("invalid redis url: %w", err)
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

func (rs *redisSubscription) Unsubscribe() error {
	return rs.pubsub.Close()
}

func (rm *RedisMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	pubsub := rm.client.Subscribe(ctx, topic)
	_, err := pubsub.Receive(ctx)
	if err != nil {
		return nil, fmt.Errorf("subscribe failed: %w", err)
	}

	go func() {
		ch := pubsub.Channel()
		for msg := range ch {
			handler([]byte(msg.Payload))
		}
	}()

	return &redisSubscription{pubsub: pubsub}, nil
}

func (rm *RedisMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration, token string) (bool, error) {
	return rm.client.SetNX(ctx, "lock:"+key, token, ttl).Result()
}

func (rm *RedisMesh) ReleaseLock(ctx context.Context, key string, token string) error {
	script := `
if redis.call("get", KEYS[1]) == ARGV[1] then
    return redis.call("del", KEYS[1])
else
    return 0
end
`
	return rm.client.Eval(ctx, script, []string{"lock:"+key}, token).Err()
}

func (rm *RedisMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	return rm.client.HSet(ctx, "presence", agentID, status).Err()
}

func (rm *RedisMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	res, err := rm.client.HGetAll(ctx, "presence").Result()
	if err != nil {
		return nil, fmt.Errorf("failed to get active agents: %w", err)
	}

	var agents []AgentPresence
	for agentID, status := range res {
		agents = append(agents, AgentPresence{
			AgentID: agentID,
			Status:  status,
		})
	}
	return agents, nil
}

func (rm *RedisMesh) HandoffState(ctx context.Context, targetAgentID string, state []byte) error {
	return rm.Publish(ctx, "handoff:"+targetAgentID, state)
}

func (rm *RedisMesh) SubscribeHandoffs(ctx context.Context, agentID string, handler func(state []byte)) (Subscription, error) {
	return rm.Subscribe(ctx, "handoff:"+agentID, handler)
}
