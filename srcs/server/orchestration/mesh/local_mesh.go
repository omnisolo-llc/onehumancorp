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

func (s *localSubscription) Unsubscribe() error {
	s.mesh.mu.Lock()
	defer s.mesh.mu.Unlock()
	if subs, ok := s.mesh.subscribers[s.topic]; ok {
		delete(subs, s.id)
		if len(subs) == 0 {
			delete(s.mesh.subscribers, s.topic)
		}
	}
	return nil
}

type LocalMesh struct {
	mu          sync.RWMutex
	subscribers map[string]map[int]func([]byte)
	nextSubID   int
	locks       map[string]time.Time
	presence    map[string]AgentPresence
}

func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		subscribers: make(map[string]map[int]func([]byte)),
		locks:       make(map[string]time.Time),
		presence:    make(map[string]AgentPresence),
	}
}

func (m *LocalMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	m.mu.RLock()
	subs, ok := m.subscribers[topic]
	var handlers []func([]byte)
	if ok {
		for _, handler := range subs {
			handlers = append(handlers, handler)
		}
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

	if _, ok := m.subscribers[topic]; !ok {
		m.subscribers[topic] = make(map[int]func([]byte))
	}
	m.nextSubID++
	id := m.nextSubID
	m.subscribers[topic][id] = handler

	return &localSubscription{topic: topic, id: id, mesh: m}, nil
}

func (m *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	if expires, ok := m.locks[key]; ok {
		if time.Now().Before(expires) {
			return false, nil // lock is currently held
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

	m.presence[agentID] = AgentPresence{AgentID: agentID, Status: status}
	return nil
}

func (m *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()

	var active []AgentPresence
	for _, p := range m.presence {
		active = append(active, p)
	}
	return active, nil
}
