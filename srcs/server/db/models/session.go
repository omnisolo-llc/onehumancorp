package models

import "time"

type Session struct {
	SessionID    string    `json:"session_id"`
	AgentID      string    `json:"agent_id"`
	ContextData  string    `json:"context_data"`
	Capabilities []string  `json:"capabilities"`
	CreatedAt    time.Time `json:"created_at"`
	LastAccessed time.Time `json:"last_accessed"`
}
