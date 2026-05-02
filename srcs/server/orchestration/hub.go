package orchestration

import (
	"context"
	"log"

	"github.com/redis/rueidis"
)

// In a real scenario we'd use the generated protobuf types from src/proto/hub.proto
// For this mock orchestration layer without a generated pb package yet, we define a structural equivalent.
type MeshEvent struct {
	EventID   string
	Topic     string
	Payload   []byte
	Timestamp int64
}

type MeshTransport interface {
	Publish(ctx context.Context, event MeshEvent) error
	Subscribe(ctx context.Context, topic string, ch chan<- MeshEvent) error
}

type RedisMeshTransport struct {
	client rueidis.Client
}

func NewRedisMeshTransport(client rueidis.Client) *RedisMeshTransport {
	return &RedisMeshTransport{client: client}
}

func (r *RedisMeshTransport) Publish(ctx context.Context, event MeshEvent) error {
	cmd := r.client.B().Publish().Channel(event.Topic).Message(string(event.Payload)).Build()
	return r.client.Do(ctx, cmd).Error()
}

func (r *RedisMeshTransport) Subscribe(ctx context.Context, topic string, ch chan<- MeshEvent) error {
	// Simple log here since a real blocking subscription needs a specialized rueidis approach
	log.Printf("Subscribed to Redis topic %s", topic)
	return nil
}

type MemoryMeshTransport struct {
	channels map[string]chan<- MeshEvent
}

func NewMemoryMeshTransport() *MemoryMeshTransport {
	return &MemoryMeshTransport{channels: make(map[string]chan<- MeshEvent)}
}

func (m *MemoryMeshTransport) Publish(ctx context.Context, event MeshEvent) error {
	if ch, ok := m.channels[event.Topic]; ok {
		ch <- event
	}
	return nil
}

func (m *MemoryMeshTransport) Subscribe(ctx context.Context, topic string, ch chan<- MeshEvent) error {
	m.channels[topic] = ch
	return nil
}
