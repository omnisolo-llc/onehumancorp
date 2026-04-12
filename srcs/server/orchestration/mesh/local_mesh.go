package mesh

import (
	"context"
	"sync"
	"time"
)

type LocalSubscription struct {
	topic string
	id    int
	mesh  *LocalMesh
}

func (s *LocalSubscription) Unsubscribe() error {
	s.mesh.mu.Lock()
	defer s.mesh.mu.Unlock()
	if subs, ok := s.mesh.subs[s.topic]; ok {
		delete(subs, s.id)
	}
	return nil
}

type LocalMesh struct {
	mu           sync.RWMutex
	subs         map[string]map[int]func([]byte)
	nextSubID    int
	locks        map[string]time.Time
	presence     map[string]AgentPresence
	presenceTime map[string]time.Time
}

func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		subs:         make(map[string]map[int]func([]byte)),
		locks:        make(map[string]time.Time),
		presence:     make(map[string]AgentPresence),
		presenceTime: make(map[string]time.Time),
	}
}

func (m *LocalMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	m.mu.RLock()
	defer m.mu.RUnlock()
	if subs, ok := m.subs[topic]; ok {
		for _, handler := range subs {
			// Copy payload to avoid data races
			pCopy := make([]byte, len(payload))
			copy(pCopy, payload)
			go handler(pCopy)
		}
	}
	return nil
}

func (m *LocalMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if _, ok := m.subs[topic]; !ok {
		m.subs[topic] = make(map[int]func([]byte))
	}
	subID := m.nextSubID
	m.nextSubID++
	m.subs[topic][subID] = handler
	return &LocalSubscription{topic: topic, id: subID, mesh: m}, nil
}

func (m *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	now := time.Now()
	if expiry, exists := m.locks[key]; exists && now.Before(expiry) {
		return false, nil
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
	m.presence[agentID] = AgentPresence{AgentID: agentID, Status: status}
	m.presenceTime[agentID] = time.Now()
	return nil
}

func (m *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()
	var active []AgentPresence
	now := time.Now()
	for id, p := range m.presence {
		// Assume agent is active if it pinged within last 30s
		if now.Sub(m.presenceTime[id]) < 30*time.Second {
			active = append(active, p)
		}
	}
	return active, nil
}
