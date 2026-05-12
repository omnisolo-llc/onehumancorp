package sync

import (
	"time"
)

// SyncDelta represents a single state change (delta) from an MCP tool.
// It includes tenant isolation and robust metadata.
type SyncDelta struct {
	ID         string    `json:"id"`
	TenantID   string    `json:"tenant_id"` // Multi-tenant safety
	EntityID   string    `json:"entity_id"`
	EntityType string    `json:"entity_type"`
	Operation  string    `json:"operation"` // e.g., "create", "update", "delete"
	Data       string    `json:"data"`      // JSON payload
	UpdatedAt  time.Time `json:"updated_at"`
	Source     string    `json:"source"`    // e.g., "sqlite-standalone"
}

// SyncResponse is the response returned to the client after syncing.
type SyncResponse struct {
	Success bool   `json:"success"`
	Message string `json:"message"`
	Synced  int    `json:"synced"`
}

// ErrorResponse is a machine-readable error response.
type ErrorResponse struct {
	Error       string `json:"error"`
	Description string `json:"description"`
	Code        int    `json:"code"`
}
