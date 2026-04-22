package departments

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// DepartmentType represents the specific type/function of an AI Department.
type DepartmentType string

const (
	DepartmentOperations DepartmentType = "operations"
	DepartmentMarketing  DepartmentType = "marketing"
	DepartmentSales      DepartmentType = "sales"
	DepartmentSuccess    DepartmentType = "customer_success"
	DepartmentFinance    DepartmentType = "finance"
	DepartmentLegal      DepartmentType = "legal"
	DepartmentAdvisory   DepartmentType = "business_advisory"
)

// DraftAction represents an action drafted by an AI Department that requires user approval.
type DraftAction struct {
	ID             string            `json:"id"`
	DepartmentType DepartmentType    `json:"department_type"`
	AgentID        string            `json:"agent_id"`
	ActionType     string            `json:"action_type"`
	Payload        map[string]string `json:"payload"`
	Status         string            `json:"status"` // e.g. "draft", "approved", "rejected"
	Description    string            `json:"description"`
}

// MemoryContext represents the context retrieved from the shared memory layer (pgvector).
type MemoryContext struct {
	RelevantEvents []orchestration.Message `json:"relevant_events"`
	Context        string                  `json:"context"`
}

// HubPublisher provides the ability to publish messages, implemented by orchestration.Hub.
type HubPublisher interface {
	Publish(msg orchestration.Message) error
}

// MemoryLayer abstracts access to the shared pgvector memory.
type MemoryLayer interface {
	Retrieve(ctx context.Context, query string) (*MemoryContext, error)
	SaveEvent(ctx context.Context, event orchestration.Message) error
}

// ReviewCenterLayer abstracts the UI Action Review Center.
type ReviewCenterLayer interface {
	ReceiveDraft(ctx context.Context, action DraftAction) error
}

// Department is the core interface for an AI Department.
type Department interface {
	// ID returns the unique identifier for the department.
	ID() string

	// Type returns the functional type of the department.
	Type() DepartmentType

	// HandleEvent processes an incoming event from the orchestration Hub.
	HandleEvent(ctx context.Context, event orchestration.Message) error

	// RetrieveMemoryContext fetches relevant context from the shared memory layer for a given query.
	RetrieveMemoryContext(ctx context.Context, query string) (*MemoryContext, error)

	// EmitDraftAction proposes an action for user review in the Action Review Center.
	EmitDraftAction(ctx context.Context, action DraftAction) error

	// Start initializes the department and registers it with the orchestration Hub.
	Start(ctx context.Context, hub HubPublisher) error
}
