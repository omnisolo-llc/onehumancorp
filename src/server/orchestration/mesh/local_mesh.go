package mesh

import (
	"context"
	"sync"
	"time"
)

type localSubscription struct {
	mesh    *LocalMesh
	topic   string
	ch      chan []byte
	handler func(msg []byte)
	ctx     context.Context
	cancel  context.CancelFunc
}

func (s *localSubscription) Close() error {
	s.cancel()
	s.mesh.unsubscribe(s.topic, s)
	return nil
}

type presenceEntry struct {
	AgentPresence
	ExpiresAt time.Time
}

type LocalMesh struct {
	mu          sync.RWMutex
	subscribers map[string]map[*localSubscription]struct{}
	locks       sync.Mutex
	activeLocks map[string]time.Time
	presences   map[string]presenceEntry
}

func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		subscribers: make(map[string]map[*localSubscription]struct{}),
		activeLocks: make(map[string]time.Time),
		presences:   make(map[string]presenceEntry),
	}
}

func (m *LocalMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	m.mu.RLock()
	defer m.mu.RUnlock()

	subs, ok := m.subscribers[topic]
	if !ok {
		return nil
	}

	for sub := range subs {
		select {
		case sub.ch <- payload:
		case <-ctx.Done():
			return ctx.Err()
		default:
			// Dropping message if channel is full in this simple implementation
			// To ensure parity and not drop messages under normal load, wait briefly or increase channel size.
			// Try with a timer to avoid pile-up.
			timer := time.NewTimer(10 * time.Millisecond)
			select {
			case sub.ch <- payload:
				timer.Stop()
			case <-timer.C:
				// drop if still full
			}
		}
	}
	return nil
}

func (m *LocalMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	subCtx, cancel := context.WithCancel(ctx)
	sub := &localSubscription{
		mesh:    m,
		topic:   topic,
		ch:      make(chan []byte, 1000), // Increased channel buffer to handle burst
		handler: handler,
		ctx:     subCtx,
		cancel:  cancel,
	}

	m.mu.Lock()
	if m.subscribers[topic] == nil {
		m.subscribers[topic] = make(map[*localSubscription]struct{})
	}
	m.subscribers[topic][sub] = struct{}{}
	m.mu.Unlock()

	go func() {
		for {
			select {
			case msg := <-sub.ch:
				sub.handler(msg)
			case <-subCtx.Done():
				m.unsubscribe(topic, sub)
				return
			}
		}
	}()

	return sub, nil
}

func (m *LocalMesh) unsubscribe(topic string, sub *localSubscription) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if subs, ok := m.subscribers[topic]; ok {
		delete(subs, sub)
		if len(subs) == 0 {
			delete(m.subscribers, topic)
		}
	}
}

func (m *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	m.locks.Lock()
	defer m.locks.Unlock()

	now := time.Now()
	if expiry, ok := m.activeLocks[key]; ok {
		if now.Before(expiry) {
			return false, nil // Lock is held
		}
	}

	m.activeLocks[key] = now.Add(ttl)
	return true, nil
}

func (m *LocalMesh) ReleaseLock(ctx context.Context, key string) error {
	m.locks.Lock()
	defer m.locks.Unlock()

	delete(m.activeLocks, key)
	return nil
}

func (m *LocalMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.presences[agentID] = presenceEntry{
		AgentPresence: AgentPresence{AgentID: agentID, Status: status},
		ExpiresAt:     time.Now().Add(30 * time.Second),
	}
	return nil
}

func (m *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	var agents []AgentPresence
	now := time.Now()
	for id, p := range m.presences {
		if now.Before(p.ExpiresAt) {
			agents = append(agents, p.AgentPresence)
		} else {
			delete(m.presences, id)
		}
	}
	return agents, nil
}
