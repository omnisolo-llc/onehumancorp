package kairos

import (
	"context"
	"fmt"
	"sync"

	"github.com/redis/go-redis/v9"
)

type TeammateMesh interface {
	Publish(channel string, message []byte) error
	Subscribe(channel string) (<-chan []byte, error)
}

// MemoryMesh implements TeammateMesh using in-memory channels (fallback).
type MemoryMesh struct {
	mu          sync.RWMutex
	subscribers map[string][]chan []byte
}

func NewMemoryMesh() *MemoryMesh {
	return &MemoryMesh{
		subscribers: make(map[string][]chan []byte),
	}
}

func (m *MemoryMesh) Publish(channel string, message []byte) error {
	m.mu.RLock()
	defer m.mu.RUnlock()

	subs, exists := m.subscribers[channel]
	if !exists {
		return nil
	}

	for _, ch := range subs {
		// Non-blocking send
		select {
		case ch <- message:
		default:
		}
	}
	return nil
}

func (m *MemoryMesh) Subscribe(channel string) (<-chan []byte, error) {
	m.mu.Lock()
	defer m.mu.Unlock() // Corrected: Unlock instead of Lock

	ch := make(chan []byte, 100)
	m.subscribers[channel] = append(m.subscribers[channel], ch)

	return ch, nil
}

// RedisMesh implements TeammateMesh using Redis Pub/Sub (primary).
type RedisMesh struct {
	client *redis.Client
	ctx    context.Context
}

func NewRedisMesh(redisURL string) (*RedisMesh, error) {
	opt, err := redis.ParseURL(redisURL)
	if err != nil {
		return nil, fmt.Errorf("failed to parse redis url: %v", err)
	}

	client := redis.NewClient(opt)
	ctx := context.Background()

	// Ping to verify connection
	if err := client.Ping(ctx).Err(); err != nil {
		return nil, fmt.Errorf("failed to connect to redis: %v", err)
	}

	return &RedisMesh{
		client: client,
		ctx:    ctx,
	}, nil
}

func (r *RedisMesh) Publish(channel string, message []byte) error {
	return r.client.Publish(r.ctx, channel, message).Err()
}

func (r *RedisMesh) Subscribe(channel string) (<-chan []byte, error) {
	pubsub := r.client.Subscribe(r.ctx, channel)

	// Wait for subscription to be established
	_, err := pubsub.Receive(r.ctx)
	if err != nil {
		pubsub.Close()
		return nil, fmt.Errorf("failed to subscribe: %v", err)
	}

	ch := make(chan []byte, 100)

	go func() {
		defer close(ch)
		defer pubsub.Close()

		for msg := range pubsub.Channel() {
			ch <- []byte(msg.Payload)
		}
	}()

	return ch, nil
}
