package mesh

import (
	"context"
	"errors"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type LocalMeshV2 struct {
	mu       sync.RWMutex
	topics   map[string]map[int64]func([]byte)
	locks    map[string]struct{
		lockID string
		expiry time.Time
	}
	presence map[string]AgentPresence
    subId    int64
}

func NewLocalMeshV2() *LocalMeshV2 {
	return &LocalMeshV2{
		topics:   make(map[string]map[int64]func([]byte)),
		locks:    make(map[string]struct{
			lockID string
			expiry time.Time
		}),
		presence: make(map[string]AgentPresence),
	}
}

type localSubscriptionV2 struct {
	mesh    *LocalMeshV2
	topic   string
	id      int64
}

func (s *localSubscriptionV2) Unsubscribe() error {
	s.mesh.mu.Lock()
	defer s.mesh.mu.Unlock()

    if s.mesh.topics[s.topic] != nil {
        delete(s.mesh.topics[s.topic], s.id)
    }
	return nil
}

func (lm *LocalMeshV2) Publish(ctx context.Context, topic string, payload []byte) error {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return errors.New("unauthorized: missing claims")
	}

	broadcastCount.Add(ctx, 1)

	lm.mu.RLock()

    // Copy the slice of handlers so we don't hold the lock while iterating
    var handlers []func([]byte)
    if topicMap, ok := lm.topics[topic]; ok {
        for _, handler := range topicMap {
            handlers = append(handlers, handler)
        }
    }
	lm.mu.RUnlock()

	for _, handler := range handlers {
		go handler(payload)
	}
	return nil
}

func (lm *LocalMeshV2) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	subscribeCount.Add(ctx, 1)

	lm.mu.Lock()
	defer lm.mu.Unlock()

    if lm.topics[topic] == nil {
        lm.topics[topic] = make(map[int64]func([]byte))
    }

    lm.subId++
    currentSubId := lm.subId

	lm.topics[topic][currentSubId] = handler
	return &localSubscriptionV2{mesh: lm, topic: topic, id: currentSubId}, nil
}

func (lm *LocalMeshV2) AcquireLock(ctx context.Context, key string, lockID string, ttl time.Duration) (bool, error) {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	lockData, exists := lm.locks[key]
	if exists && time.Now().Before(lockData.expiry) {
		return false, nil // Lock is held
	}

	lm.locks[key] = struct{
		lockID string
		expiry time.Time
	}{
		lockID: lockID,
		expiry: time.Now().Add(ttl),
	}
	return true, nil
}

func (lm *LocalMeshV2) ReleaseLock(ctx context.Context, key string, lockID string) error {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	lockData, exists := lm.locks[key]
	if !exists {
		return errors.New("lock is not owned or has expired")
	}

	if lockData.lockID != lockID {
		return errors.New("lock is not owned or has expired")
	}

	delete(lm.locks, key)
	return nil
}

func (lm *LocalMeshV2) RegisterPresence(ctx context.Context, agentID string, status string) error {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	lm.presence[agentID] = AgentPresence{
		AgentID:  agentID,
		Status:   status,
		LastSeen: time.Now(),
	}
	return nil
}

func (lm *LocalMeshV2) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	var agents []AgentPresence
	now := time.Now()
	for id, p := range lm.presence {
		if now.Sub(p.LastSeen) < 30*time.Second {
			agents = append(agents, p)
		} else {
            delete(lm.presence, id)
        }
	}
	return agents, nil
}
