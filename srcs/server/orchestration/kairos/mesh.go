package kairos

import (
	"context"
	"sync"

	"github.com/redis/go-redis/v9"
)

type TeammateMesh interface {
	Publish(channel string, message []byte) error
	Subscribe(channel string) (<-chan []byte, error)
}

// MemoryMesh implements TeammateMesh using local memory channels.
type MemoryMesh struct {
	mu       sync.RWMutex
	channels map[string][]chan []byte
}

func NewMemoryMesh() *MemoryMesh {
	return &MemoryMesh{
		channels: make(map[string][]chan []byte),
	}
}

func (m *MemoryMesh) Publish(channel string, message []byte) error {
	m.mu.RLock()
	defer m.mu.RUnlock()

	subs := m.channels[channel]
	for _, sub := range subs {
		select {
		case sub <- message:
		default:
			// Non-blocking send
		}
	}
	return nil
}

func (m *MemoryMesh) Subscribe(channel string) (<-chan []byte, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	ch := make(chan []byte, 100)
	m.channels[channel] = append(m.channels[channel], ch)
	return ch, nil
}

// RedisMesh implements TeammateMesh using Redis Pub/Sub.
type RedisMesh struct {
	client *redis.Client
	ctx    context.Context
}

func NewRedisMesh(client *redis.Client) *RedisMesh {
	return &RedisMesh{
		client: client,
		ctx:    context.Background(),
	}
}

func (r *RedisMesh) Publish(channel string, message []byte) error {
	return r.client.Publish(r.ctx, channel, message).Err()
}

func (r *RedisMesh) Subscribe(channel string) (<-chan []byte, error) {
	pubsub := r.client.Subscribe(r.ctx, channel)
	ch := make(chan []byte, 100)

	go func() {
		defer close(ch)
		for msg := range pubsub.Channel() {
			ch <- []byte(msg.Payload)
		}
	}()

	return ch, nil
}

func NewTeammateMesh(redisClient *redis.Client) TeammateMesh {
	if redisClient != nil {
		return NewRedisMesh(redisClient)
	}
	return NewMemoryMesh()
}
