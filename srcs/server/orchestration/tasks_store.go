package orchestration

import (
	"context"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type TaskStore interface {
	ClaimTask(ctx context.Context, agentID string) (*SharedTaskDB, error)
	TransitionTask(ctx context.Context, taskID, agentID, fromState, toState, reason string) error
	ClaimDecompositionTask(ctx context.Context, agentID string) (*SharedTaskDecomposition, error)
}

func NewTaskStore(dbProvider db.Provider) TaskStore {
	return NewSharedTaskOrchestrator(dbProvider)
}
