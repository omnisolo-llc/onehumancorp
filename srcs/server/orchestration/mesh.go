package orchestration

import (
	"context"
	"encoding/json"
	"net/http"
	"sync"
	"time"

	"onehumancorp/srcs/server/pb"
	"github.com/redis/rueidis"
	"github.com/centrifugal/centrifuge"

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
	GetHTTPHandler() http.Handler
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

// MemoryMeshTransport implements MeshTransport for standalone operation using Go channels.
type MemoryMeshTransport struct {
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

// NewMemoryMeshTransport creates a new MemoryMeshTransport.
func NewMemoryMeshTransport() *MemoryMeshTransport {
	m := &MemoryMeshTransport{}
	for i := 0; i < numShards; i++ {
		m.shards[i] = &meshShard{
			subscribers: make(map[string][]subscriber),
		}
	}
	return m
}

func (m *MemoryMeshTransport) GetHTTPHandler() http.Handler {
	return nil // Memory mesh uses basic websocket upgrader logic in the handler
}

// Publish sends data to all subscribers of the given channel concurrently.
func (m *MemoryMeshTransport) Publish(ctx context.Context, channel string, data []byte) error {
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
func (m *MemoryMeshTransport) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
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

func (m *MemoryMeshTransport) AdvertiseCapabilities(ctx context.Context, agent pb.Agent) error {
	m.registry.Store(agent.ID, agent)
	return nil
}

func (m *MemoryMeshTransport) DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error) {
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

func (m *MemoryMeshTransport) StartHeartbeat(ctx context.Context, agent pb.Agent) {
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

// RedisMeshTransport implements MeshTransport using rueidis and Centrifugo primitives.
type RedisMeshTransport struct {
	client rueidis.Client
	node   *centrifuge.Node
}

// NewRedisMeshTransport creates a new RedisMeshTransport.
func NewRedisMeshTransport(redisURL string) (*RedisMeshTransport, error) {
	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress: []string{redisURL},
	})
	if err != nil {
		return nil, err
	}

	node, err := centrifuge.New(centrifuge.Config{})
	if err != nil {
		return nil, err
	}
	// Redis engine setup goes here

	node.OnConnecting(func(ctx context.Context, e centrifuge.ConnectEvent) (centrifuge.ConnectReply, error) {
		return centrifuge.ConnectReply{}, nil
	})

	if err := node.Run(); err != nil {
		return nil, err
	}


	return &RedisMeshTransport{
		client: client,
		node:   node,
	}, nil
}

func (m *RedisMeshTransport) GetHTTPHandler() http.Handler {
	return centrifuge.NewWebsocketHandler(m.node, centrifuge.WebsocketConfig{})
}

// Publish publishes a message via Redis Pub/Sub.
func (m *RedisMeshTransport) Publish(ctx context.Context, channel string, data []byte) error {

	_, err := m.node.Publish(channel, data)
	cmd := m.client.B().Publish().Channel(channel).Message(string(data)).Build()

	m.client.Do(ctx, cmd)
	return err
}


// Subscribe subscribes to a channel via Redis Pub/Sub.
func (m *RedisMeshTransport) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	go func() {
		err := m.client.Receive(ctx, m.client.B().Subscribe().Channel(channel).Build(), func(msg rueidis.PubSubMessage) {
			handler([]byte(msg.Message))
		})
		if err != nil {
			// Handle error if necessary
		}
	}()
	return nil
}

func (m *RedisMeshTransport) AdvertiseCapabilities(ctx context.Context, agent pb.Agent) error {
	data, err := json.Marshal(agent)
	if err != nil {
		return err
	}
	cmd := m.client.B().Hset().Key("mesh:capabilities").FieldValue().FieldValue(agent.ID, string(data)).Build()
	return m.client.Do(ctx, cmd).Error()

}

func (m *RedisMeshTransport) DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error) {
	cmd := m.client.B().Hgetall().Key("mesh:capabilities").Build()
	res, err := m.client.Do(ctx, cmd).AsStrMap()
	if err != nil {
		return nil, err
	}

	var agents []pb.Agent
	for _, data := range res {
		var agent pb.Agent
		if err := json.Unmarshal([]byte(data), &agent); err == nil {
			for _, cap := range agent.Capabilities {
				if cap == skill {
					agents = append(agents, agent)
					break
				}
			}
		}
	}
	return agents, nil
}

func (m *RedisMeshTransport) StartHeartbeat(ctx context.Context, agent pb.Agent) {
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
