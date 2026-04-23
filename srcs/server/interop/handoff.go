package interop

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/proto/interop"
)

// StateHandoffManager ensures idempotent state synchronization when switching
// between Cloud and Standalone modes.
type StateHandoffManager interface {
	SyncState(ctx context.Context, data *interoppb.StateHandoffData) error
	GetState(ctx context.Context, tenantID string) (*interoppb.StateHandoffData, error)
}

type inMemoryHandoffManager struct {
	mu    sync.RWMutex
	store map[string]*interoppb.StateHandoffData
}

// NewStateHandoffManager creates a new StateHandoffManager.
// In a full implementation, this could interface with a database or vector store.
func NewStateHandoffManager() StateHandoffManager {
	return &inMemoryHandoffManager{
		store: make(map[string]*interoppb.StateHandoffData),
	}
}

func (m *inMemoryHandoffManager) SyncState(ctx context.Context, data *interoppb.StateHandoffData) error {
	if data == nil || data.TenantId == "" {
		return fmt.Errorf("invalid handoff data: missing tenant_id")
	}

	m.mu.Lock()
	defer m.mu.Unlock()

	existing, ok := m.store[data.TenantId]
	if ok {
		// Idempotency check: only update if the incoming state is strictly newer
		if data.LastSynced <= existing.LastSynced {
			// Already have this state or a newer one, ignore to prevent replay overwrites
			return nil
		}
	}

	// Update timestamp if it wasn't set, though the caller should set it
	if data.LastSynced == 0 {
		data.LastSynced = time.Now().Unix()
	}

	m.store[data.TenantId] = data
	return nil
}

func (m *inMemoryHandoffManager) GetState(ctx context.Context, tenantID string) (*interoppb.StateHandoffData, error) {
	if tenantID == "" {
		return nil, fmt.Errorf("tenant_id required")
	}

	m.mu.RLock()
	defer m.mu.RUnlock()

	state, ok := m.store[tenantID]
	if !ok {
		return nil, fmt.Errorf("state not found for tenant: %s", tenantID)
	}

	return state, nil
}
