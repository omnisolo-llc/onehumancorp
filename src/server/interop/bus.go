package interop

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/onehumancorp/mono/src/proto/agentservice"
	"google.golang.org/protobuf/proto"
)

// DefaultInteropBus is the standard implementation of the InteropBus interface.
type DefaultInteropBus struct {
	mesh TeammateMesh
}

// NewInteropBus creates a new InteropBus based on the available TeammateMesh.
func NewInteropBus(mesh TeammateMesh) *DefaultInteropBus {
	return &DefaultInteropBus{mesh: mesh}
}

// DispatchJob sends a RunTaskRequest to an agent asynchronously.
func (b *DefaultInteropBus) DispatchJob(ctx context.Context, req *agentservicepb.RunTaskRequest) error {
	data, err := proto.Marshal(req)
	if err != nil {
		return fmt.Errorf("failed to marshal RunTaskRequest: %w", err)
	}

	channel := fmt.Sprintf("job.dispatch.%s", req.TaskId)
	err = b.mesh.Publish(ctx, channel, data)
	if err != nil {
		slog.Error("Failed to dispatch job", "taskId", req.TaskId, "error", err)
		return err
	}
	slog.Info("Dispatched job successfully", "taskId", req.TaskId)
	return nil
}

// ReportStatus sends a RunTaskEvent back to the orchestrator.
func (b *DefaultInteropBus) ReportStatus(ctx context.Context, event *agentservicepb.RunTaskEvent) error {
	data, err := proto.Marshal(event)
	if err != nil {
		return fmt.Errorf("failed to marshal RunTaskEvent: %w", err)
	}

	channel := "job.status"
	err = b.mesh.Publish(ctx, channel, data)
	if err != nil {
		slog.Error("Failed to report status", "eventType", event.Type, "error", err)
		return err
	}
	return nil
}

// HandoffState synchronizes agent state when switching between modes.
func (b *DefaultInteropBus) HandoffState(ctx context.Context, state *State) error {
	// Simple JSON-based handoff mechanism as a placeholder
	// In reality, this would likely serialize the state into protobuf or
	// another binary format and handle more complex reconciliation.
	data := []byte(fmt.Sprintf(`{"id": "%s", "owner": "%s"}`, state.ID, state.Owner))
	channel := fmt.Sprintf("state.handoff.%s", state.ID)

	err := b.mesh.Publish(ctx, channel, data)
	if err != nil {
		slog.Error("Failed to handoff state", "stateId", state.ID, "error", err)
		return err
	}
	slog.Info("State handoff initiated", "stateId", state.ID)
	return nil
}
