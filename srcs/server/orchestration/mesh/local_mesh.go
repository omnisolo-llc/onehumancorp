package mesh

import (
	"context"
	"sync"
	"time"
)

type LocalMesh struct {
	mu          sync.RWMutex
	subscribers map[string][]func(msg []byte)
	locks       map[string]time.Time
	presence    map[string]AgentPresence
}

func NewLocalMesh() *LocalMesh {
	lm := &LocalMesh{
		subscribers: make(map[string][]func(msg []byte)),
		locks:       make(map[string]time.Time),
		presence:    make(map[string]AgentPresence),
	}
	go lm.cleanupRoutine()
	return lm
}

func (lm *LocalMesh) cleanupRoutine() {
	ticker := time.NewTicker(5 * time.Second)
	for range ticker.C {
		lm.mu.Lock()
		now := time.Now()
		for k, expiry := range lm.locks {
			if now.After(expiry) {
				delete(lm.locks, k)
			}
		}
		for k, p := range lm.presence {
			if now.Sub(p.UpdatedAt) > 30*time.Second {
				delete(lm.presence, k)
			}
		}
		lm.mu.Unlock()
	}
}

func (lm *LocalMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	lm.mu.RLock()
	subs := lm.subscribers[topic]
	lm.mu.RUnlock()
	for _, handler := range subs {
		go handler(payload)
	}
	return nil
}

type localSubscription struct{}

func (ls *localSubscription) Close() error { return nil }

func (lm *LocalMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	lm.mu.Lock()
	lm.subscribers[topic] = append(lm.subscribers[topic], handler)
	lm.mu.Unlock()
	return &localSubscription{}, nil
}

func (lm *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	lm.mu.Lock()
	defer lm.mu.Unlock()
	now := time.Now()
	expiry, ok := lm.locks[key]
	if ok && now.Before(expiry) {
		return false, nil
	}
	lm.locks[key] = now.Add(ttl)
	return true, nil
}

func (lm *LocalMesh) ReleaseLock(ctx context.Context, key string) error {
	lm.mu.Lock()
	defer lm.mu.Unlock()
	delete(lm.locks, key)
	return nil
}

func (lm *LocalMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	lm.mu.Lock()
	defer lm.mu.Unlock()
	lm.presence[agentID] = AgentPresence{
		AgentID:   agentID,
		Status:    status,
		UpdatedAt: time.Now(),
	}
	return nil
}

func (lm *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	lm.mu.RLock()
	defer lm.mu.RUnlock()
	var agents []AgentPresence
	now := time.Now()
	for _, p := range lm.presence {
		if now.Sub(p.UpdatedAt) <= 30*time.Second {
			agents = append(agents, p)
		}
	}
	return agents, nil
}
