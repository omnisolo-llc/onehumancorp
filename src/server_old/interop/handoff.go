package interop

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
	"time"
)

// HandoffManager coordinates mission state synchronization across Cloud and Standalone modes.
type HandoffManager struct {
	mesh      TeammateMesh
	lock      DistributedLock
	imported  map[string]bool
	mu        sync.RWMutex
}

// NewHandoffManager initializes a new HandoffManager using the shared interop mesh and lock.
func NewHandoffManager(mesh TeammateMesh, lock DistributedLock) *HandoffManager {
	return &HandoffManager{
		mesh:     mesh,
		lock:     lock,
		imported: make(map[string]bool),
	}
}

// ExportState securely publishes the agent's current State into the mesh for cross-mode consumption.
func (h *HandoffManager) ExportState(ctx context.Context, tenantID string, state *State) error {
	lockKey := fmt.Sprintf("ohc:lock:handoff:%s", tenantID)

	acquired, err := h.lock.Lock(ctx, lockKey, 10*time.Second)
	if err != nil {
		return fmt.Errorf("failed to acquire handoff lock: %w", err)
	}
	if !acquired {
		return fmt.Errorf("handoff already in progress for tenant %s", tenantID)
	}
	defer h.lock.Unlock(ctx, lockKey)

	payload, err := json.Marshal(state)
	if err != nil {
		return fmt.Errorf("failed to serialize state: %w", err)
	}

	channel := fmt.Sprintf("mesh:handoff:%s", tenantID)
	return h.mesh.Publish(ctx, channel, payload)
}

// ImportState subscribes to the mesh and applies idempotent processing of incoming State payloads.
func (h *HandoffManager) ImportState(ctx context.Context, tenantID string, handler func(*State) error) error {
	channel := fmt.Sprintf("mesh:handoff:%s", tenantID)
	ch, err := h.mesh.Subscribe(ctx, channel)
	if err != nil {
		return fmt.Errorf("failed to subscribe to handoff channel: %w", err)
	}

	go func() {
		for {
			select {
			case <-ctx.Done():
				return
			case payload, ok := <-ch:
				if !ok {
					return
				}

				var state State
				if err := json.Unmarshal(payload, &state); err != nil {
					continue // Drop malformed state
				}

				h.mu.Lock()
				if h.imported[state.ID] {
					h.mu.Unlock()
					continue // Idempotency check: already imported
				}
				h.imported[state.ID] = true
				h.mu.Unlock()

				_ = handler(&state)
			}
		}
	}()

	return nil
}
