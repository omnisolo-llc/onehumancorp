package perf

import (

	"context"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type Coordinator struct {
	db *orchestration.SIPDB
}

func NewCoordinator(db *orchestration.SIPDB) *Coordinator {
	return &Coordinator{
		db: db,
	}
}

func (c *Coordinator) ParallelUpdateMemory(ctx context.Context, updates map[string]string) error {
	// Regardless of SQLite or Postgres, a single BatchUpdateMemory query is significantly faster than
	// parallel UpdateMemory queries because it avoids transaction parsing overhead and pool roundtrips.
	return c.db.BatchUpdateMemory(ctx, updates)
}
