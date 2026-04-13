package mesh

import (
	"context"
	"sync"
	"time"
)

type localSubscription struct {
	ch     chan []byte
	cancel context.CancelFunc
}

func (s *localSubscription) Channel() <-chan []byte { return s.ch }
func (s *localSubscription) Close() error {
	s.cancel()
	return nil
}

type presenceEntry struct {
	status    string
	expiresAt time.Time
}

type LocalMesh struct {
	mu       sync.RWMutex
	channels map[string][]chan []byte

	lockMu sync.Mutex
	locks  map[string]time.Time

	presMu   sync.RWMutex
	presence map[string]presenceEntry
}

func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		channels: make(map[string][]chan []byte),
		locks:    make(map[string]time.Time),
		presence: make(map[string]presenceEntry),
	}
}

func (m *LocalMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	m.mu.RLock()
	defer m.mu.RUnlock()

	for _, ch := range m.channels[topic] {
		select {
		case ch <- payload:
		case <-ctx.Done():
			return ctx.Err()
		default:
		}
	}
	return nil
}

func (m *LocalMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	ch := make(chan []byte, 100)
	m.channels[topic] = append(m.channels[topic], ch)

	subCtx, cancel := context.WithCancel(ctx)

	if handler != nil {
		go func() {
			for {
				select {
				case msg, ok := <-ch:
					if !ok {
						return
					}
					handler(msg)
				case <-subCtx.Done():
					m.mu.Lock()
					subs := m.channels[topic]
					for i, sub := range subs {
						if sub == ch {
							m.channels[topic] = append(subs[:i], subs[i+1:]...)
							break
						}
					}
					m.mu.Unlock()
					return
				}
			}
		}()
	} else {
		go func() {
			<-subCtx.Done()
			m.mu.Lock()
			subs := m.channels[topic]
			for i, sub := range subs {
				if sub == ch {
					m.channels[topic] = append(subs[:i], subs[i+1:]...)
					break
				}
			}
			m.mu.Unlock()
			close(ch)
		}()
	}

	return &localSubscription{ch: ch, cancel: cancel}, nil
}

func (m *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	m.lockMu.Lock()
	defer m.lockMu.Unlock()

	if expires, ok := m.locks[key]; ok && time.Now().Before(expires) {
		return false, nil
	}
	m.locks[key] = time.Now().Add(ttl)
	return true, nil
}

func (m *LocalMesh) ReleaseLock(ctx context.Context, key string) error {
	m.lockMu.Lock()
	defer m.lockMu.Unlock()
	delete(m.locks, key)
	return nil
}

func (m *LocalMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	m.presMu.Lock()
	defer m.presMu.Unlock()
	m.presence[agentID] = presenceEntry{status: status, expiresAt: time.Now().Add(1 * time.Minute)}
	return nil
}

func (m *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	m.presMu.RLock()
	defer m.presMu.RUnlock()

	var agents []AgentPresence
	now := time.Now()
	for id, entry := range m.presence {
		if now.Before(entry.expiresAt) {
			agents = append(agents, AgentPresence{AgentID: id, Status: entry.status})
		}
	}
	return agents, nil
}
