package sync

import (
	"context"
)

// SyncService defines the interface for synchronizing deltas.
type SyncService interface {
	SyncDeltas(ctx context.Context, deltas []SyncDelta) error
}
