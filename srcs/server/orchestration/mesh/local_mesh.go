package mesh

import (
	"context"
	"errors"
	"sync"
	"time"
)

type localSubscription struct {
	topic string
	id    int
	mesh  *LocalTeammateMesh
}

func (s *localSubscription) Unsubscribe(ctx context.Context) error {
	s.mesh.mu.Lock()
	defer s.mesh.mu.Unlock()

	subs, ok := s.mesh.subscriptions[s.topic]
	if !ok {
		return nil
	}

	for i, sub := range subs {
		if sub.id == s.id {
			s.mesh.subscriptions[s.topic] = append(subs[:i], subs[i+1:]...)
			break
		}
	}
	return nil
}

type localSubHandler struct {
	id      int
	handler func(msg []byte)
}

// LocalTeammateMesh implements TeammateMesh for standalone environments.
type LocalTeammateMesh struct {
	mu            sync.RWMutex
	subscriptions map[string][]localSubHandler
	nextSubID     int

	locks map[string]time.Time

	presence map[string]AgentPresence
}

// NewLocalTeammateMesh creates a new LocalTeammateMesh.
func NewLocalTeammateMesh() *LocalTeammateMesh {
	return &LocalTeammateMesh{
		subscriptions: make(map[string][]localSubHandler),
		locks:         make(map[string]time.Time),
		presence:      make(map[string]AgentPresence),
	}
}

// Publish sends a payload to a topic.
func (lm *LocalTeammateMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	lm.mu.RLock()
	subs := lm.subscriptions[topic]
	// Make a deep copy to prevent data race with Unsubscribe
	subsCopy := make([]localSubHandler, len(subs))
	copy(subsCopy, subs)
	lm.mu.RUnlock()

	for _, sub := range subsCopy {
		// execute handler asynchronously to simulate pub/sub delivery
		go sub.handler(payload)
	}

	return nil
}

// Subscribe listens to a topic.
func (lm *LocalTeammateMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	lm.nextSubID++
	id := lm.nextSubID

	lm.subscriptions[topic] = append(lm.subscriptions[topic], localSubHandler{
		id:      id,
		handler: handler,
	})

	return &localSubscription{
		topic: topic,
		id:    id,
		mesh:  lm,
	}, nil
}

// AcquireLock attempts to acquire a lock for a given key.
func (lm *LocalTeammateMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	expireAt, ok := lm.locks[key]
	if ok && time.Now().Before(expireAt) {
		return false, nil
	}

	lm.locks[key] = time.Now().Add(ttl)
	return true, nil
}

// ReleaseLock releases the lock for a given key.
func (lm *LocalTeammateMesh) ReleaseLock(ctx context.Context, key string) error {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	if _, ok := lm.locks[key]; !ok {
		return errors.New("lock not found or expired")
	}

	delete(lm.locks, key)
	return nil
}

// RegisterPresence updates the presence for an agent.
func (lm *LocalTeammateMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	lm.presence[agentID] = AgentPresence{
		AgentID: agentID,
		Status:  status,
	}

	return nil
}

// GetActiveAgents returns a list of all active agents.
func (lm *LocalTeammateMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	lm.mu.RLock()
	defer lm.mu.RUnlock()

	var agents []AgentPresence
	for _, p := range lm.presence {
		agents = append(agents, p)
	}

	return agents, nil
}
