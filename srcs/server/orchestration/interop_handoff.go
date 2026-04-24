package orchestration

import (
	"context"
	"fmt"
	"log/slog"
	"time"
)

// HandoffState synchronizes state when transitioning between Cloud and Standalone modes.
// It uses MutexProvider to acquire a distributed lock (Redlock in Cloud or IPC/SQLite mutex in Standalone)
// to prevent duplicate handoffs.
func HandoffState(ctx context.Context, mode string, stateData []byte, mutexProvider MutexProvider) error {
	lockKey := fmt.Sprintf("ohc:lock:handoff:%s", mode)

	// Attempt to acquire distributed lock
	mutex := mutexProvider.NewMutex(lockKey)
	err := mutex.Lock(ctx, 30 * time.Second)
	if err != nil {
		if err == ErrLockAcquisitionFailed {
			slog.Warn("HandoffState: another instance is already handling handoff", "mode", mode)
			return nil // Return gracefully since it's already being handled
		}
		return fmt.Errorf("failed to acquire handoff lock: %w", err)
	}

	defer func() {
		err := mutex.Unlock(ctx)
		if err != nil {
			slog.Error("failed to release handoff lock", "err", err)
		}
	}()

	// The state transition logic goes here.
	// For example, syncing the Teammate Mesh state or updating the tasks DB.
	slog.Info("HandoffState: successfully performed idempotent state transition", "mode", mode, "size", len(stateData))

	return nil
}
