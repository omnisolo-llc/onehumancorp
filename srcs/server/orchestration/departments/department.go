package departments

import (
	"context"
)

// Department interface for AI Agent Departments.
type Department interface {
	// HandleEvent processes an incoming event (e.g., from Redis Pub/Sub).
	HandleEvent(ctx context.Context, tenantID, eventType string, payload []byte) error

	// RetrieveMemoryContext retrieves shared context from pgvector memory layer.
	RetrieveMemoryContext(ctx context.Context, tenantID, query string, limit int) ([]string, error)

	// EmitDraftAction proposes an action that requires user approval via the Action Review Center.
	EmitDraftAction(ctx context.Context, tenantID, actionType string, details []byte) error
}
