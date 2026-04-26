package models

import "time"

// Session represents an agent session stored in agent_session_data
type Session struct {
	SessionID   string    `json:"session_id" db:"session_id"`
	AgentID     string    `json:"agent_id" db:"agent_id"`
	ContextData string    `json:"context_data" db:"context_data"`
	Capabilities []string `json:"capabilities" db:"capabilities"`
	CreatedAt   time.Time `json:"created_at" db:"created_at"`
	LastAccessed time.Time `json:"last_accessed" db:"last_accessed"`
}
