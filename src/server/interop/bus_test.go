package interop

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/proto/agentservice"
	"github.com/stretchr/testify/assert"
	"google.golang.org/protobuf/proto"
)

func TestInteropBus_DispatchJob(t *testing.T) {
	mesh, err := NewTeammateMesh()
	assert.NoError(t, err)

	bus := NewInteropBus(mesh)
	ctx := context.Background()

	req := &agentservicepb.RunTaskRequest{
		TaskId: "test-task-1",
		Task:   "Write a poem",
		Model:  "gpt-4",
	}

	sub, err := mesh.Subscribe(ctx, "job.dispatch.test-task-1")
	assert.NoError(t, err)

	err = bus.DispatchJob(ctx, req)
	assert.NoError(t, err)

	select {
	case msg := <-sub:
		var receivedReq agentservicepb.RunTaskRequest
		err = proto.Unmarshal(msg, &receivedReq)
		assert.NoError(t, err)
		assert.Equal(t, req.TaskId, receivedReq.TaskId)
		assert.Equal(t, req.Task, receivedReq.Task)
	case <-time.After(time.Second):
		t.Fatal("timeout waiting for job dispatch message")
	}
}

func TestInteropBus_ReportStatus(t *testing.T) {
	mesh, err := NewTeammateMesh()
	assert.NoError(t, err)

	bus := NewInteropBus(mesh)
	ctx := context.Background()

	event := &agentservicepb.RunTaskEvent{
		Type:    agentservicepb.EventType_TASK_COMPLETE,
		Content: "Done",
	}

	sub, err := mesh.Subscribe(ctx, "job.status")
	assert.NoError(t, err)

	err = bus.ReportStatus(ctx, event)
	assert.NoError(t, err)

	select {
	case msg := <-sub:
		var receivedEvent agentservicepb.RunTaskEvent
		err = proto.Unmarshal(msg, &receivedEvent)
		assert.NoError(t, err)
		assert.Equal(t, event.Type, receivedEvent.Type)
		assert.Equal(t, event.Content, receivedEvent.Content)
	case <-time.After(time.Second):
		t.Fatal("timeout waiting for status report message")
	}
}

func TestInteropBus_HandoffState(t *testing.T) {
	mesh, err := NewTeammateMesh()
	assert.NoError(t, err)

	bus := NewInteropBus(mesh)
	ctx := context.Background()

	state := &State{
		ID:    "state-1",
		Owner: "owner-A",
	}

	sub, err := mesh.Subscribe(ctx, "state.handoff.state-1")
	assert.NoError(t, err)

	err = bus.HandoffState(ctx, state)
	assert.NoError(t, err)

	select {
	case <-sub:
		// Message received successfully
	case <-time.After(time.Second):
		t.Fatal("timeout waiting for state handoff message")
	}
}
