package teammates

import (
	"context"
	"sync"
	"time"
)

type LocalSubscription struct {
	id     int
	topic  string
	mesh   *LocalMesh
	closed bool
	mu     sync.Mutex
}

func (s *LocalSubscription) Close() error {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.closed {
		return nil
	}
	s.closed = true
	s.mesh.mu.Lock()
	defer s.mesh.mu.Unlock()
	if subs, ok := s.mesh.topics[s.topic]; ok {
		delete(subs, s.id)
	}
	return nil
}

type LocalMesh struct {
	mu        sync.RWMutex
	topics    map[string]map[int]func(msg []byte)
	nextSubID int
	locks     map[string]time.Time
	presence  map[string]string
}

func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		topics:   make(map[string]map[int]func(msg []byte)),
		locks:    make(map[string]time.Time),
		presence: make(map[string]string),
	}
}

func (l *LocalMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	l.mu.RLock()
	subs, ok := l.topics[topic]
	var handlers []func(msg []byte)
	if ok {
		for _, handler := range subs {
			handlers = append(handlers, handler)
		}
	}
	l.mu.RUnlock()

	for _, handler := range handlers {
		handler(payload)
	}
	return nil
}

func (l *LocalMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	l.mu.Lock()
	defer l.mu.Unlock()
	if l.topics[topic] == nil {
		l.topics[topic] = make(map[int]func(msg []byte))
	}
	id := l.nextSubID
	l.nextSubID++
	l.topics[topic][id] = handler

	sub := &LocalSubscription{
		id:    id,
		topic: topic,
		mesh:  l,
	}

	go func() {
		<-ctx.Done()
		sub.Close()
	}()

	return sub, nil
}

func (l *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	l.mu.Lock()
	defer l.mu.Unlock()
	if expiresAt, ok := l.locks[key]; ok {
		if time.Now().Before(expiresAt) {
			return false, nil
		}
	}
	l.locks[key] = time.Now().Add(ttl)
	return true, nil
}

func (l *LocalMesh) ReleaseLock(ctx context.Context, key string) error {
	l.mu.Lock()
	defer l.mu.Unlock()
	delete(l.locks, key)
	return nil
}

func (l *LocalMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	l.mu.Lock()
	defer l.mu.Unlock()
	l.presence[agentID] = status
	return nil
}

func (l *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	l.mu.RLock()
	defer l.mu.RUnlock()
	var agents []AgentPresence
	for id, status := range l.presence {
		agents = append(agents, AgentPresence{AgentID: id, Status: status})
	}
	return agents, nil
}
