package mesh

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"sync"

	"github.com/redis/rueidis"
)

type MeshTransport interface {
	Publish(ctx context.Context, channel string, payload []byte) error
	Subscribe(ctx context.Context, channel string) (<-chan []byte, error)
}

// MemoryTransport implements MeshTransport using Go channels.
type MemoryTransport struct {
	mu          sync.RWMutex
	subscribers map[string][]chan []byte
}

func NewMemoryTransport() *MemoryTransport {
	return &MemoryTransport{
		subscribers: make(map[string][]chan []byte),
	}
}

func (m *MemoryTransport) Publish(ctx context.Context, channel string, payload []byte) error {
	m.mu.RLock()
	defer m.mu.RUnlock()

	subs, ok := m.subscribers[channel]
	if !ok {
		return nil
	}

	for _, sub := range subs {
		select {
		case sub <- payload:
		case <-ctx.Done():
			return ctx.Err()
		default:
			// Non-blocking send
		}
	}
	return nil
}

func (m *MemoryTransport) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	ch := make(chan []byte, 100)
	m.subscribers[channel] = append(m.subscribers[channel], ch)

	go func() {
		<-ctx.Done()
		m.mu.Lock()
		defer m.mu.Unlock()

		subs := m.subscribers[channel]
		for i, sub := range subs {
			if sub == ch {
				m.subscribers[channel] = append(subs[:i], subs[i+1:]...)
				break
			}
		}
		close(ch)
	}()

	return ch, nil
}

// RedisTransport implements MeshTransport using rueidis.
type RedisTransport struct {
	client rueidis.Client
}

func NewRedisTransport(client rueidis.Client) *RedisTransport {
	return &RedisTransport{
		client: client,
	}
}

func (r *RedisTransport) Publish(ctx context.Context, channel string, payload []byte) error {
	cmd := r.client.B().Publish().Channel(channel).Message(string(payload)).Build()
	return r.client.Do(ctx, cmd).Error()
}

func (r *RedisTransport) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	ch := make(chan []byte, 100)

	go func() {
		defer close(ch)
		err := r.client.Receive(ctx, r.client.B().Subscribe().Channel(channel).Build(), func(msg rueidis.PubSubMessage) {
			select {
			case ch <- []byte(msg.Message):
			case <-ctx.Done():
			}
		})
		_ = err // Handle or log error if needed
	}()

	return ch, nil
}

// NewMeshTransport returns a new MeshTransport depending on the execution mode.
// If REDIS_URL is present and OHC_STANDALONE is not true, it returns a cloud transport (Redis).
// Otherwise, it returns a local memory transport.
func NewMeshTransport() (MeshTransport, error) {
	redisURL := os.Getenv("REDIS_URL")
	if redisURL != "" && os.Getenv("OHC_STANDALONE") != "true" {
		opts, err := rueidis.ParseURL(redisURL)
		if err != nil {
			return nil, fmt.Errorf("failed to parse REDIS_URL: %w", err)
		}
		c, err := rueidis.NewClient(opts)
		if err != nil {
			return nil, fmt.Errorf("failed to connect to redis: %w", err)
		}
		slog.Info("MeshTransport initialized in Cloud mode (Redis)")
		return NewRedisTransport(c), nil
	}

	slog.Info("MeshTransport initialized in Standalone mode (In-Memory)")
	return NewMemoryTransport(), nil
}
