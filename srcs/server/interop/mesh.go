package interop


import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"sync"

	"github.com/redis/rueidis"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                 = otel.Meter("github.com/onehumancorp/mono/srcs/server/interop")
	meshMessagesPublished, _ = meter.Int64Counter("mesh.messages.published")
	meshMessagesReceived, _  = meter.Int64Counter("mesh.messages.received")
)


// TeammateMesh provides the interface for agents to publish and subscribe
// to real-time communication messages across the swarm.
type TeammateMesh interface {
	Publish(ctx context.Context, channel string, data []byte) error
	Subscribe(ctx context.Context, channel string) (<-chan []byte, error)
}

// NewTeammateMesh returns a new TeammateMesh depending on the execution mode.
// If REDIS_URL is present and OHC_STANDALONE is not true, it returns a cloud mesh.
// Otherwise, it returns an in-memory mesh.
func NewTeammateMesh() (TeammateMesh, error) {
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
		slog.Info("TeammateMesh initialized in Cloud mode (Redis)")
		return &cloudMesh{client: c}, nil
	}

	slog.Info("TeammateMesh initialized in Standalone mode (In-Memory)")
	return &memoryMesh{
		channels: make(map[string][]chan []byte),
	}, nil
}

// NewTeammateMeshWithClient returns a new TeammateMesh using an existing rueidis client.
// Useful for dependency injection in testing or sharing clients.
func NewTeammateMeshWithClient(c rueidis.Client) TeammateMesh {
	if c != nil {
		return &cloudMesh{client: c}
	}
	return &memoryMesh{
		channels: make(map[string][]chan []byte),
	}
}

// memoryMesh provides a local in-memory pub/sub.
type memoryMesh struct {
	mu       sync.RWMutex
	channels map[string][]chan []byte
}

func (m *memoryMesh) Publish(ctx context.Context, channel string, data []byte) error {
	m.mu.RLock()
	defer m.mu.RUnlock()

	if meshMessagesPublished != nil {
		meshMessagesPublished.Add(ctx, 1, metric.WithAttributes(attribute.String("channel", channel), attribute.String("mode", "standalone")))
	}

	subs, ok := m.channels[channel]
	if !ok {
		return nil // No subscribers, no error
	}

	for _, sub := range subs {
		// Non-blocking send
		select {
		case sub <- data:
		case <-ctx.Done():
			return ctx.Err()
		default:
			// Dropping message if channel is full
		}
	}
	return nil
}

func (m *memoryMesh) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	out := make(chan []byte, 100)

	m.mu.Lock()
	m.channels[channel] = append(m.channels[channel], out)
	m.mu.Unlock()

	go func() {
		<-ctx.Done()
		m.mu.Lock()
		defer m.mu.Unlock()
		subs := m.channels[channel]
		for i, sub := range subs {
			if sub == out {
				m.channels[channel] = append(subs[:i], subs[i+1:]...)
				break
			}
		}
		close(out)
	}()

	// Intercept the output channel to track metrics before sending to consumer
	meteredOut := make(chan []byte, 100)
	go func() {
		defer close(meteredOut)
		for {
			select {
			case msg, ok := <-out:
				if !ok {
					return
				}
				if meshMessagesReceived != nil {
					meshMessagesReceived.Add(context.Background(), 1, metric.WithAttributes(attribute.String("channel", channel), attribute.String("mode", "standalone")))
				}
				select {
				case meteredOut <- msg:
				case <-ctx.Done():
					return
				}
			case <-ctx.Done():
				return
			}
		}
	}()

	return meteredOut, nil
}

// cloudMesh provides a Redis pub/sub backed mesh using rueidis.
type cloudMesh struct {
	client rueidis.Client
}

func (c *cloudMesh) Publish(ctx context.Context, channel string, data []byte) error {
	if meshMessagesPublished != nil {
		meshMessagesPublished.Add(ctx, 1, metric.WithAttributes(attribute.String("channel", channel), attribute.String("mode", "cloud")))
	}
	cmd := c.client.B().Publish().Channel(channel).Message(string(data)).Build()
	return c.client.Do(ctx, cmd).Error()
}

func (c *cloudMesh) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	out := make(chan []byte, 100)

	go func() {
		defer close(out)

		err := c.client.Receive(ctx, c.client.B().Subscribe().Channel(channel).Build(), func(msg rueidis.PubSubMessage) {
			if meshMessagesReceived != nil {
				meshMessagesReceived.Add(context.Background(), 1, metric.WithAttributes(attribute.String("channel", channel), attribute.String("mode", "cloud")))
			}
			select {
			case out <- []byte(msg.Message):
			case <-ctx.Done():
			}
		})

		if err != nil && err != context.Canceled {
			// Suppress expected transient errors if context is done or redis connection is closed during shutdown.
			slog.Error("Redis subscription failed", "channel", channel, "error", err)
		}
	}()

	return out, nil
}
