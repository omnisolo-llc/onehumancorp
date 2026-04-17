package models

import "time"

type Mission struct {
	ID              string  `json:"id" db:"id"`
	EpicID          string  `json:"epic_id" db:"epic_id"`
	Title           string  `json:"title" db:"title"`
	Status          string  `json:"status" db:"status"`
	AssignedAgentID *string `json:"assigned_agent_id" db:"assigned_agent_id"`
}

type MissionDependency struct {
	MissionID   string `json:"mission_id" db:"mission_id"`
	DependsOnID string `json:"depends_on_id" db:"depends_on_id"`
}

type AutodreamVector struct {
	ID        string                 `json:"id" db:"id"`
	TaskID    string                 `json:"task_id" db:"task_id"`
	Content   string                 `json:"content" db:"content"`
	Embedding []float32              `json:"embedding" db:"embedding"`
	Metadata  map[string]interface{} `json:"metadata" db:"metadata"`
	CreatedAt time.Time              `json:"created_at" db:"created_at"`
}
