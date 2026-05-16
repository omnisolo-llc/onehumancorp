package kairos

import (
	"context"
	"sync"
	"time"
)

type Subscription interface {
	Unsubscribe() error
}

type AgentPresence struct {
	AgentID string
	Status  string
}

type TeammateMesh interface {
	Publish(ctx context.Context, topic string, payload []byte) error
	Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error)
	AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error)
	ReleaseLock(ctx context.Context, key string) error
	RegisterPresence(ctx context.Context, agentID string, status string) error
	GetActiveAgents(ctx context.Context) ([]AgentPresence, error)
	Acknowledge(ctx context.Context, messageID string) error
}

type localSubInfo struct {
	id       int64
	handler  func(msg []byte)
	cancelCh chan struct{}
}

type localSubscription struct {
	mesh  *LocalTeammateMesh
	topic string
	id    int64
}

func (s *localSubscription) Unsubscribe() error {
	s.mesh.mu.Lock()
	defer s.mesh.mu.Unlock()

	subs := s.mesh.subs[s.topic]
	for i, sub := range subs {
		if sub.id == s.id {
			close(sub.cancelCh)
			s.mesh.subs[s.topic] = append(subs[:i], subs[i+1:]...)
			break
		}
	}
	return nil
}

type LocalTeammateMesh struct {
	mu       sync.RWMutex
	subs     map[string][]localSubInfo
	locks    map[string]time.Time
	presence map[string]string
	nextID   int64
}

func NewLocalTeammateMesh() *LocalTeammateMesh {
	return &LocalTeammateMesh{
		subs:     make(map[string][]localSubInfo),
		locks:    make(map[string]time.Time),
		presence: make(map[string]string),
	}
}

func (m *LocalTeammateMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	m.mu.RLock()
	subs := m.subs[topic]
	subsCopy := make([]localSubInfo, len(subs))
	copy(subsCopy, subs)
	m.mu.RUnlock()

	dataCopy := make([]byte, len(payload))
	copy(dataCopy, payload)

	for _, sub := range subsCopy {
		go sub.handler(dataCopy)
	}

	return nil
}

func (m *LocalTeammateMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	id := m.nextID
	m.nextID++

	cancelCh := make(chan struct{})
	m.subs[topic] = append(m.subs[topic], localSubInfo{
		id:       id,
		handler:  handler,
		cancelCh: cancelCh,
	})

	return &localSubscription{
		mesh:  m,
		topic: topic,
		id:    id,
	}, nil
}

func (m *LocalTeammateMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	expire, exists := m.locks[key]
	if exists && time.Now().Before(expire) {
		return false, nil
	}

	m.locks[key] = time.Now().Add(ttl)
	return true, nil
}

func (m *LocalTeammateMesh) ReleaseLock(ctx context.Context, key string) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	delete(m.locks, key)
	return nil
}

func (m *LocalTeammateMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.presence[agentID] = status
	return nil
}

func (m *LocalTeammateMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()

	var agents []AgentPresence
	for k, v := range m.presence {
		agents = append(agents, AgentPresence{
			AgentID: k,
			Status:  v,
		})
	}
	return agents, nil
}

func (m *LocalTeammateMesh) Acknowledge(ctx context.Context, messageID string) error {
	return m.Publish(ctx, "mesh:ack:"+messageID, []byte("ack"))
}
