package kairos

import "time"

// MissionStatus represents the current state of a mission
type MissionStatus string

const (
	MissionStatusPending    MissionStatus = "PENDING"
	MissionStatusAssigned   MissionStatus = "ASSIGNED"
	MissionStatusInProgress MissionStatus = "IN_PROGRESS"
	MissionStatusReview     MissionStatus = "REVIEW"
	MissionStatusCompleted  MissionStatus = "COMPLETED"
	MissionStatusFailed     MissionStatus = "FAILED"
)

// Mission represents a decomposed task
type Mission struct {
	ID              string        `json:"id"`
	EpicID          string        `json:"epic_id"`
	Title           string        `json:"title"`
	Status          MissionStatus `json:"status"`
	AssignedAgentID string        `json:"assigned_agent_id,omitempty"`
	CreatedAt       time.Time     `json:"created_at"`
	UpdatedAt       time.Time     `json:"updated_at"`
}

// MissionDependency represents a directed edge in the mission DAG
type MissionDependency struct {
	ID                 string    `json:"id"`
	MissionID          string    `json:"mission_id"`
	DependsOnMissionID string    `json:"depends_on_mission_id"`
	CreatedAt          time.Time `json:"created_at"`
}

// AutodreamVector stores vectorized state of completed missions
type AutodreamVector struct {
	ID        string    `json:"id"`
	MissionID string    `json:"mission_id"`
	Embedding []float32 `json:"embedding"` // pgvector representation
	CreatedAt time.Time `json:"created_at"`
}
