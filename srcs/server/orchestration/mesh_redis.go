package orchestration

import (
	"context"
	"encoding/json"
	"time"

	"github.com/redis/go-redis/v9"

	"onehumancorp/srcs/server/pb"
)

// RedisMeshTransport implements MeshTransport for Cloud operation using Redis Pub/Sub.
type RedisMeshTransport struct {
	client redis.UniversalClient
}

// NewRedisMeshTransport creates a new RedisMeshTransport.
func NewRedisMeshTransport(client redis.UniversalClient) *RedisMeshTransport {
	return &RedisMeshTransport{
		client: client,
	}
}

// Publish sends data to all subscribers of the given channel via Redis.
func (m *RedisMeshTransport) Publish(ctx context.Context, channel string, data []byte) error {
	return m.client.Publish(ctx, channel, data).Err()
}

// Subscribe registers a handler for the given channel using Redis PubSub. Unsubscribes when ctx is done.
func (m *RedisMeshTransport) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	pubsub := m.client.Subscribe(ctx, channel)

	// Ensure subscription is active
	_, err := pubsub.Receive(ctx)
	if err != nil {
		return err
	}

	go func() {
		ch := pubsub.Channel()
		for {
			select {
			case <-ctx.Done():
				pubsub.Close()
				return
			case msg, ok := <-ch:
				if !ok {
					return
				}
				handler([]byte(msg.Payload))
			}
		}
	}()

	return nil
}

// AdvertiseCapabilities stores the agent capabilities in Redis with a TTL.
func (m *RedisMeshTransport) AdvertiseCapabilities(ctx context.Context, agent pb.Agent) error {
	data, err := json.Marshal(agent)
	if err != nil {
		return err
	}
	// Store in a hash or as individual keys with TTL
	// Let's use individual keys for easy TTL management
	key := "mesh:agents:" + agent.ID
	return m.client.Set(ctx, key, data, 30*time.Second).Err()
}

// DiscoverAgents queries Redis to find agents with the required capabilities.
func (m *RedisMeshTransport) DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error) {
	keys, err := m.client.Keys(ctx, "mesh:agents:*").Result()
	if err != nil {
		return nil, err
	}

	var agents []pb.Agent
	for _, key := range keys {
		data, err := m.client.Get(ctx, key).Result()
		if err != nil {
			continue // skip errors or expired keys
		}

		var agent pb.Agent
		if err := json.Unmarshal([]byte(data), &agent); err != nil {
			continue
		}

		for _, cap := range agent.Capabilities {
			if cap == skill {
				agents = append(agents, agent)
				break
			}
		}
	}

	return agents, nil
}

// StartHeartbeat periodically updates agent status/capabilities in Redis.
func (m *RedisMeshTransport) StartHeartbeat(ctx context.Context, agent pb.Agent) {
	ticker := time.NewTicker(10 * time.Second)
	go func() {
		defer ticker.Stop()
		// Initial advertisement
		m.AdvertiseCapabilities(ctx, agent)
		for {
			select {
			case <-ctx.Done():
				// Optional: Remove agent key on shutdown
				m.client.Del(context.Background(), "mesh:agents:"+agent.ID)
				return
			case <-ticker.C:
				m.AdvertiseCapabilities(ctx, agent)
			}
		}
	}()
}
