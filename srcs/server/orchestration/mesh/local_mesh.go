package mesh

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/google/uuid"
)

type localSubscription struct {
	topic string
	ch    chan []byte
	lm    *LocalMesh
}

func (s *localSubscription) Close() error {
	s.lm.mu.Lock()
	defer s.lm.mu.Unlock()

	subs := s.lm.topics[s.topic]
	for i, sub := range subs {
		if sub == s.ch {
			s.lm.topics[s.topic] = append(subs[:i], subs[i+1:]...)
			break
		}
	}
	return nil
}

type lockEntry struct {
	token     string
	expiresAt time.Time
}

// LocalMesh implements TeammateMesh in-memory.
type LocalMesh struct {
	mu       sync.RWMutex
	topics   map[string][]chan []byte
	locks    map[string]lockEntry
	presence map[string]AgentPresence

	// Local mapping to emulate unique connection instance for ReleaseLock token check
	instanceID string
}

// NewLocalMesh creates a new LocalMesh instance.
func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		topics:     make(map[string][]chan []byte),
		locks:      make(map[string]lockEntry),
		presence:   make(map[string]AgentPresence),
		instanceID: uuid.NewString(),
	}
}

// Publish sends a message to all subscribers of the topic.
func (m *LocalMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	m.mu.RLock()
	subs, ok := m.topics[topic]
	var subsCopy []chan []byte
	if ok {
		subsCopy = make([]chan []byte, len(subs))
		copy(subsCopy, subs)
	}
	m.mu.RUnlock()

	if !ok {
		return nil // No subscribers
	}

	for _, sub := range subsCopy {
		select {
		case sub <- payload:
		case <-ctx.Done():
			return ctx.Err()
		default:
			// Non-blocking send: drop message if channel is full
		}
	}
	return nil
}

// Subscribe listens to messages on a given topic.
func (m *LocalMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	ch := make(chan []byte, 100) // buffered to prevent blocking
	m.mu.Lock()
	m.topics[topic] = append(m.topics[topic], ch)
	m.mu.Unlock()

	sub := &localSubscription{
		topic: topic,
		ch:    ch,
		lm:    m,
	}

	go func() {
		defer sub.Close()
		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-ch:
				if !ok {
					return
				}
				handler(msg)
			}
		}
	}()

	return sub, nil
}

// AcquireLock attempts to acquire a lock for a given key.
func (m *LocalMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	now := time.Now()
	if entry, ok := m.locks[key]; ok {
		if now.Before(entry.expiresAt) && entry.token != m.instanceID {
			return false, nil // Locked by someone else and not expired
		}
	}

	m.locks[key] = lockEntry{
		token:     m.instanceID,
		expiresAt: now.Add(ttl),
	}
	return true, nil
}

// ReleaseLock releases the lock if owned by this instance.
func (m *LocalMesh) ReleaseLock(ctx context.Context, key string) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	entry, ok := m.locks[key]
	if !ok {
		return nil // Not locked
	}

	if entry.token == m.instanceID {
		delete(m.locks, key)
		return nil
	}
	return fmt.Errorf("lock not owned by this instance")
}

// RegisterPresence updates the presence status of an agent.
func (m *LocalMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	m.presence[agentID] = AgentPresence{
		AgentID: agentID,
		Status:  status,
		Updated: time.Now(),
	}
	return nil
}

// GetActiveAgents retrieves all agents that have reported presence recently.
func (m *LocalMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	now := time.Now()
	agents := make([]AgentPresence, 0, len(m.presence))
	for id, p := range m.presence {
		// Example: consider inactive if not updated in the last 2 minutes
		if now.Sub(p.Updated) < 2*time.Minute {
			agents = append(agents, p)
		} else {
			delete(m.presence, id)
		}
	}
	return agents, nil
}
