package orchestration

import (
	"context"
	"fmt"
	"sync"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/redis/rueidis"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

type MeshTransport interface {
	Publish(ctx context.Context, topic string, payload []byte) error
	Subscribe(ctx context.Context, topic string, handler func([]byte)) (func(), error)
}

type RedisMeshTransport struct {
	client rueidis.Client
	mu     sync.RWMutex
    pubCounter metric.Int64Counter
    subCounter metric.Int64Counter
    pubLatency metric.Float64Histogram
    subLatency metric.Float64Histogram
}

func NewRedisMeshTransport(url string) (*RedisMeshTransport, error) {
	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress: []string{url},
	})
	if err != nil {
		return nil, err
	}
	meter := otel.Meter("mesh.transport")
	pubCounter, _ := meter.Int64Counter("mesh.publish.count")
	subCounter, _ := meter.Int64Counter("mesh.subscribe.count")
	pubLatency, _ := meter.Float64Histogram("mesh.publish.latency")
	subLatency, _ := meter.Float64Histogram("mesh.subscribe.latency")
	return &RedisMeshTransport{
		client: client,
		pubCounter: pubCounter,
		subCounter: subCounter,
		pubLatency: pubLatency,
		subLatency: subLatency,
	}, nil
}

func (r *RedisMeshTransport) Publish(ctx context.Context, topic string, payload []byte) error {
	if r.pubCounter != nil {
		r.pubCounter.Add(ctx, 1)
	}

	start := time.Now()
	defer func() {
		if r.pubLatency != nil {
			r.pubLatency.Record(ctx, time.Since(start).Seconds())
		}
	}()

	cmd := r.client.B().Publish().Channel(topic).Message(string(payload)).Build()
	return r.client.Do(ctx, cmd).Error()
}

func (r *RedisMeshTransport) Subscribe(ctx context.Context, topic string, handler func([]byte)) (func(), error) {
	if r.subCounter != nil {
		r.subCounter.Add(ctx, 1)
	}

	subCtx, cancel := context.WithCancel(ctx)

	go func() {
		err := r.client.Receive(subCtx, r.client.B().Subscribe().Channel(topic).Build(), func(msg rueidis.PubSubMessage) {
			start := time.Now()
			handler([]byte(msg.Message))
			if r.subLatency != nil {
				r.subLatency.Record(context.Background(), time.Since(start).Seconds())
			}
		})
		if err != nil {
			fmt.Printf("Redis subscribe error: %v\n", err)
		}
	}()

	return func() {
		cancel()
	}, nil
}

type MemoryMeshTransport struct {
	mu          sync.RWMutex
	subscribers map[string]map[int]func([]byte)
	nextID      int
	pubCounter  metric.Int64Counter
	subCounter  metric.Int64Counter
	pubLatency  metric.Float64Histogram
	subLatency  metric.Float64Histogram
}

func NewMemoryMeshTransport() *MemoryMeshTransport {
	meter := otel.Meter("mesh.transport")
	pubCounter, _ := meter.Int64Counter("mesh.publish.count")
	subCounter, _ := meter.Int64Counter("mesh.subscribe.count")
	pubLatency, _ := meter.Float64Histogram("mesh.publish.latency")
	subLatency, _ := meter.Float64Histogram("mesh.subscribe.latency")
	return &MemoryMeshTransport{
		subscribers: make(map[string]map[int]func([]byte)),
		pubCounter:  pubCounter,
		subCounter:  subCounter,
		pubLatency:  pubLatency,
		subLatency:  subLatency,
	}
}

func (m *MemoryMeshTransport) Publish(ctx context.Context, topic string, payload []byte) error {
	if m.pubCounter != nil {
		m.pubCounter.Add(ctx, 1)
	}

	start := time.Now()
	defer func() {
		if m.pubLatency != nil {
			m.pubLatency.Record(ctx, time.Since(start).Seconds())
		}
	}()

	m.mu.RLock()
	var handlers []func([]byte)
	if subs, ok := m.subscribers[topic]; ok {
		for _, sub := range subs {
			handlers = append(handlers, sub)
		}
	}
	m.mu.RUnlock()

	for _, sub := range handlers {
		go sub(payload)
	}
	return nil
}

