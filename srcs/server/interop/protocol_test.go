package interop

import (
	"context"
	"sync"
	"testing"
	"time"

	pb "onehumancorp/srcs/server/interop/pb"
	"github.com/stretchr/testify/assert"
	"github.com/golang/protobuf/proto"
)

type MockTransport struct {
	mu       sync.Mutex
	subs     map[string]func([]byte)
	published [][]byte
	locks    map[string]string
}

func NewMockTransport() *MockTransport {
	return &MockTransport{
		subs:  make(map[string]func([]byte)),
		locks: make(map[string]string),
	}
}

func (m *MockTransport) Publish(ctx context.Context, channel string, data []byte) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.published = append(m.published, data)
	if handler, ok := m.subs[channel]; ok {
		go handler(data)
	}
	return nil
}

func (m *MockTransport) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	m.mu.Lock()
	m.subs[channel] = handler
	m.mu.Unlock()
	return nil
}

func (m *MockTransport) AcquireLock(ctx context.Context, resource, owner string, ttlSeconds int) (bool, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if current, ok := m.locks[resource]; ok && current != owner {
		return false, nil
	}
	m.locks[resource] = owner
	return true, nil
}

func (m *MockTransport) ReleaseLock(ctx context.Context, resource, owner string) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	if current, ok := m.locks[resource]; ok && current == owner {
		delete(m.locks, resource)
	}
	return nil
}

func TestInteropProtocol_Handoff(t *testing.T) {
	mock := NewMockTransport()
	protocol := NewInteropProtocol(mock, "node_1")

	err := protocol.Handoff(context.Background(), "mission_123", "tenant_x", []byte(`{"state": "test"}`))
	assert.NoError(t, err)

	mock.mu.Lock()
	defer mock.mu.Unlock()
	assert.Len(t, mock.published, 1)

	var msg pb.StateHandoff
	err = proto.Unmarshal(mock.published[0], &msg)
	assert.NoError(t, err)
	assert.Equal(t, "mission_123", msg.MissionId)
	assert.Equal(t, "tenant_x", msg.TenantId)
	assert.Equal(t, []byte(`{"state": "test"}`), msg.StateSnapshotJson)
}

func TestInteropProtocol_CheckHealth(t *testing.T) {
	mock := NewMockTransport()
	protocol := NewInteropProtocol(mock, "node_1")

	// Simulate Ack handler
	go func() {
		time.Sleep(50 * time.Millisecond)
		mock.mu.Lock()
		handler := mock.subs["system:health_ping"]
		mock.mu.Unlock()
		if handler != nil {
			handler([]byte{})
		} else {
			// fallback directly call the ack topic
			mock.Publish(context.Background(), "system:health_ack:node_1", []byte{})
		}
	}()

	ok, err := protocol.CheckHealth(context.Background(), 500)
	assert.NoError(t, err)
	assert.True(t, ok)
}

func TestInteropProtocol_DispatchJob(t *testing.T) {
	mock := NewMockTransport()
	protocol := NewInteropProtocol(mock, "node_1")

	// Simulate Ack handler
	go func() {
		time.Sleep(50 * time.Millisecond)
		mock.Publish(context.Background(), "system:job_ack:job_123", []byte{})
	}()

	ok, err := protocol.DispatchJob(context.Background(), "job_123", "tenant_x", "action_1", []byte(`{}`), 500)
	assert.NoError(t, err)
	assert.True(t, ok)
}
