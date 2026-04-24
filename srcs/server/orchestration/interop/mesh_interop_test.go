package interop

import (
	"context"
	"testing"
	"time"

	meshpb "github.com/onehumancorp/mono/srcs/proto/mesh"
	"github.com/stretchr/testify/assert"
	"google.golang.org/protobuf/proto"
)

type mockTransport struct {
	publishFunc   func(topic string, data []byte) error
	subscribeFunc func(topic string) (<-chan []byte, error)
}

func (m *mockTransport) Publish(topic string, data []byte) error {
	return m.publishFunc(topic, data)
}

func (m *mockTransport) Subscribe(topic string) (<-chan []byte, error) {
	return m.subscribeFunc(topic)
}

func TestMeshInterop_DispatchJob(t *testing.T) {
	called := false
	transport := &mockTransport{
		publishFunc: func(topic string, data []byte) error {
			assert.Equal(t, "mesh:jobs:dispatch", topic)
			var msg meshpb.MeshJobDispatch
			err := proto.Unmarshal(data, &msg)
			assert.NoError(t, err)
			assert.Equal(t, "job-1", msg.JobId)
			assert.Equal(t, "tenant-1", msg.TenantId)
			assert.Equal(t, "payload-data", msg.Payload)
			called = true
			return nil
		},
	}

	interop := NewMeshInterop(transport)
	err := interop.DispatchJob(context.Background(), &meshpb.MeshJobDispatch{
		JobId:    "job-1",
		TenantId: "tenant-1",
		Payload:  "payload-data",
	})

	assert.NoError(t, err)
	assert.True(t, called)
}

func TestMeshInterop_SubscribeJobStatus(t *testing.T) {
	ch := make(chan []byte, 1)
	transport := &mockTransport{
		subscribeFunc: func(topic string) (<-chan []byte, error) {
			assert.Equal(t, "mesh:jobs:status", topic)
			return ch, nil
		},
	}

	interop := NewMeshInterop(transport)
	statusCh, err := interop.SubscribeJobStatus(context.Background())
	assert.NoError(t, err)

	msg := &meshpb.MeshJobStatus{
		JobId:  "job-1",
		Status: "COMPLETED",
		Result: "success",
	}
	data, _ := proto.Marshal(msg)
	ch <- data

	select {
	case received := <-statusCh:
		assert.Equal(t, "job-1", received.JobId)
		assert.Equal(t, "COMPLETED", received.Status)
		assert.Equal(t, "success", received.Result)
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for job status")
	}
}

func TestMeshInterop_SyncContext(t *testing.T) {
	called := false
	transport := &mockTransport{
		publishFunc: func(topic string, data []byte) error {
			assert.Equal(t, "mesh:context:sync", topic)
			var msg meshpb.MeshContextSync
			err := proto.Unmarshal(data, &msg)
			assert.NoError(t, err)
			assert.Equal(t, "tenant-1", msg.TenantId)
			assert.Equal(t, "mem-1", msg.MemoryId)
			called = true
			return nil
		},
	}

	interop := NewMeshInterop(transport)
	err := interop.SyncContext(context.Background(), &meshpb.MeshContextSync{
		TenantId: "tenant-1",
		MemoryId: "mem-1",
	})

	assert.NoError(t, err)
	assert.True(t, called)
}

func TestMeshInterop_SubscribeContextSync(t *testing.T) {
	ch := make(chan []byte, 1)
	transport := &mockTransport{
		subscribeFunc: func(topic string) (<-chan []byte, error) {
			assert.Equal(t, "mesh:context:sync", topic)
			return ch, nil
		},
	}

	interop := NewMeshInterop(transport)
	syncCh, err := interop.SubscribeContextSync(context.Background())
	assert.NoError(t, err)

	msg := &meshpb.MeshContextSync{
		TenantId: "tenant-1",
		MemoryId: "mem-1",
	}
	data, _ := proto.Marshal(msg)
	ch <- data

	select {
	case received := <-syncCh:
		assert.Equal(t, "tenant-1", received.TenantId)
		assert.Equal(t, "mem-1", received.MemoryId)
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for context sync")
	}
}

func TestMeshInterop_HandoffState(t *testing.T) {
	called := false
	transport := &mockTransport{
		publishFunc: func(topic string, data []byte) error {
			assert.Equal(t, "mesh:state:handoff", topic)
			var msg meshpb.MeshHandoff
			err := proto.Unmarshal(data, &msg)
			assert.NoError(t, err)
			assert.Equal(t, "tenant-1", msg.TenantId)
			assert.Equal(t, "CLOUD", msg.Mode)
			called = true
			return nil
		},
	}

	interop := NewMeshInterop(transport)
	err := interop.HandoffState(context.Background(), &meshpb.MeshHandoff{
		TenantId: "tenant-1",
		Mode:     "CLOUD",
	})

	assert.NoError(t, err)
	assert.True(t, called)
}

func TestMeshInterop_SubscribeHandoff(t *testing.T) {
	ch := make(chan []byte, 1)
	transport := &mockTransport{
		subscribeFunc: func(topic string) (<-chan []byte, error) {
			assert.Equal(t, "mesh:state:handoff", topic)
			return ch, nil
		},
	}

	interop := NewMeshInterop(transport)
	handoffCh, err := interop.SubscribeHandoff(context.Background())
	assert.NoError(t, err)

	msg := &meshpb.MeshHandoff{
		TenantId: "tenant-1",
		Mode:     "CLOUD",
	}
	data, _ := proto.Marshal(msg)
	ch <- data

	select {
	case received := <-handoffCh:
		assert.Equal(t, "tenant-1", received.TenantId)
		assert.Equal(t, "CLOUD", received.Mode)
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for state handoff")
	}
}

// Added tests to cover the unmarshal error edge cases
func TestMeshInterop_SubscribeJobStatus_UnmarshalError(t *testing.T) {
	ch := make(chan []byte, 1)
	transport := &mockTransport{
		subscribeFunc: func(topic string) (<-chan []byte, error) {
			return ch, nil
		},
	}

	interop := NewMeshInterop(transport)
	statusCh, _ := interop.SubscribeJobStatus(context.Background())

	// Send invalid data
	ch <- []byte("invalid-protobuf-data")

	// Give it a moment to process, then close channel and verify nothing was received
	close(ch)

	count := 0
	for range statusCh {
		count++
	}
	assert.Equal(t, 0, count)
}

func TestMeshInterop_SubscribeContextSync_UnmarshalError(t *testing.T) {
	ch := make(chan []byte, 1)
	transport := &mockTransport{
		subscribeFunc: func(topic string) (<-chan []byte, error) {
			return ch, nil
		},
	}

	interop := NewMeshInterop(transport)
	syncCh, _ := interop.SubscribeContextSync(context.Background())

	// Send invalid data
	ch <- []byte("invalid-protobuf-data")

	// Give it a moment to process, then close channel and verify nothing was received
	close(ch)

	count := 0
	for range syncCh {
		count++
	}
	assert.Equal(t, 0, count)
}

func TestMeshInterop_SubscribeHandoff_UnmarshalError(t *testing.T) {
	ch := make(chan []byte, 1)
	transport := &mockTransport{
		subscribeFunc: func(topic string) (<-chan []byte, error) {
			return ch, nil
		},
	}

	interop := NewMeshInterop(transport)
	handoffCh, _ := interop.SubscribeHandoff(context.Background())

	// Send invalid data
	ch <- []byte("invalid-protobuf-data")

	// Give it a moment to process, then close channel and verify nothing was received
	close(ch)

	count := 0
	for range handoffCh {
		count++
	}
	assert.Equal(t, 0, count)
}