func (m *MemoryMeshTransport) Subscribe(ctx context.Context, topic string, handler func([]byte)) (func(), error) {
	if m.subCounter != nil {
		m.subCounter.Add(ctx, 1)
	}

	m.mu.Lock()
	defer m.mu.Unlock()

	if m.subscribers[topic] == nil {
		m.subscribers[topic] = make(map[int]func([]byte))
	}
	m.nextID++
	id := m.nextID
	m.subscribers[topic][id] = handler

	return func() {
		m.mu.Lock()
		defer m.mu.Unlock()
		if subs, ok := m.subscribers[topic]; ok {
			delete(subs, id)
			if len(subs) == 0 {
				delete(m.subscribers, topic)
			}
		}
	}, nil
}

type CentrifugeNode struct {
	transport MeshTransport
	wsClients map[string]interface{}
	mu        sync.RWMutex
}

func NewCentrifugeNode(transport MeshTransport) *CentrifugeNode {
	return &CentrifugeNode{
		transport: transport,
		wsClients: make(map[string]interface{}),
	}
}

func (c *CentrifugeNode) RegisterWebSocketClient(clientID string, client interface{}) {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.wsClients[clientID] = client
}

func (c *CentrifugeNode) RemoveWebSocketClient(clientID string) {
	c.mu.Lock()
	defer c.mu.Unlock()
	delete(c.wsClients, clientID)
}

func (c *CentrifugeNode) Broadcast(ctx context.Context, topic string, event *pb.MeshEvent) error {
	return c.transport.Publish(ctx, topic, event.Payload)
}

type TaskManager struct {
	mesh *CentrifugeNode
}

func NewTaskManager(mesh *CentrifugeNode) *TaskManager {
	return &TaskManager{mesh: mesh}
}

func (tm *TaskManager) CreateTask(ctx context.Context, taskID string, payload []byte) error {
	event := &pb.MeshEvent{
		EventId:   fmt.Sprintf("evt_%s", taskID),
		Topic:     "task.created",
		Payload:   payload,
		Timestamp: time.Now().Unix(),
	}
	return tm.mesh.Broadcast(ctx, "task.created", event)
}


type HubServiceServerImpl struct {
    pb.UnimplementedHubServiceServer
    transport MeshTransport
}

func NewHubServiceServer(transport MeshTransport) *HubServiceServerImpl {
    return &HubServiceServerImpl{transport: transport}
}

func (s *HubServiceServerImpl) StreamMeshEvents(req *pb.EventStreamRequest, stream pb.HubService_StreamMeshEventsServer) error {
    ch := make(chan []byte, 100)
    unsub, err := s.transport.Subscribe(stream.Context(), req.Topic, func(msg []byte) {
        ch <- msg
    })
    if err != nil {
        return err
    }
    defer unsub()

    for {
        select {
        case <-stream.Context().Done():
            return nil
        case msg := <-ch:
            err := stream.Send(&pb.MeshEvent{
                Topic: req.Topic,
                Payload: msg,
                Timestamp: time.Now().Unix(),
            })
            if err != nil {
                return err
            }
        }
    }
}

func (s *HubServiceServerImpl) PublishMeshEvent(ctx context.Context, req *pb.PublishMeshEventRequest) (*pb.PublishMessageResponse, error) {
    if req == nil || req.Event == nil {
        return &pb.PublishMessageResponse{Success: false}, fmt.Errorf("invalid request")
    }
    err := s.transport.Publish(ctx, req.Event.Topic, req.Event.Payload)
    return &pb.PublishMessageResponse{Success: err == nil}, err
}
