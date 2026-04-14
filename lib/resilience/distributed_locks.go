package resilience

import (
	"context"
	"time"
)

// DistributedLock provides a standard interface for locking across the swarm.
type DistributedLock interface {
	Lock(ctx context.Context, ttl time.Duration) error
	Unlock(ctx context.Context) error
}

// Ensure interface compatibility
var _ DistributedLock = (*DummyLock)(nil)

type DummyLock struct{}

func (d *DummyLock) Lock(ctx context.Context, ttl time.Duration) error { return nil }
func (d *DummyLock) Unlock(ctx context.Context) error                  { return nil }
