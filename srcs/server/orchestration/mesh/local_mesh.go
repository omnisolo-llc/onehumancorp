package mesh

import (
	"context"
	"sync"
	"time"

	"github.com/google/uuid"
)

type LocalMesh struct {
	mu        sync.RWMutex
	topics    map[string]map[string]chan []byte
	locks     map[string]*lockInfo
	presence  map[string]presenceInfo
	lockOwner string
}

type lockInfo struct {
	owner     string
	expiresAt time.Time
}

type presenceInfo struct {
	status    string
	expiresAt time.Time
}

type localSubscription struct {
	closeOnce sync.Once
	mesh    *LocalMesh
	topic   string
	subID   string
	ch      chan []byte
	closeCh chan struct{}
}

func (s *localSubscription) Close() error {
	s.mesh.mu.Lock()
	defer s.mesh.mu.Unlock()

	if subs, ok := s.mesh.topics[s.topic]; ok {
		delete(subs, s.subID)
		if len(subs) == 0 {
			delete(s.mesh.topics, s.topic)
		}
	}
	s.closeOnce.Do(func() { close(s.closeCh) })
	return nil
}

func NewLocalTeammateMesh() *LocalMesh {
	return &LocalMesh{
		topics:    make(map[string]map[string]chan []byte),
		locks:     make(map[string]*lockInfo),
		presence:  make(map[string]presenceInfo),
		lockOwner: uuid.New().String(),
	}
}

func (lm *LocalMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	lm.mu.RLock()
	subs, ok := lm.topics[topic]
	if !ok {
		lm.mu.RUnlock()
		return nil
	}

	var channels []chan []byte
	for _, ch := range subs {
		channels = append(channels, ch)
	}
	lm.mu.RUnlock()

	for _, ch := range channels {
		select {
		case ch <- payload:
		case <-ctx.Done():
			return ctx.Err()
		default:
			// Non-blocking send if channel is full
		}
	}
	return nil
}

func (lm *LocalMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	lm.mu.Lock()
	if _, ok := lm.topics[topic]; !ok {
		lm.topics[topic] = make(map[string]chan []byte)
	}
	subID := uuid.New().String()
	ch := make(chan []byte, 100)
	lm.topics[topic][subID] = ch
	lm.mu.Unlock()

	sub := &localSubscription{
		mesh:    lm,
		topic:   topic,
		subID:   subID,
		ch:      ch,
		closeCh: make(chan struct{}),
	}

	go func() {
		for {
			select {
			case msg := <-ch:
				handler(msg)
			case <-sub.closeCh:
				return
			case <-ctx.Done():
				sub.Close()
				return
			}
		}
	}()

	return sub, nil
}

func (lm *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	now := time.Now()
	if lock, exists := lm.locks[key]; exists {
		if now.Before(lock.expiresAt) {
			return false, nil // Lock is held and not expired
		}
	}

	lm.locks[key] = &lockInfo{
		owner:     lm.lockOwner,
		expiresAt: now.Add(ttl),
	}
	return true, nil
}

func (lm *LocalMesh) ReleaseLock(ctx context.Context, key string) error {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	if lock, exists := lm.locks[key]; exists {
		if lock.owner == lm.lockOwner {
			delete(lm.locks, key)
		}
	}
	return nil
}

func (lm *LocalMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	lm.mu.Lock()
	defer lm.mu.Unlock()

	lm.presence[agentID] = presenceInfo{
		status:    status,
		expiresAt: time.Now().Add(30 * time.Second),
	}
	return nil
}

func (lm *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	lm.mu.RLock()
	defer lm.mu.RUnlock()

	var agents []AgentPresence
	now := time.Now()

	for agentID, info := range lm.presence {
		if now.Before(info.expiresAt) {
			agents = append(agents, AgentPresence{
				AgentID: agentID,
				Status:  info.status,
			})
		}
	}
	return agents, nil
}
