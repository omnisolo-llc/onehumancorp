package kairos

import (
	"context"
	"sync"

	"github.com/redis/go-redis/v9"
	"github.com/nats-io/nats.go"
)

type TeammateMesh interface {
	Publish(ctx context.Context, channel string, message []byte) error
	Subscribe(ctx context.Context, channel string) (<-chan []byte, error)
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

func (m *MemoryMesh) Publish(ctx context.Context, channel string, message []byte) error {
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

func (m *MemoryMesh) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	ch := make(chan []byte, 100)
	m.channels[channel] = append(m.channels[channel], ch)

	go func() {
		<-ctx.Done()
		m.mu.Lock()
		defer m.mu.Unlock()
		subs := m.channels[channel]
		for i, s := range subs {
			if s == ch {
				m.channels[channel] = append(subs[:i], subs[i+1:]...)
				break
			}
		}
		close(ch)
	}()

	return ch, nil
}

// RedisMesh implements TeammateMesh using Redis Pub/Sub.
type RedisMesh struct {
	client *redis.Client
}

func NewRedisMesh(client *redis.Client) *RedisMesh {
	return &RedisMesh{
		client: client,
	}
}

func (r *RedisMesh) Publish(ctx context.Context, channel string, message []byte) error {
	return r.client.Publish(ctx, channel, message).Err()
}

func (r *RedisMesh) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	pubsub := r.client.Subscribe(ctx, channel)
	ch := make(chan []byte, 100)

	go func() {
		defer close(ch)
		defer pubsub.Close()

		msgCh := pubsub.Channel()
		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-msgCh:
				if !ok {
					return
				}
				select {
				case ch <- []byte(msg.Payload):
				case <-ctx.Done():
					return
				}
			}
		}
	}()
		return ch, nil
}

// NatsMesh implements TeammateMesh using NATS Pub/Sub.
type NatsMesh struct {
	nc *nats.Conn
}

func NewNatsMesh(nc *nats.Conn) *NatsMesh {
	return &NatsMesh{
		nc: nc,
	}
}

func (n *NatsMesh) Publish(ctx context.Context, channel string, message []byte) error {
	return n.nc.Publish(channel, message)
}

func (n *NatsMesh) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	ch := make(chan []byte, 100)

	sub, err := n.nc.Subscribe(channel, func(msg *nats.Msg) {
		select {
		case ch <- msg.Data:
		case <-ctx.Done():
		default:
		}
	})
	if err != nil {
		return nil, err
	}

	go func() {
		<-ctx.Done()
		sub.Unsubscribe()
		close(ch)
	}()

	return ch, nil
}

// LocalTeammateMesh implements TeammateMesh and provides explicit channels for mesh:tasks and mesh:coordination.
type LocalTeammateMesh struct {
	mesh *MemoryMesh
}

func NewLocalTeammateMesh() *LocalTeammateMesh {
	return &LocalTeammateMesh{
		mesh: NewMemoryMesh(),
	}
}

func (l *LocalTeammateMesh) Publish(ctx context.Context, channel string, message []byte) error {
	return l.mesh.Publish(ctx, channel, message)
}

func (l *LocalTeammateMesh) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	return l.mesh.Subscribe(ctx, channel)
}

func (l *LocalTeammateMesh) PublishTask(ctx context.Context, message []byte) error {
	return l.Publish(ctx, "mesh:tasks", message)
}

func (l *LocalTeammateMesh) SubscribeTasks(ctx context.Context) (<-chan []byte, error) {
	return l.Subscribe(ctx, "mesh:tasks")
}

func (l *LocalTeammateMesh) PublishCoordination(ctx context.Context, message []byte) error {
	return l.Publish(ctx, "mesh:coordination", message)
}

func (l *LocalTeammateMesh) SubscribeCoordination(ctx context.Context) (<-chan []byte, error) {
	return l.Subscribe(ctx, "mesh:coordination")
}

func NewTeammateMesh(redisClient *redis.Client) TeammateMesh {
	if redisClient != nil {
		return NewRedisMesh(redisClient)
	}
	return NewMemoryMesh()
}
