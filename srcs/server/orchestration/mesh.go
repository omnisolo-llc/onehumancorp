package orchestration


import (
	"context"
	"encoding/json"
	"hash/fnv"
	"sync"
	"time"
)

// Agent represents an AI agent in the mesh.
type Agent struct {
	ID    string   `json:"agent_id"`
	Role  string   `json:"role"`
	Skills []string `json:"skills"`
}

// MeshMessage represents a realtime message sent across the mesh.
// OHC-SIP compliant root fields.
type MeshMessage struct {
	AgentID string          `json:"agent_id"`
	Action  string          `json:"action"`
	Status  string          `json:"status"`
	Payload json.RawMessage `json:"payload"`
	MsgID   string          `json:"msg_id"`
}

// Task represents a task in the mesh.
type Task struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
}

// AgentRegistry maintains a list of active agents in the mesh.
type AgentRegistry struct {
	mu     sync.RWMutex
	agents map[string]*agentEntry
	cancel context.CancelFunc
}

type agentEntry struct {
	agent      Agent
	lastSeenAt time.Time
}

func NewAgentRegistry() *AgentRegistry {
	ctx, cancel := context.WithCancel(context.Background())
	r := &AgentRegistry{
		agents: make(map[string]*agentEntry),
		cancel: cancel,
	}
	// Start heartbeat checker
	go r.heartbeatMonitor(ctx)
	return r
}

func (r *AgentRegistry) Stop() {
	if r.cancel != nil {
		r.cancel()
	}
}

func (r *AgentRegistry) heartbeatMonitor(ctx context.Context) {
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			r.mu.Lock()
			now := time.Now()
			for id, entry := range r.agents {
				if now.Sub(entry.lastSeenAt) > 2*time.Minute {
					delete(r.agents, id)
				}
			}
			r.mu.Unlock()
		}
	}
}

func (r *AgentRegistry) register(agent Agent) {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.agents[agent.ID] = &agentEntry{
		agent:      agent,
		lastSeenAt: time.Now(),
	}
}

func (r *AgentRegistry) discover(skill string) []Agent {
	r.mu.RLock()
	defer r.mu.RUnlock()
	var matched []Agent
	for _, entry := range r.agents {
		for _, s := range entry.agent.Skills {
			if s == skill {
				matched = append(matched, entry.agent)
				break
			}
		}
	}
	return matched
}


// MeshHub defines the interface for the highly available realtime communication layer.
type MeshHub interface {
	Publish(ctx context.Context, channel string, data []byte) error
	Subscribe(ctx context.Context, channel string, handler func(data []byte)) error
	AdvertiseCapabilities(ctx context.Context, agent Agent) error
	DiscoverAgents(ctx context.Context, skill string) ([]Agent, error)
}

type subscriber struct {
	id      int
	handler func(data []byte)
}


type shard struct {
	mu          sync.RWMutex
	subscribers map[string][]subscriber
}

// LocalTeammateMesh implements MeshHub for standalone operation using Go channels.
// It uses sharding to match Redis performance characteristics.
type LocalTeammateMesh struct {
	shards   [32]*shard
	registry *AgentRegistry
	nextID   int
	idMu     sync.Mutex
}

func getShardIndex(channel string) uint32 {
	h := fnv.New32a()
	h.Write([]byte(channel))
	return h.Sum32() % 32
}

// NewLocalTeammateMesh creates a new LocalTeammateMesh.
func NewLocalTeammateMesh() *LocalTeammateMesh {
	m := &LocalTeammateMesh{
		registry: NewAgentRegistry(),
	}
	for i := 0; i < 32; i++ {
		m.shards[i] = &shard{
			subscribers: make(map[string][]subscriber),
		}
	}
	return m
}

// Publish sends data to all subscribers of the given channel concurrently.
func (m *LocalTeammateMesh) Publish(ctx context.Context, channel string, data []byte) error {
	shardIdx := getShardIndex(channel)
	s := m.shards[shardIdx]

	s.mu.RLock()
	subs, ok := s.subscribers[channel]
	if !ok {
		s.mu.RUnlock()
		return nil
	}
	subsCopy := make([]subscriber, len(subs))
	copy(subsCopy, subs)
	s.mu.RUnlock()

	dataCopy := make([]byte, len(data))
	copy(dataCopy, data)

	for _, sub := range subsCopy {
		go sub.handler(dataCopy)
	}

	return nil
}

// Subscribe registers a handler for the given channel. Unsubscribes when ctx is done.
func (m *LocalTeammateMesh) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	shardIdx := getShardIndex(channel)
	s := m.shards[shardIdx]

	m.idMu.Lock()
	id := m.nextID
	m.nextID++
	m.idMu.Unlock()

	s.mu.Lock()
	s.subscribers[channel] = append(s.subscribers[channel], subscriber{id: id, handler: handler})
	s.mu.Unlock()

	go func() {
		<-ctx.Done()
		s.mu.Lock()
		defer s.mu.Unlock()
		subs := s.subscribers[channel]
		for i, sub := range subs {
			if sub.id == id {
				s.subscribers[channel] = append(subs[:i], subs[i+1:]...)
				break
			}
		}
	}()

	return nil
}

func (m *LocalTeammateMesh) AdvertiseCapabilities(ctx context.Context, agent Agent) error {
	m.registry.register(agent)
	return nil
}

func (m *LocalTeammateMesh) DiscoverAgents(ctx context.Context, skill string) ([]Agent, error) {
	return m.registry.discover(skill), nil
}



// CentrifugeMesh implements MeshHub using rueidis and Centrifugo primitives.
// This is currently a stub for cloud-native setup.
type CentrifugeMesh struct {
}

// NewCentrifugeMesh creates a new CentrifugeMesh.
func NewCentrifugeMesh() *CentrifugeMesh {
	return &CentrifugeMesh{}
}


// Publish is a stub for CentrifugeMesh.
func (m *CentrifugeMesh) Publish(ctx context.Context, channel string, data []byte) error {
	return nil
}

// Subscribe is a stub for CentrifugeMesh.
func (m *CentrifugeMesh) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	return nil
}

func (m *CentrifugeMesh) AdvertiseCapabilities(ctx context.Context, agent Agent) error {
	return nil
}

func (m *CentrifugeMesh) DiscoverAgents(ctx context.Context, skill string) ([]Agent, error) {
	return nil, nil
}
