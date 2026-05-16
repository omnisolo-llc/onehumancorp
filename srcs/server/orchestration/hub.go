package orchestration

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"sync"

	"github.com/redis/rueidis"
)

var (
	ErrTopicEmpty   = errors.New("topic cannot be empty")
	ErrMessageEmpty = errors.New("message cannot be empty")
)

type MeshMessage struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
	Payload []byte `json:"payload"`
	MsgID   string `json:"msg_id"`
}

type MeshTransport interface {
	Publish(ctx context.Context, topic string, message MeshMessage) error
	Subscribe(ctx context.Context, topic string, handler func(MeshMessage)) (func(), error)
	Close() error
}

type RedisMeshTransport struct {
	client rueidis.Client
	mu     sync.Mutex
	closed bool
}

func NewRedisMeshTransport(url string) (*RedisMeshTransport, error) {
	if url == "" {
		url = "127.0.0.1:6379"
	}
	client, err := rueidis.NewClient(rueidis.ClientOption{InitAddress: []string{url}})
	if err != nil {
		return nil, fmt.Errorf("failed to init redis client: %w", err)
	}
	return &RedisMeshTransport{client: client}, nil
}

func (r *RedisMeshTransport) Publish(ctx context.Context, topic string, message MeshMessage) error {
	r.mu.Lock()
	if r.closed {
		r.mu.Unlock()
		return errors.New("transport closed")
	}
	r.mu.Unlock()

	if topic == "" {
		return ErrTopicEmpty
	}
	data, err := json.Marshal(message)
	if err != nil {
		return fmt.Errorf("marshal err: %w", err)
	}
	cmd := r.client.B().Publish().Channel(topic).Message(string(data)).Build()
	return r.client.Do(ctx, cmd).Error()
}

func (r *RedisMeshTransport) Subscribe(ctx context.Context, topic string, handler func(MeshMessage)) (func(), error) {
	r.mu.Lock()
	if r.closed {
		r.mu.Unlock()
		return nil, errors.New("transport closed")
	}
	r.mu.Unlock()

	if topic == "" {
		return nil, ErrTopicEmpty
	}

	cancelCtx, cancel := context.WithCancel(ctx)
	go func() {
		err := r.client.Receive(cancelCtx, r.client.B().Subscribe().Channel(topic).Build(), func(msg rueidis.PubSubMessage) {
			var m MeshMessage
			if err := json.Unmarshal([]byte(msg.Message), &m); err == nil {
				handler(m)
			} else {
				log.Printf("failed to unmarshal redis pubsub message: %v", err)
			}
		})
		if err != nil && err != context.Canceled {
			log.Printf("redis subscribe error: %v", err)
		}
	}()

	return cancel, nil
}

func (r *RedisMeshTransport) Close() error {
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.closed {
		return nil
	}
	r.closed = true
	r.client.Close()
	return nil
}

type MemoryMeshTransport struct {
	subscribers map[string][]func(MeshMessage)
	mu          sync.RWMutex
	closed      bool
}

func NewMemoryMeshTransport() *MemoryMeshTransport {
	return &MemoryMeshTransport{
		subscribers: make(map[string][]func(MeshMessage)),
	}
}

func (m *MemoryMeshTransport) Publish(ctx context.Context, topic string, message MeshMessage) error {
	m.mu.RLock()
	defer m.mu.RUnlock()
	if m.closed {
		return errors.New("transport closed")
	}
	if topic == "" {
		return ErrTopicEmpty
	}
	if _, ok := m.subscribers[topic]; ok {
		for _, h := range m.subscribers[topic] {
			go h(message)
		}
	}
	return nil
}

func (m *MemoryMeshTransport) Subscribe(ctx context.Context, topic string, handler func(MeshMessage)) (func(), error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if m.closed {
		return nil, errors.New("transport closed")
	}
	if topic == "" {
		return nil, ErrTopicEmpty
	}

	m.subscribers[topic] = append(m.subscribers[topic], handler)
	return func() {
		m.mu.Lock()
		defer m.mu.Unlock()
		if _, ok := m.subscribers[topic]; ok {
			var newHandlers []func(MeshMessage)
			m.subscribers[topic] = newHandlers
		}
	}, nil
}

func (m *MemoryMeshTransport) Close() error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.closed = true
	m.subscribers = nil
	return nil
}
