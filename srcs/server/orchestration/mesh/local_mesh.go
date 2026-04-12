package mesh

import (
	"context"
	"sync"
	"time"
)

type LocalMesh struct {
	mu       sync.RWMutex
	topics   map[string][]func(msg []byte)
	locks    map[string]time.Time
	presence map[string]AgentPresence
}

func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		topics:   make(map[string][]func(msg []byte)),
		locks:    make(map[string]time.Time),
		presence: make(map[string]AgentPresence),
	}
}

type localSubscription struct {
	mesh  *LocalMesh
	topic string
	idx   int
}

func (s *localSubscription) Unsubscribe() error {
	s.mesh.mu.Lock()
	defer s.mesh.mu.Unlock()
	subs := s.mesh.topics[s.topic]
	if s.idx >= 0 && s.idx < len(subs) {
		s.mesh.topics[s.topic] = append(subs[:s.idx], subs[s.idx+1:]...)
	}
	return nil
}

func (m *LocalMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	m.mu.RLock()
	subs := m.topics[topic]
	m.mu.RUnlock()

	for _, handler := range subs {
		go handler(payload)
	}
	return nil
}

func (m *LocalMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.topics[topic] = append(m.topics[topic], handler)
	return &localSubscription{mesh: m, topic: topic, idx: len(m.topics[topic]) - 1}, nil
}

func (m *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	expiresAt, exists := m.locks[key]
	if exists && time.Now().Before(expiresAt) {
		return false, nil
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
		AgentID:  agentID,
		Status:   status,
		LastSeen: time.Now(),
	}
	return nil
}

func (m *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()
	var agents []AgentPresence
	for _, p := range m.presence {
		if time.Since(p.LastSeen) < 10*time.Minute {
			agents = append(agents, p)
		}
	}
	return agents, nil
}
