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

type localLockInfo struct {
	expiry time.Time
	token  string
}

type LocalMesh struct {
	mu          sync.RWMutex
	subscribers map[string]map[*localSubscription]struct{}
	locks       sync.Mutex
	activeLocks map[string]localLockInfo
	presences   map[string]AgentPresence
	presenceTtl map[string]time.Time
}

func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		subscribers: make(map[string]map[*localSubscription]struct{}),
		activeLocks: make(map[string]localLockInfo),
		presences:   make(map[string]AgentPresence),
		presenceTtl: make(map[string]time.Time),
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

func generateToken() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b)
}

func (m *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (string, bool, error) {
	m.locks.Lock()
	defer m.locks.Unlock()

	now := time.Now()
	if info, ok := m.activeLocks[key]; ok {
		if now.Before(info.expiry) {
			return "", false, nil // Lock is held
		}
	}

	token := generateToken()
	m.activeLocks[key] = localLockInfo{
		expiry: now.Add(ttl),
		token:  token,
	}
	return token, true, nil
}

func (m *LocalMesh) ReleaseLock(ctx context.Context, key string, token string) error {
	m.locks.Lock()
	defer m.locks.Unlock()

	info, ok := m.activeLocks[key]
	if !ok {
		return errors.New("lock not found or expired")
	}

	if info.token != token {
		return errors.New("invalid token")
	}

	delete(m.activeLocks, key)
	return nil
}

func (m *LocalMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.presences[agentID] = AgentPresence{AgentID: agentID, Status: status}
	m.presenceTtl[agentID] = time.Now().Add(30 * time.Second)
	return nil
}

func (m *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()

	var agents []AgentPresence
	now := time.Now()
	for id, p := range m.presences {
		if ttl, ok := m.presenceTtl[id]; ok && now.Before(ttl) {
			agents = append(agents, p)
		}
	}
	return agents, nil
}
