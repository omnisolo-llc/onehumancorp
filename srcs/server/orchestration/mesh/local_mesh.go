package mesh

import (
	"context"
	"sync"
	"time"
)

type localSubscription struct {
	topic string
	ch    chan []byte
	lm    *LocalMesh
}

func (s *localSubscription) Unsubscribe() error {
	s.lm.mu.Lock()
	defer s.lm.mu.Unlock()
	subs := s.lm.subs[s.topic]
	for i, sub := range subs {
		if sub == s.ch {
			s.lm.subs[s.topic] = append(subs[:i], subs[i+1:]...)
			break
		}
	}
	// We do not close the channel here to prevent a panic when a concurrent Publish writes to it.
	// Instead, we let it be garbage collected or we can use a select to gracefully drain/drop.
	return nil
}

type lockData struct {
	token  string
	expiry time.Time
}

type LocalMesh struct {
	mu       sync.RWMutex
	subs     map[string][]chan []byte
	locks    map[string]lockData
	presence map[string]AgentPresence
}

func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		subs:     make(map[string][]chan []byte),
		locks:    make(map[string]lockData),
		presence: make(map[string]AgentPresence),
	}
}

func (lm *LocalMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	lm.mu.RLock()
	subs := lm.subs[topic]
	var toSend []chan []byte
	toSend = append(toSend, subs...)
	lm.mu.RUnlock()

	for _, ch := range toSend {
		select {
		case <-ctx.Done():
			return ctx.Err()
		case ch <- payload:
		default:
		}
	}
	return nil
}

func (lm *LocalMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	ch := make(chan []byte, 100)
	lm.mu.Lock()
	lm.subs[topic] = append(lm.subs[topic], ch)
	lm.mu.Unlock()

	go func() {
		for msg := range ch {
			handler(msg)
		}
	}()

	return &localSubscription{topic: topic, ch: ch, lm: lm}, nil
}

func (lm *LocalMesh) AcquireLock(ctx context.Context, key string, token string, ttl time.Duration) (bool, error) {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	if ld, ok := lm.locks[key]; ok && time.Now().Before(ld.expiry) {
		return false, nil
	}

	lm.locks[key] = lockData{token: token, expiry: time.Now().Add(ttl)}
	return true, nil
}

func (lm *LocalMesh) ReleaseLock(ctx context.Context, key string, token string) error {
	lm.mu.Lock()
	defer lm.mu.Unlock()
	if ld, ok := lm.locks[key]; ok && ld.token == token {
		delete(lm.locks, key)
	}
	return nil
}

func (lm *LocalMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	lm.mu.Lock()
	defer lm.mu.Unlock()
	lm.presence[agentID] = AgentPresence{
		AgentID:  agentID,
		Status:   status,
		LastSeen: time.Now().UTC(),
	}
	return nil
}

func (lm *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	lm.mu.RLock()
	defer lm.mu.RUnlock()

	var agents []AgentPresence
	now := time.Now().UTC()
	for _, p := range lm.presence {
		// Clean up stale presence (e.g., > 10s old)
		if now.Sub(p.LastSeen) > 10*time.Second {
			continue // Note: actual deletion could happen here or in a separate goroutine
		}
		agents = append(agents, p)
	}
	return agents, nil
}
