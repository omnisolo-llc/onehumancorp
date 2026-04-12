package mesh

import (
	"context"
	"sync"
	"time"
)

// LocalMesh implements TeammateMesh for standalone/SQLite mode.
type LocalMesh struct {
	mu            sync.RWMutex
	subscriptions map[string][]*localSubscription
	locks         map[string]time.Time
	presence      map[string]AgentPresence
}

type localSubscription struct {
	topic   string
	handler func(msg []byte)
	mesh    *LocalMesh
}

func (s *localSubscription) Unsubscribe(ctx context.Context) error {
	s.mesh.mu.Lock()
	defer s.mesh.mu.Unlock()

	subs := s.mesh.subscriptions[s.topic]
	for i, sub := range subs {
		if sub == s {
			s.mesh.subscriptions[s.topic] = append(subs[:i], subs[i+1:]...)
			break
		}
	}
	return nil
}

// NewLocalMesh creates a new in-memory TeammateMesh.
func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		subscriptions: make(map[string][]*localSubscription),
		locks:         make(map[string]time.Time),
		presence:      make(map[string]AgentPresence),
	}
}

func (m *LocalMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	m.mu.RLock()
	subs := m.subscriptions[topic]
	handlers := make([]func(msg []byte), len(subs))
	for i, s := range subs {
		handlers[i] = s.handler
	}
	m.mu.RUnlock()

	for _, handler := range handlers {
		go handler(payload)
	}

	return nil
}

func (m *LocalMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	sub := &localSubscription{
		topic:   topic,
		handler: handler,
		mesh:    m,
	}

	m.subscriptions[topic] = append(m.subscriptions[topic], sub)

	return sub, nil
}

func (m *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	now := time.Now()
	if exp, exists := m.locks[key]; exists {
		if now.Before(exp) {
			return false, nil // Lock is held
		}
	}

	m.locks[key] = now.Add(ttl)
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
		AgentID:   agentID,
		Status:    status,
		UpdatedAt: time.Now(),
	}
	return nil
}

func (m *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	now := time.Now()
	agents := make([]AgentPresence, 0)
	for id, p := range m.presence {
		// Clean up stale agents (older than 5 minutes)
		if now.Sub(p.UpdatedAt) > 5*time.Minute {
			delete(m.presence, id)
			continue
		}
		agents = append(agents, p)
	}
	return agents, nil
}
