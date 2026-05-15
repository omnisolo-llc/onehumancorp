package mesh

import (
	"context"
	"encoding/json"
	"time"
    "sync"

	hub "github.com/onehumancorp/mono/src/proto/hub"
	"github.com/onehumancorp/mono/src/server/orchestration/telemetry"
	"github.com/redis/rueidis"
)

type MeshTransport interface {
	Publish(ctx context.Context, topic string, event *hub.MeshEvent) error
	Subscribe(ctx context.Context, topic string, handler func(*hub.MeshEvent)) error
	AcquireLock(ctx context.Context, resource string, owner string, ttl time.Duration) (bool, error)
	ReleaseLock(ctx context.Context, resource string, owner string) error
}

type CentrifugeNode struct {
	transport MeshTransport
    mu        sync.RWMutex
}

func NewCentrifugeNode(transport MeshTransport) *CentrifugeNode {
	return &CentrifugeNode{transport: transport}
}

func (c *CentrifugeNode) Broadcast(ctx context.Context, topic string, event *hub.MeshEvent) error {
	telemetry.RecordMeshLatency(ctx, 0.5)
	telemetry.RecordMeshThroughput(ctx, 1)
	return c.transport.Publish(ctx, topic, event)
}

func (c *CentrifugeNode) SubscribeNode(ctx context.Context, topic string, handler func(*hub.MeshEvent)) error {
    return c.transport.Subscribe(ctx, topic, handler)
}



type RedisMeshTransport struct {
	client rueidis.Client
}

func NewRedisMeshTransport(addrs []string) (*RedisMeshTransport, error) {
	client, err := rueidis.NewClient(rueidis.ClientOption{InitAddress: addrs})
	if err != nil {
		return nil, err
	}
	return &RedisMeshTransport{client: client}, nil
}

func (r *RedisMeshTransport) Publish(ctx context.Context, topic string, event *hub.MeshEvent) error {
	data, err := json.Marshal(event)
    if err != nil {
        return err
    }
    cmd := r.client.B().Publish().Channel(topic).Message(string(data)).Build()
	return r.client.Do(ctx, cmd).Error()
}

func (r *RedisMeshTransport) Subscribe(ctx context.Context, topic string, handler func(*hub.MeshEvent)) error {
	go func() {
		_ = r.client.Receive(ctx, r.client.B().Subscribe().Channel(topic).Build(), func(msg rueidis.PubSubMessage) {
			var event hub.MeshEvent
			if err := json.Unmarshal([]byte(msg.Message), &event); err == nil {
				handler(&event)
			}
		})
	}()
	return nil
}

func (r *RedisMeshTransport) AcquireLock(ctx context.Context, resource string, owner string, ttl time.Duration) (bool, error) {
	cmd := r.client.B().Set().Key(resource).Value(owner).Nx().Build()
	err := r.client.Do(ctx, cmd).Error()
	if err == nil {
		return true, nil
	}
	return false, err
}

func (r *RedisMeshTransport) ReleaseLock(ctx context.Context, resource string, owner string) error {
	cmd := r.client.B().Del().Key(resource).Build()
	return r.client.Do(ctx, cmd).Error()
}

type MemoryMeshTransport struct{
    handlers map[string][]func(*hub.MeshEvent)
    locks    map[string]string
    mu       sync.RWMutex
}

func NewMemoryMeshTransport() *MemoryMeshTransport {
	return &MemoryMeshTransport{
        handlers: make(map[string][]func(*hub.MeshEvent)),
        locks:    make(map[string]string),
    }
}

func (m *MemoryMeshTransport) Publish(ctx context.Context, topic string, event *hub.MeshEvent) error {
	m.mu.RLock()
    funcs := m.handlers[topic]
    m.mu.RUnlock()
    for _, f := range funcs {
        go f(event)
    }
    return nil
}

func (m *MemoryMeshTransport) Subscribe(ctx context.Context, topic string, handler func(*hub.MeshEvent)) error {
	m.mu.Lock()
    m.handlers[topic] = append(m.handlers[topic], handler)
    m.mu.Unlock()
    return nil
}

func (m *MemoryMeshTransport) AcquireLock(ctx context.Context, resource string, owner string, ttl time.Duration) (bool, error) {
	m.mu.Lock()
    defer m.mu.Unlock()
    if _, exists := m.locks[resource]; !exists {
        m.locks[resource] = owner
        return true, nil
    }
    return false, nil
}

func (m *MemoryMeshTransport) ReleaseLock(ctx context.Context, resource string, owner string) error {
	m.mu.Lock()
    defer m.mu.Unlock()
    if m.locks[resource] == owner {
        delete(m.locks, resource)
    }
    return nil
}
