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

func (ls *localSubscription) Unsubscribe() error {
	ls.lm.mu.Lock()
	defer ls.lm.mu.Unlock()

	subs := ls.lm.subs[ls.topic]
	var newSubs []chan []byte
	for _, sub := range subs {
		if sub != ls.ch {
			newSubs = append(newSubs, sub)
		}
	}
	ls.lm.subs[ls.topic] = newSubs
	close(ls.ch)
	return nil
}

type LocalMesh struct {
	mu       sync.RWMutex
	subs     map[string][]chan []byte
	locks    map[string]lockEntry
	presence map[string]string
}

type lockEntry struct {
	expiresAt time.Time
	token     string
}

func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		subs:     make(map[string][]chan []byte),
		locks:    make(map[string]lockEntry),
		presence: make(map[string]string),
	}
}

// ... skipped down to lock ...

func (lm *LocalMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	lm.mu.RLock()
	defer lm.mu.RUnlock()

	for _, ch := range lm.subs[topic] {
		select {
		case ch <- payload:
		case <-ctx.Done():
			return ctx.Err()
		default:
		}
	}
	return nil
}

func (lm *LocalMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	ch := make(chan []byte, 100)
	lm.subs[topic] = append(lm.subs[topic], ch)

	go func() {
		for msg := range ch {
			handler(msg)
		}
	}()

	return &localSubscription{
		topic: topic,
		ch:    ch,
		lm:    lm,
	}, nil
}

func (lm *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration, token string) (bool, error) {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	now := time.Now()
	if entry, ok := lm.locks[key]; ok {
		if now.Before(entry.expiresAt) {
			return false, nil
		}
	}

	lm.locks[key] = lockEntry{
		expiresAt: now.Add(ttl),
		token:     token,
	}
	return true, nil
}

func (lm *LocalMesh) ReleaseLock(ctx context.Context, key string, token string) error {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	if entry, ok := lm.locks[key]; ok {
		if entry.token == token {
			delete(lm.locks, key)
		}
	}
	return nil
}

func (lm *LocalMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	lm.mu.Lock()
	defer lm.mu.Unlock()
	lm.presence[agentID] = status
	return nil
}

func (lm *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	lm.mu.RLock()
	defer lm.mu.RUnlock()

	var agents []AgentPresence
	for agentID, status := range lm.presence {
		agents = append(agents, AgentPresence{
			AgentID: agentID,
			Status:  status,
		})
	}
	return agents, nil
}

func (lm *LocalMesh) HandoffState(ctx context.Context, targetAgentID string, state []byte) error {
	return lm.Publish(ctx, "handoff:"+targetAgentID, state)
}

func (lm *LocalMesh) SubscribeHandoffs(ctx context.Context, agentID string, handler func(state []byte)) (Subscription, error) {
	return lm.Subscribe(ctx, "handoff:"+agentID, handler)
}
