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
	// Test connection
	if err := client.Ping(context.Background()).Err(); err != nil {
		return nil, err
	}
	return &RedisMesh{client: client}, nil
}

type redisSubscription struct {
	pubsub *redis.PubSub
}

func (s *redisSubscription) Unsubscribe() error {
	return s.pubsub.Close()
}

func (rm *RedisMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	return rm.client.Publish(ctx, topic, payload).Err()
}

func (rm *RedisMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	pubsub := rm.client.Subscribe(ctx, topic)
	_, err := pubsub.Receive(ctx)
	if err != nil {
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

func (rm *RedisMesh) AcquireLock(ctx context.Context, key string, token string, ttl time.Duration) (bool, error) {
	return rm.client.SetNX(ctx, "lock:"+key, token, ttl).Result()
}

const releaseLockScript = `
if redis.call("get", KEYS[1]) == ARGV[1] then
    return redis.call("del", KEYS[1])
else
    return 0
end
`

func (rm *RedisMesh) ReleaseLock(ctx context.Context, key string, token string) error {
	return rm.client.Eval(ctx, releaseLockScript, []string{"lock:"+key}, token).Err()
}

func (rm *RedisMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	presence := AgentPresence{
		AgentID:  agentID,
		Status:   status,
		LastSeen: time.Now().UTC(),
	}
	data, err := json.Marshal(presence)
	if err != nil {
		return err
	}
	// TTL for presence, assuming heartbeat every few seconds
	return rm.client.Set(ctx, "presence:"+agentID, data, 10*time.Second).Err()
}

func (rm *RedisMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	var cursor uint64
	var keys []string
	var err error
	for {
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
	if len(keys) == 0 {
		return agents, nil
	}

	res, err := rm.client.MGet(ctx, keys...).Result()
	if err != nil {
		return nil, err
	}
	for _, v := range res {
		if strData, ok := v.(string); ok {
			var presence AgentPresence
			if err := json.Unmarshal([]byte(strData), &presence); err == nil {
				agents = append(agents, presence)
			}
		}
	}
	return agents, nil
}
