package models

import (
	"time"
)

type Epic struct {
	ID string `json:"id" db:"id"`
}

type EpicTask struct {
	ID            string    `json:"id" db:"id"`
	EpicID        string    `json:"epic_id" db:"epic_id"`
	Title         string    `json:"title" db:"title"`
	Status        string    `json:"status" db:"status"`
	AssignedAgent *string   `json:"assigned_agent" db:"assigned_agent"`
	CreatedAt     time.Time `json:"created_at" db:"created_at"`
	UpdatedAt     time.Time `json:"updated_at" db:"updated_at"`
}
