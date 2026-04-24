package db

import (
	"context"
)

// SharedTaskRepository defines the interface for managing shared tasks.
type SharedTaskRepository interface {
	AcquireTask(ctx context.Context, organizationID, agentID string) (*TaskRecord, error)
}

// sharedTaskRepositoryImpl implements SharedTaskRepository using Provider.
type sharedTaskRepositoryImpl struct {
	provider Provider
}

// NewSharedTaskRepository creates a new SharedTaskRepository.
func NewSharedTaskRepository(provider Provider) SharedTaskRepository {
	return &sharedTaskRepositoryImpl{
		provider: provider,
	}
}

// AcquireTask delegates to the underlying db Provider.
func (r *sharedTaskRepositoryImpl) AcquireTask(ctx context.Context, organizationID, agentID string) (*TaskRecord, error) {
	return r.provider.AcquireTask(ctx, organizationID, agentID)
}
