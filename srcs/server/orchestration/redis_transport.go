package orchestration

import (
	"context"
	"fmt"
	"sync"

	"github.com/redis/rueidis"
	"google.golang.org/protobuf/proto"

	pb "github.com/onehumancorp/ohc/srcs/proto"
)

// RedisMeshTransport implements MeshTransport using Redis Pub/Sub via rueidis for Cloud-native mode.
type RedisMeshTransport struct {
	client rueidis.Client
	mu     sync.Mutex
	cancel context.CancelFunc
}

// NewRedisMeshTransport creates a new RedisMeshTransport.
func NewRedisMeshTransport(redisURL string) (*RedisMeshTransport, error) {
	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress: []string{redisURL},
	})
	if err != nil {
		return nil, fmt.Errorf("failed to create redis client: %w", err)
	}

	return &RedisMeshTransport{
		client: client,
	}, nil
}

// Publish broadcasts an event to a Redis channel.
func (t *RedisMeshTransport) Publish(ctx context.Context, channel string, event *pb.MeshEvent) error {
	data, err := proto.Marshal(event)
	if err != nil {
		return fmt.Errorf("failed to marshal event: %w", err)
	}

	cmd := t.client.B().Publish().Channel(channel).Message(string(data)).Build()
	err = t.client.Do(ctx, cmd).Error()
	if err != nil {
		return fmt.Errorf("failed to publish to redis: %w", err)
	}

	return nil
}

// Subscribe registers a handler for a Redis channel.
func (t *RedisMeshTransport) Subscribe(ctx context.Context, channel string, handler func(*pb.MeshEvent)) error {
	t.mu.Lock()
	defer t.mu.Unlock()

	// Create a sub-context for the subscription that can be cancelled when Close is called
	subCtx, cancel := context.WithCancel(context.Background())
	if t.cancel != nil {
		oldCancel := t.cancel
		t.cancel = func() {
			oldCancel()
			cancel()
		}
	} else {
		t.cancel = cancel
	}

	// Receive is a blocking call, so we must run it in a separate goroutine
	go func() {
		err := t.client.Receive(subCtx, t.client.B().Subscribe().Channel(channel).Build(), func(msg rueidis.PubSubMessage) {
			var event pb.MeshEvent
			if err := proto.Unmarshal([]byte(msg.Message), &event); err != nil {
				fmt.Printf("failed to unmarshal redis message: %v\n", err)
				return
			}
			handler(&event)
		})

		if err != nil && err != context.Canceled {
			fmt.Printf("failed to subscribe to redis: %v\n", err)
		}
	}()

    return nil
}

// Close closes the transport.
func (t *RedisMeshTransport) Close() error {
	t.mu.Lock()
	defer t.mu.Unlock()

	if t.cancel != nil {
		t.cancel()
	}

	if t.client != nil {
		t.client.Close()
	}

	return nil
}
