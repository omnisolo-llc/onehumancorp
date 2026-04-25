package mesh

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"errors"
	"encoding/json"
	"time"

	"github.com/redis/go-redis/v9"
)

type redisSubscription struct {
	pubsub *redis.PubSub
	cancel context.CancelFunc
}

func (s *redisSubscription) Close() error {
	s.cancel()
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

	// Wait for confirmation that subscription is created before proceeding
	_, err := pubsub.Receive(ctx)
	if err != nil {
		pubsub.Close()
		return nil, err
	}

	subCtx, cancel := context.WithCancel(ctx)
	ch := pubsub.Channel()

	go func() {
		for {
			select {
			case msg, ok := <-ch:
				if !ok {
					return
				}
				handler([]byte(msg.Payload))
			case <-subCtx.Done():
				return
			}
		}
	}()

	return &redisSubscription{
		pubsub: pubsub,
		cancel: cancel,
	}, nil
}

func (m *RedisMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (string, bool, error) {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	token := hex.EncodeToString(b)

	ok, err := m.client.SetNX(ctx, "lock:"+key, token, ttl).Result()
	if err != nil {
		return "", false, err
	}
	if !ok {
		return "", false, nil
	}

	return token, true, nil
}

func (m *RedisMesh) ReleaseLock(ctx context.Context, key string, token string) error {
	const script = `
		if redis.call("get", KEYS[1]) == ARGV[1] then
			return redis.call("del", KEYS[1])
		else
			return 0
		end
	`

	res, err := m.client.Eval(ctx, script, []string{"lock:" + key}, token).Result()
	if err != nil {
		return err
	}

	if count, ok := res.(int64); ok && count == 1 {
		return nil
	}

	return errors.New("lock not found or invalid token")
}

func (m *RedisMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	// Use a sorted set or just set with TTL. For simplicity, we use Set with TTL here,
	// but to get all active agents efficiently, we can use a Hash + TTL logic, or standard Redis
	// keys with a prefix.
	// A robust way to track presences with TTL is setting an individual key per agent.
	err := m.client.Set(ctx, "presence:"+agentID, status, 30*time.Second).Err() // e.g. 30s heartbeat
	return err
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
			if err == redis.Nil {
				continue // Expired between scan and get
			} else if err != nil {
				return nil, err
			}

			agentID := key[len("presence:"):]
			agents = append(agents, AgentPresence{
				AgentID: agentID,
				Status:  status,
			})
		}

		if cursor == 0 {
			break
		}
	}

	return agents, nil
}

func (m *RedisMesh) SyncState(ctx context.Context, agentID string) error {
	status, err := m.client.Get(ctx, "presence:"+agentID).Result()
	if err == redis.Nil {
		return nil // No presence found to sync
	} else if err != nil {
		return err
	}

	presence := AgentPresence{AgentID: agentID, Status: status}
	payload, _ := json.Marshal(presence)
	return m.Publish(ctx, "sync:"+agentID, payload)
}
