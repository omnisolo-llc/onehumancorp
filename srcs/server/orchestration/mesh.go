package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"sync"
	"time"

	"onehumancorp/srcs/server/pb"
)

// MeshMessage represents an OHC-SIP compliant message over the mesh.
type MeshMessage struct {
	AgentID   string           `json:"agent_id"`
	EventType string           `json:"event_type"`
	Data      *json.RawMessage `json:"data,omitempty"`
	Channel   string           `json:"channel,omitempty"`
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
}

// NewCentrifugeMesh creates a new CentrifugeMesh.
func NewCentrifugeMesh(baseURL string) *CentrifugeMesh {
	return &CentrifugeMesh{
		BaseURL:    baseURL,
		HTTPClient: &http.Client{Timeout: 5 * time.Second},
	}
}

// Publish is a stub for CentrifugeMesh.
func (m *CentrifugeMesh) Publish(ctx context.Context, channel string, data []byte) error {
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
