package mesh

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"errors"
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

type LocalMesh struct {
	mu          sync.RWMutex
	subscribers map[string]map[*localSubscription]struct{}
	locks       sync.Mutex
	activeLocks map[string]time.Time
	lockTokens  map[string]string
	presences   map[string]AgentPresence
}

func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		subscribers: make(map[string]map[*localSubscription]struct{}),
		activeLocks: make(map[string]time.Time),
		lockTokens:  make(map[string]string),
		presences:   make(map[string]AgentPresence),
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
		}
	}
	return nil
}

func (m *LocalMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	subCtx, cancel := context.WithCancel(ctx)
	sub := &localSubscription{
		mesh:    m,
		topic:   topic,
		ch:      make(chan []byte, 100),
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

func (m *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (string, bool, error) {
	m.locks.Lock()
	defer m.locks.Unlock()

	now := time.Now()
	if expiry, ok := m.activeLocks[key]; ok {
		if now.Before(expiry) {
			return "", false, nil // Lock is held
		}
	}

	b := make([]byte, 16)
	_, _ = rand.Read(b)
	token := hex.EncodeToString(b)
	m.activeLocks[key] = now.Add(ttl)
	m.lockTokens[key] = token
	return token, true, nil
}

func (m *LocalMesh) ReleaseLock(ctx context.Context, key string, token string) error {
	m.locks.Lock()
	defer m.locks.Unlock()

	if expectedToken, ok := m.lockTokens[key]; ok && expectedToken == token {
		delete(m.activeLocks, key)
		delete(m.lockTokens, key)
		return nil
	}
	return errors.New("lock not found or invalid token")
}

func (m *LocalMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.presences[agentID] = AgentPresence{AgentID: agentID, Status: status}
	return nil
}

func (m *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()

	var agents []AgentPresence
	for _, p := range m.presences {
		agents = append(agents, p)
	}
	return agents, nil
}
