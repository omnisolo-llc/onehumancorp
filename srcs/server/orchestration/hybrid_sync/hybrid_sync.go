package hybrid_sync

import (
	"context"
	"encoding/json"
	"fmt"
	"onehumancorp/srcs/server/orchestration"
)

// MissionSynchronizer safely copies tasks from the local SQLite agent_missions to the Cloud Postgres DB.
type MissionSynchronizer interface {
	SyncLocalToCloud(ctx context.Context, mission *orchestration.SharedTask) error
}

type DefaultMissionSynchronizer struct {
	cloudStore orchestration.TaskStore
}

func NewMissionSynchronizer(localStore orchestration.TaskStore, cloudStore orchestration.TaskStore) *DefaultMissionSynchronizer {
	return &DefaultMissionSynchronizer{
		cloudStore: cloudStore,
	}
}

func (s *DefaultMissionSynchronizer) SyncLocalToCloud(ctx context.Context, mission *orchestration.SharedTask) error {
	if mission == nil {
		return fmt.Errorf("mission cannot be nil")
	}

	// Create a shallow copy to prevent mutating caller's memory
	cloudMission := *mission

	if cloudMission.Payload != nil {
		payloadStr := string(*cloudMission.Payload)
		sanitized, err := orchestration.SanitizePayload(payloadStr)
		if err != nil {
			return fmt.Errorf("failed to sanitize payload: %w", err)
		}

		rawSanitized := json.RawMessage(sanitized)
		cloudMission.Payload = &rawSanitized
	}

	err := s.cloudStore.CreateTask(ctx, &cloudMission)
	if err != nil {
		return fmt.Errorf("failed to create task in cloud store: %w", err)
	}

	return nil
}
