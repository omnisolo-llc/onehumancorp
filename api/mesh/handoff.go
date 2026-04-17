package mesh

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
)

// StateHandoffManager coordinates the transfer of state between Hybrid environments.
type StateHandoffManager struct {
	cloudMesh *TeammateMesh
	localMesh *TeammateMesh
}

// NewStateHandoffManager creates a new StateHandoffManager.
func NewStateHandoffManager(cloudMesh, localMesh *TeammateMesh) *StateHandoffManager {
	return &StateHandoffManager{
		cloudMesh: cloudMesh,
		localMesh: localMesh,
	}
}

// HandoffToCloud escalates a local state to the cloud mesh.
func (m *StateHandoffManager) HandoffToCloud(ctx context.Context, agentID, channel string, data interface{}) error {
	if m.cloudMesh == nil {
		return fmt.Errorf("cloud mesh is not configured")
	}

	payload, err := json.Marshal(data)
	if err != nil {
		return fmt.Errorf("failed to marshal data for cloud handoff: %w", err)
	}

	msg := MeshMessage{
		SenderID: agentID,
		Topic:    channel,
		Payload:  payload,
	}

	err = m.cloudMesh.Publish(ctx, msg)
	if err != nil {
		return fmt.Errorf("failed to publish to cloud mesh: %w", err)
	}

	log.Printf("Successfully escalated state to cloud for channel %s", channel)
	return nil
}

// HandoffToLocal downgrades a cloud state to the local mesh.
func (m *StateHandoffManager) HandoffToLocal(ctx context.Context, agentID, channel string, data interface{}) error {
	if m.localMesh == nil {
		return fmt.Errorf("local mesh is not configured")
	}

	payload, err := json.Marshal(data)
	if err != nil {
		return fmt.Errorf("failed to marshal data for local handoff: %w", err)
	}

	msg := MeshMessage{
		SenderID: agentID,
		Topic:    channel,
		Payload:  payload,
	}

	err = m.localMesh.Publish(ctx, msg)
	if err != nil {
		return fmt.Errorf("failed to publish to local mesh: %w", err)
	}

	log.Printf("Successfully downgraded state to local for channel %s", channel)
	return nil
}
