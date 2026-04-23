package orchestration

import (
	"context"
)

// DepartmentEvent represents a domain event in the department architecture.
type DepartmentEvent struct {
	Type    string                 `json:"type"` // e.g., "order.created"
	Payload map[string]interface{} `json:"payload"`
}

// DraftAction represents a drafted action that requires user approval.
type DraftAction struct {
	ID           string                 `json:"id"`
	Department   string                 `json:"department"`
	Description  string                 `json:"description"`
	Payload      map[string]interface{} `json:"payload"`
	Status       string                 `json:"status"` // "pending", "approved", "rejected"
}

// Department defines the core interface for an AI Department.
type Department interface {
	// Name returns the name of the department.
	Name() string

	// HandleEvent processes an incoming domain event.
	HandleEvent(ctx context.Context, event DepartmentEvent) error

	// RetrieveMemoryContext retrieves shared context from the pgvector memory layer.
	RetrieveMemoryContext(ctx context.Context, query string) (string, error)

	// EmitDraftAction emits a drafted action for the user to review.
	EmitDraftAction(ctx context.Context, action DraftAction) error
}
