package orchestration

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type mockStream struct {
	ctx     context.Context
	events  []*MeshEvent
	done    chan struct{}
	maxRecv int
}

func (m *mockStream) Context() context.Context {
	return m.ctx
}

func (m *mockStream) Send(event *MeshEvent) error {
	m.events = append(m.events, event)
	if len(m.events) >= m.maxRecv {
		close(m.done)
	}
	return nil
}

func TestAdvertiseCapabilities(t *testing.T) {
	mesh := NewLocalTeammateMesh()
	server := NewTeammateMeshServer(mesh)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	var receivedData []byte
	mesh.Subscribe(ctx, "mesh:presence", func(data []byte) {
		receivedData = data
	})

	req := &AgentCapabilities{
		AgentId:      "agent-1",
		Capabilities: []string{"test-cap"},
	}

	_, err := server.AdvertiseCapabilities(ctx, req)
	require.NoError(t, err)

	time.Sleep(10 * time.Millisecond)
	assert.NotEmpty(t, receivedData)

	var event MeshEvent
	err = json.Unmarshal(receivedData, &event)
	require.NoError(t, err)
	assert.Equal(t, "CAPABILITIES_ADVERTISED", event.EventType)
	assert.Equal(t, "agent-1", event.AgentId)
}

func TestStreamMeshEvents(t *testing.T) {
	mesh := NewLocalTeammateMesh()
	server := NewTeammateMeshServer(mesh)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	stream := &mockStream{
		ctx:     ctx,
		done:    make(chan struct{}),
		maxRecv: 1,
	}

	req := &EventStreamRequest{
		AgentId:  "agent-1",
		Channels: []string{"mesh:tasks"},
	}

	go func() {
		err := server.StreamMeshEvents(req, stream)
		if err != nil && err != context.Canceled {
			t.Errorf("StreamMeshEvents failed: %v", err)
		}
	}()

	time.Sleep(10 * time.Millisecond)

	event := MeshEvent{
		EventType: "TEST",
		AgentId:   "agent-2",
	}
	data, _ := json.Marshal(event)
	mesh.Publish(ctx, "mesh:tasks", data)

	select {
	case <-stream.done:
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for stream to receive event")
	}

	assert.Len(t, stream.events, 1)
	assert.Equal(t, "TEST", stream.events[0].EventType)
}
