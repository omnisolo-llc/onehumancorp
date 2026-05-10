package cloudsync

import (
	"context"
	"time"
)

// MissionPayload defines the structure of the task payload
type MissionPayload struct {
	Role    string `json:"role"`
	Task    string `json:"task"`
	Context string `json:"context,omitempty"`
}

// LocalMission represents a row in the local agent_missions table
type LocalMission struct {
	ID             string
	Status         string
	Payload        MissionPayload
	CreatedAt      time.Time
	SyncedToCloud  bool
	CloudMissionID string
	SyncError      string
	LastSyncedAt   time.Time
}

// CloudSynchronizer handles pushing local missions to the cloud and pulling updates
type CloudSynchronizer interface {
	// PushPendingMissions finds tasks marked for escalation and sends them to the cloud
	PushPendingMissions(ctx context.Context) error

	// PullMissionUpdates polls the cloud for updates to previously escalated tasks
	PullMissionUpdates(ctx context.Context) error
}

// LocalRepository defines the interface for interacting with the local SQLite agent_missions table
type LocalRepository interface {
	GetPendingSync(ctx context.Context, limit int) ([]LocalMission, error)
	MarkSynced(ctx context.Context, localID string, cloudID string) error
	MarkSyncError(ctx context.Context, localID string, syncError string) error
	GetActiveEscalations(ctx context.Context) ([]LocalMission, error)
	UpdateLocalStatus(ctx context.Context, localID string, newStatus string) error
}
