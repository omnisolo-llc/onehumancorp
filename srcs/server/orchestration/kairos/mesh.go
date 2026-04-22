package kairos

import (
	"context"
	"sync"

	"github.com/redis/go-redis/v9"
)

type Subscription interface {
	Close() error
}

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
	for _, ch := range m.channels[channel] {
		select {
		case ch <- message:
		default:
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
		for i, sub := range subs {
			if sub == ch {
				m.channels[channel] = append(subs[:i], subs[i+1:]...)
				close(ch)
				break
			}
		}
	}()

	return ch, nil
}

type RedisMesh struct {
	client *redis.Client
}

func NewRedisMesh(client *redis.Client) *RedisMesh {
	return &RedisMesh{
		client: client,
	}
}

func (m *RedisMesh) Publish(ctx context.Context, channel string, message []byte) error {
	return m.client.Publish(ctx, channel, message).Err()
}

func (m *RedisMesh) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	pubsub := m.client.Subscribe(ctx, channel)

	_, err := pubsub.Receive(ctx)
	if err != nil {
		pubsub.Close()
		return nil, err
	}

	subCtx, cancel := context.WithCancel(ctx)
	ch := pubsub.Channel()

	outCh := make(chan []byte, 100)

	go func() {
		defer close(outCh)
		defer pubsub.Close()
		for {
			select {
			case msg, ok := <-ch:
				if !ok {
					return
				}
				outCh <- []byte(msg.Payload)
			case <-subCtx.Done():
				return
			}
		}
	}()

	go func() {
		<-ctx.Done()
		cancel()
	}()

	return outCh, nil
}

func NewTeammateMesh(client *redis.Client) TeammateMesh {
	if client != nil {
		return NewRedisMesh(client)
	}
	return NewMemoryMesh()
}

// LocalTeammateMesh implements TeammateMesh and provides explicit channels for mesh:tasks and mesh:coordination.
// Per problem statement: Implement LocalTeammateMesh using Redis Pub/Sub channels mesh:tasks and mesh:coordination.
type LocalTeammateMesh struct {
	TeammateMesh
}

// We change NewLocalTeammateMesh to accept an optional *redis.Client
// so we can initialize a RedisMesh if provided, otherwise MemoryMesh
func NewLocalTeammateMesh(client *redis.Client) *LocalTeammateMesh {
	return &LocalTeammateMesh{
		TeammateMesh: NewTeammateMesh(client),
	}
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
