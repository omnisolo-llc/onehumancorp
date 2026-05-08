package orchestration

import (
	"context"

	"github.com/redis/rueidis"
)

// RedisMeshTransport implements MeshTransport using Redis Pub/Sub via rueidis.
type RedisMeshTransport struct {
	client rueidis.Client
}

// NewRedisMeshTransport creates a new RedisMeshTransport.
func NewRedisMeshTransport(addr string) (MeshTransport, error) {
	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{addr},
		DisableCache: true,
	})
	if err != nil {
		return nil, err
	}
	return &RedisMeshTransport{client: client}, nil
}

// Publish sends data to the given channel using Redis Pub/Sub.
func (m *RedisMeshTransport) Publish(ctx context.Context, channel string, data []byte) error {
	if m.client == nil {
		return nil
	}
	return m.client.Do(ctx, m.client.B().Publish().Channel(channel).Message(rueidis.BinaryString(data)).Build()).Error()
}

// Subscribe registers a handler for the given channel using Redis Pub/Sub.
func (m *RedisMeshTransport) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	if m.client == nil {
		return nil
	}
	go func() {
		err := m.client.Receive(ctx, m.client.B().Subscribe().Channel(channel).Build(), func(msg rueidis.PubSubMessage) {
			handler([]byte(msg.Message))
		})
		if err != nil && ctx.Err() == nil {
		}
	}()
	return nil
}
