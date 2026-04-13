package mesh

import (
	"context"
	"time"

	"github.com/redis/go-redis/v9"
)

type redisSubscription struct {
	pubsub *redis.PubSub
	ch     chan []byte
}

func (s *redisSubscription) Channel() <-chan []byte { return s.ch }
func (s *redisSubscription) Close() error           { return s.pubsub.Close() }

type RedisMesh struct {
	client *redis.Client
}

func NewRedisMesh(redisURL string) (*RedisMesh, error) {
	opts, err := redis.ParseURL(redisURL)
	if err != nil {
		return nil, err
	}
	return &RedisMesh{client: redis.NewClient(opts)}, nil
}

func (m *RedisMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	return m.client.Publish(ctx, topic, payload).Err()
}

func (m *RedisMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	pubsub := m.client.Subscribe(ctx, topic)
	ch := make(chan []byte, 100)

	go func() {
		defer close(ch)
		for {
			select {
			case <-ctx.Done():
				pubsub.Close()
				return
			case msg, ok := <-pubsub.Channel():
				if !ok {
					return
				}
				if handler != nil {
					handler([]byte(msg.Payload))
				} else {
					select {
					case ch <- []byte(msg.Payload):
					case <-ctx.Done():
						pubsub.Close()
						return
					default:
					}
				}
			}
		}
	}()

	return &redisSubscription{pubsub: pubsub, ch: ch}, nil
}

func (m *RedisMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	return m.client.SetNX(ctx, "lock:"+key, "1", ttl).Result()
}

func (m *RedisMesh) ReleaseLock(ctx context.Context, key string) error {
	return m.client.Del(ctx, "lock:"+key).Err()
}

func (m *RedisMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	return m.client.Set(ctx, "presence:"+agentID, status, 1*time.Minute).Err()
}

func (m *RedisMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	var agents []AgentPresence
	var cursor uint64
	for {
		var keys []string
		var err error
		keys, cursor, err = m.client.Scan(ctx, cursor, "presence:*", 100).Result()
		if err != nil {
			return nil, err
		}
		for _, key := range keys {
			status, err := m.client.Get(ctx, key).Result()
			if err == nil {
				agents = append(agents, AgentPresence{AgentID: key[len("presence:"):], Status: status})
			}
		}
		if cursor == 0 {
			break
		}
	}
	return agents, nil
}
