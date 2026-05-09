package orchestration

import (
	"context"
	"fmt"
	"sync"
	"time"

	"onehumancorp/srcs/server/db"
	"github.com/redis/rueidis"
)

// MeshHub defines the interface for the Teammate Mesh APIs.
type MeshHub interface {
	Publish(ctx context.Context, topic string, payload []byte) error
	Subscribe(ctx context.Context, topic string) (<-chan []byte, error)
	Close() error
}

// NewMeshHub creates a new MeshHub based on the deployment mode (Cloud vs Standalone).
func NewMeshHub(ctx context.Context) (MeshHub, error) {
	isSQLite := false
	if db.GlobalProvider != nil {
		isSQLite = db.GlobalProvider.IsSQLite()
	}
	if isSQLite {
		return NewLocalMeshHub(), nil
	}
	return NewRedisMeshHub(ctx)
}

// LocalMeshHub implements MeshHub using Go channels for Standalone Desktop Mode.
type LocalMeshHub struct {
	mu          sync.RWMutex
	subscribers map[string][]chan []byte
}

func NewLocalMeshHub() *LocalMeshHub {
	return &LocalMeshHub{
		subscribers: make(map[string][]chan []byte),
	}
}

func (h *LocalMeshHub) Publish(ctx context.Context, topic string, payload []byte) error {
	h.mu.RLock()
	defer h.mu.RUnlock()

	subs := h.subscribers[topic]
	for _, sub := range subs {
		select {
		case sub <- payload:
		default:
			// Drop message if channel is full
		}
	}
	return nil
}

func (h *LocalMeshHub) Subscribe(ctx context.Context, topic string) (<-chan []byte, error) {
	ch := make(chan []byte, 100)

	h.mu.Lock()
	h.subscribers[topic] = append(h.subscribers[topic], ch)
	h.mu.Unlock()

	go func() {
		<-ctx.Done()

		h.mu.Lock()
		defer h.mu.Unlock()

		subs := h.subscribers[topic]
		for i, sub := range subs {
			if sub == ch {
				h.subscribers[topic] = append(subs[:i], subs[i+1:]...)
				break
			}
		}
	}()

	return ch, nil
}

func (h *LocalMeshHub) Close() error {
	h.mu.Lock()
	defer h.mu.Unlock()
	h.subscribers = make(map[string][]chan []byte)
	return nil
}

// RedisMeshHub implements MeshHub using rueidis for Cloud-Native mode.
type RedisMeshHub struct {
	client rueidis.Client
	closed bool
	mu     sync.RWMutex
}

func NewRedisMeshHub(ctx context.Context) (*RedisMeshHub, error) {
	var opts rueidis.ClientOption
	if db.GlobalProvider != nil && db.GlobalProvider.RedisClient != nil {
		goRedisOpts := db.GlobalProvider.RedisClient.Options()
		opts = rueidis.ClientOption{
			InitAddress: []string{goRedisOpts.Addr},
			Password:    goRedisOpts.Password,
			SelectDB:    goRedisOpts.DB,
			TLSConfig:   goRedisOpts.TLSConfig,
		}
	} else {
		// Fallback for tests or uninitialized environments
		opts = rueidis.ClientOption{InitAddress: []string{"127.0.0.1:6379"}}
	}

	client, err := rueidis.NewClient(opts)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to redis: %w", err)
	}
	return &RedisMeshHub{client: client}, nil
}

// For testing purposes when we can provide our own client
func NewRedisMeshHubWithClient(client rueidis.Client) *RedisMeshHub {
	return &RedisMeshHub{client: client}
}

func (h *RedisMeshHub) Publish(ctx context.Context, topic string, payload []byte) error {
	h.mu.RLock()
	if h.closed {
		h.mu.RUnlock()
		return fmt.Errorf("mesh hub closed")
	}
	h.mu.RUnlock()

	cmd := h.client.B().Publish().Channel(topic).Message(string(payload)).Build()
	return h.client.Do(ctx, cmd).Error()
}

func (h *RedisMeshHub) Subscribe(ctx context.Context, topic string) (<-chan []byte, error) {
	ch := make(chan []byte, 100)

	go func() {
		for {
			h.mu.RLock()
			isClosed := h.closed
			h.mu.RUnlock()

			if isClosed {
				return
			}

			select {
			case <-ctx.Done():
				return
			default:
				err := h.client.Receive(ctx, h.client.B().Subscribe().Channel(topic).Build(), func(msg rueidis.PubSubMessage) {
					if msg.Channel == topic {
						select {
						case ch <- []byte(msg.Message):
						case <-ctx.Done():
						default:
							// Message dropped
						}
					}
				})

				if err != nil {
					h.mu.RLock()
					isClosed := h.closed
					h.mu.RUnlock()

					if isClosed {
						return
					}

					// In a real system, log error. Here we just sleep and retry.
					time.Sleep(1 * time.Second)
				}
			}
		}
	}()

	return ch, nil
}

func (h *RedisMeshHub) Close() error {
	h.mu.Lock()
	h.closed = true
	h.mu.Unlock()
	h.client.Close()
	return nil
}
