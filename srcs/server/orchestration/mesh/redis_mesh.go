package mesh

import (
	"context"
	"encoding/json"
	"time"

	"github.com/redis/rueidis"
)

// RedisMesh implements TeammateMesh for Cloud-Native mode.
type RedisMesh struct {
	client rueidis.Client
}

type redisSubscription struct {
	cancel func()
}

func (s *redisSubscription) Unsubscribe(ctx context.Context) error {
	s.cancel()
	return nil
}

// NewRedisMesh creates a new Redis-backed TeammateMesh.
func NewRedisMesh(client rueidis.Client) *RedisMesh {
	return &RedisMesh{
		client: client,
	}
}

func (m *RedisMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	cmd := m.client.B().Publish().Channel(topic).Message(string(payload)).Build()
	return m.client.Do(ctx, cmd).Error()
}

func (m *RedisMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	ctx, cancel := context.WithCancel(context.Background())
	err := m.client.Receive(ctx, m.client.B().Subscribe().Channel(topic).Build(), func(msg rueidis.PubSubMessage) {
		handler([]byte(msg.Message))
	})

	if err != nil && err != context.Canceled {
		cancel()
		return nil, err
	}

	return &redisSubscription{cancel: cancel}, nil
}

func (m *RedisMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	cmd := m.client.B().Set().Key("lock:"+key).Value("1").Nx().Px(ttl).Build()
	err := m.client.Do(ctx, cmd).Error()
	if err == rueidis.Nil {
		return false, nil
	}
	if err != nil {
		return false, err
	}
	return true, nil
}

func (m *RedisMesh) ReleaseLock(ctx context.Context, key string) error {
	cmd := m.client.B().Del().Key("lock:"+key).Build()
	return m.client.Do(ctx, cmd).Error()
}

func (m *RedisMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	p := AgentPresence{
		AgentID:   agentID,
		Status:    status,
		UpdatedAt: time.Now(),
	}
	data, err := json.Marshal(p)
	if err != nil {
		return err
	}
	cmd := m.client.B().Set().Key("presence:"+agentID).Value(string(data)).Ex(5*time.Minute).Build()
	return m.client.Do(ctx, cmd).Error()
}

func (m *RedisMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	var cursor uint64
	var agents []AgentPresence

	for {
		cmd := m.client.B().Scan().Cursor(cursor).Match("presence:*").Count(10).Build()
		resp, err := m.client.Do(ctx, cmd).AsScanEntry()
		if err != nil {
			return nil, err
		}

		cursor = resp.Cursor
		for _, key := range resp.Elements {
			getCmd := m.client.B().Get().Key(key).Build()
			val, err := m.client.Do(ctx, getCmd).ToString()
			if err != nil {
				if err == rueidis.Nil {
					continue
				}
				return nil, err
			}
			var p AgentPresence
			if err := json.Unmarshal([]byte(val), &p); err == nil {
				agents = append(agents, p)
			}
		}

		if cursor == 0 {
			break
		}
	}

	return agents, nil
}
