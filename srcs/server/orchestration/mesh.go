package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"sync"
	"time"

	"github.com/redis/rueidis"
	"onehumancorp/srcs/server/pb"
	"onehumancorp/srcs/server/telemetry"
)

// MeshMessage represents an OHC-SIP compliant message over the mesh.
type MeshMessage struct {
	AgentID string           `json:"agent_id"`
	Action  string           `json:"action"`
	Status  string           `json:"status"`
	Channel string           `json:"channel,omitempty"`
	Payload *json.RawMessage `json:"payload,omitempty"`
}

// MeshTransport defines the interface for the highly available realtime communication layer.
type MeshTransport interface {
	Publish(ctx context.Context, channel string, data []byte) error
	Subscribe(ctx context.Context, channel string, handler func(data []byte)) error
	AdvertiseCapabilities(ctx context.Context, agent pb.Agent) error
	DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error)
	StartHeartbeat(ctx context.Context, agent pb.Agent)
}

type subscriber struct {
	id      int
	handler func(data []byte)
}

const numShards = 32

type meshShard struct {
	mu          sync.RWMutex
	subscribers map[string][]subscriber
}

// LocalTeammateMesh implements MeshTransport for standalone operation using Go channels.
type LocalTeammateMesh struct {
	shards   [numShards]*meshShard
	nextID   int
	idMu     sync.Mutex // lock just for generating subscriber IDs
	registry sync.Map   // map[string]pb.Agent
}

func getShard(channel string) int {
	var hash uint32 = 2166136261
	for i := 0; i < len(channel); i++ {
		hash ^= uint32(channel[i])
		hash *= 16777619
	}
	return int(hash % numShards)
}

// NewLocalTeammateMesh creates a new LocalTeammateMesh.
func NewLocalTeammateMesh() *LocalTeammateMesh {
	m := &LocalTeammateMesh{}
	for i := 0; i < numShards; i++ {
		m.shards[i] = &meshShard{
			subscribers: make(map[string][]subscriber),
		}
	}
	return m
}

// Publish sends data to all subscribers of the given channel concurrently.
func (m *LocalTeammateMesh) Publish(ctx context.Context, channel string, data []byte) error {
	if telemetry.MeshBroadcastTotal != nil {
		telemetry.MeshBroadcastTotal.WithLabelValues("standalone").Inc()
	}

	shard := m.shards[getShard(channel)]
	shard.mu.RLock()
	subs, ok := shard.subscribers[channel]
	if !ok {
		shard.mu.RUnlock()
		return nil
	}
	// Copy subs to avoid holding lock while dispatching
	subsCopy := make([]subscriber, len(subs))
	copy(subsCopy, subs)
	shard.mu.RUnlock()

	dataCopy := make([]byte, len(data))
	copy(dataCopy, data)

	for _, sub := range subsCopy {
		go sub.handler(dataCopy)
	}

	return nil
}

// Subscribe registers a handler for the given channel. Unsubscribes when ctx is done.
func (m *LocalTeammateMesh) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	m.idMu.Lock()
	id := m.nextID
	m.nextID++
	m.idMu.Unlock()

	shard := m.shards[getShard(channel)]

	shard.mu.Lock()
	shard.subscribers[channel] = append(shard.subscribers[channel], subscriber{id: id, handler: handler})
	shard.mu.Unlock()

	go func() {
		<-ctx.Done()
		shard.mu.Lock()
		defer shard.mu.Unlock()
		subs := shard.subscribers[channel]
		for i, sub := range subs {
			if sub.id == id {
				shard.subscribers[channel] = append(subs[:i], subs[i+1:]...)
				break
			}
		}
	}()

	return nil
}

func (m *LocalTeammateMesh) AdvertiseCapabilities(ctx context.Context, agent pb.Agent) error {
	m.registry.Store(agent.ID, agent)
	return nil
}

func (m *LocalTeammateMesh) DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error) {
	var agents []pb.Agent
	m.registry.Range(func(key, value interface{}) bool {
		agent := value.(pb.Agent)
		for _, cap := range agent.Capabilities {
			if cap == skill {
				agents = append(agents, agent)
				break
			}
		}
		return true
	})
	return agents, nil
}

func (m *LocalTeammateMesh) StartHeartbeat(ctx context.Context, agent pb.Agent) {
	ticker := time.NewTicker(10 * time.Second)
	go func() {
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				m.AdvertiseCapabilities(ctx, agent)
			}
		}
	}()
}

// CentrifugeMesh implements MeshTransport using rueidis and Centrifugo primitives.
// This is currently a stub for cloud-native setup.
type CentrifugeMesh struct {
	BaseURL    string
	HTTPClient *http.Client
	pubClient  rueidis.Client
	subClient  rueidis.Client
}

// NewCentrifugeMesh creates a new CentrifugeMesh.
func NewCentrifugeMesh(baseURL string) *CentrifugeMesh {
	// Simple default redis address. In production this should be configured via environment or options.
	pub, err := rueidis.NewClient(rueidis.ClientOption{InitAddress: []string{"127.0.0.1:6379"}})
	if err != nil {
		fmt.Printf("Failed to connect pub client to redis: %v\n", err)
	}
	sub, err := rueidis.NewClient(rueidis.ClientOption{InitAddress: []string{"127.0.0.1:6379"}})
	if err != nil {
		fmt.Printf("Failed to connect sub client to redis: %v\n", err)
	}

	return &CentrifugeMesh{
		BaseURL:    baseURL,
		HTTPClient: &http.Client{Timeout: 5 * time.Second},
		pubClient:  pub,
		subClient:  sub,
	}
}

// Publish is a stub for CentrifugeMesh.
func (m *CentrifugeMesh) Publish(ctx context.Context, channel string, data []byte) error {
	if telemetry.MeshBroadcastTotal != nil {
		telemetry.MeshBroadcastTotal.WithLabelValues("cloud").Inc()
	}

	if m.pubClient != nil {
		cmd := m.pubClient.B().Publish().Channel(channel).Message(string(data)).Build()
		err := m.pubClient.Do(context.Background(), cmd).Error()
		if err != nil {
			return err
		}
	}

	req, err := http.NewRequestWithContext(ctx, "POST", m.BaseURL, bytes.NewBuffer(data))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := m.HTTPClient.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 400 {
		return fmt.Errorf("publish failed with status: %d", resp.StatusCode)
	}

	return nil
}

// Subscribe is a stub for CentrifugeMesh.
func (m *CentrifugeMesh) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	if m.subClient == nil {
		return fmt.Errorf("redis sub client not initialized")
	}

	go func() {
		_ = m.subClient.Receive(ctx, m.subClient.B().Subscribe().Channel(channel).Build(), func(msg rueidis.PubSubMessage) {
			handler([]byte(msg.Message))
		})
	}()
	return nil
}

func (m *CentrifugeMesh) AdvertiseCapabilities(ctx context.Context, agent pb.Agent) error {
	// Stub for cloud-native setup
	return nil
}

func (m *CentrifugeMesh) DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error) {
	// Stub for cloud-native setup
	return nil, nil
}

func (m *CentrifugeMesh) StartHeartbeat(ctx context.Context, agent pb.Agent) {
	ticker := time.NewTicker(10 * time.Second)
	go func() {
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				m.AdvertiseCapabilities(ctx, agent)
			}
		}
	}()
}
