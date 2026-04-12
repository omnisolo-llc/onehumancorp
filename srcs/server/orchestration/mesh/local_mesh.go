package mesh

import (
	"context"
	"sync"
	"time"
)

type localSubscription struct {
	topic string
	id    int
	mesh  *LocalMesh
}

func (s *localSubscription) Close() error {
	s.mesh.mu.Lock()
	defer s.mesh.mu.Unlock()
	if handlers, ok := s.mesh.subscriptions[s.topic]; ok {
		delete(handlers, s.id)
		if len(handlers) == 0 {
			delete(s.mesh.subscriptions, s.topic)
		}
	}
	return nil
}

type LocalMesh struct {
	mu            sync.RWMutex
	subscriptions map[string]map[int]func([]byte)
	nextSubID     int
	locks         map[string]time.Time
	presence      map[string]AgentPresence
}

func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		subscriptions: make(map[string]map[int]func([]byte)),
		locks:         make(map[string]time.Time),
		presence:      make(map[string]AgentPresence),
	}
}

func (m *LocalMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	m.mu.RLock()
	handlers, ok := m.subscriptions[topic]
	m.mu.RUnlock()

	if !ok {
		return nil
	}

	for _, handler := range handlers {
		go handler(payload)
	}

	return nil
}

func (m *LocalMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	if _, ok := m.subscriptions[topic]; !ok {
		m.subscriptions[topic] = make(map[int]func([]byte))
	}

	subID := m.nextSubID
	m.nextSubID++
	m.subscriptions[topic][subID] = handler

	return &localSubscription{
		topic: topic,
		id:    subID,
		mesh:  m,
	}, nil
}

func (m *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	if exp, ok := m.locks[key]; ok {
		if time.Now().Before(exp) {
			return false, nil
		}
	}

	m.locks[key] = time.Now().Add(ttl)
	return true, nil
}

func (m *LocalMesh) ReleaseLock(ctx context.Context, key string) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	delete(m.locks, key)
	return nil
}

func (m *LocalMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	m.presence[agentID] = AgentPresence{
		AgentID: agentID,
		Status:  status,
	}
	return nil
}

func (m *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()

	var agents []AgentPresence
	for _, p := range m.presence {
		agents = append(agents, p)
	}
	return agents, nil
}
