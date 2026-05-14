package orchestration

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/redis/rueidis"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
    // "google.golang.org/protobuf/proto" // Removed proto dependency to keep standalone testing simple
)

// MeshEvent defines the struct matching the proto definition
type MeshEvent struct {
    Id string
    Topic string
    Payload []byte
    TimestampUnix int64
    SenderId string
}

var (
	meter         = otel.Meter("orchestration-mesh")
	publishOps, _ = meter.Int64Counter("mesh_publish_ops_total")
	publishErr, _ = meter.Int64Counter("mesh_publish_errors_total")
    subscribeOps, _ = meter.Int64Counter("mesh_subscribe_ops_total")
)

type MeshTransport interface {
	Publish(ctx context.Context, topic string, event MeshEvent) error
	Subscribe(ctx context.Context, topic string, handler func(event MeshEvent)) error
	Close() error
}

type RedisMeshTransport struct {
	client rueidis.Client
	addr   string
}

func NewRedisMeshTransport(addr string) (*RedisMeshTransport, error) {
	client, err := rueidis.NewClient(rueidis.ClientOption{
        InitAddress: []string{addr},
        DisableCache: true, // required for basic redis instances like miniredis
    })
	if err != nil {
		return nil, fmt.Errorf("failed to create redis client: %w", err)
	}
	return &RedisMeshTransport{client: client, addr: addr}, nil
}

func (r *RedisMeshTransport) Publish(ctx context.Context, topic string, event MeshEvent) error {
	publishOps.Add(ctx, 1, metric.WithAttributes())

	if r.client == nil {
        publishErr.Add(ctx, 1, metric.WithAttributes())
		return fmt.Errorf("redis client is nil")
	}

	err := r.client.Do(ctx, r.client.B().Publish().Channel(topic).Message(string(event.Payload)).Build()).Error()
	if err != nil {
		publishErr.Add(ctx, 1, metric.WithAttributes())
	}
	return err
}

func (r *RedisMeshTransport) Subscribe(ctx context.Context, topic string, handler func(event MeshEvent)) error {
    subscribeOps.Add(ctx, 1, metric.WithAttributes())
	go func() {
		client, err := rueidis.NewClient(rueidis.ClientOption{
            InitAddress: []string{r.addr},
            DisableCache: true,
        })
		if err != nil {
			return
		}
		defer client.Close()
        err = client.Receive(ctx, client.B().Subscribe().Channel(topic).Build(), func(msg rueidis.PubSubMessage) {
			handler(MeshEvent{Payload: []byte(msg.Message)})
		})
        if err != nil {
            fmt.Println("Subscribe error:", err)
        }
	}()
	return nil
}

func (r *RedisMeshTransport) Close() error {
	if r.client != nil {
		r.client.Close()
	}
	return nil
}

type MemoryMeshTransport struct {
	mu          sync.RWMutex
	subscribers map[string][]func(event MeshEvent)
}

func NewMemoryMeshTransport() *MemoryMeshTransport {
	return &MemoryMeshTransport{
		subscribers: make(map[string][]func(event MeshEvent)),
	}
}

func (m *MemoryMeshTransport) Publish(ctx context.Context, topic string, event MeshEvent) error {
	publishOps.Add(ctx, 1, metric.WithAttributes())
	m.mu.RLock()
	subs := m.subscribers[topic]
	m.mu.RUnlock()

	for _, handler := range subs {
		h := handler
		go func() {
			time.Sleep(time.Millisecond) // Simulate async
			h(event)
		}()
	}
	return nil
}

func (m *MemoryMeshTransport) Subscribe(ctx context.Context, topic string, handler func(event MeshEvent)) error {
    subscribeOps.Add(ctx, 1, metric.WithAttributes())
	m.mu.Lock()
	m.subscribers[topic] = append(m.subscribers[topic], handler)
	m.mu.Unlock()
	return nil
}

func (m *MemoryMeshTransport) Close() error {
	return nil
}
