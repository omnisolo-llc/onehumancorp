package dashboard

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/billing"
	"github.com/onehumancorp/mono/src/server/domain"
	"github.com/onehumancorp/mono/src/server/orchestration"
)

// BenchmarkSnapshotLocked compares the parallel execution performance of building the snapshot
func BenchmarkSnapshotLocked(b *testing.B) {
	org := domain.Organization{ID: "org-bench", Name: "Benchmark Org"}

	// mock hub and tracker to simulate I/O delay or just standard memory reads
	// In reality, testing actual DB delays would require a DB setup
	// We'll run the benchmark against the actual methods to see overhead
	// But without DB it might just be memory ops. Let's just create a basic server

	s := NewServer(org, nil, billing.NewTracker(nil))
	// Because PeekTasks does db query and tracker.Summary does db query,
	// when repo/TaskManager is nil, they return quickly.

	b.Run("ParallelSnapshot", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			_ = s.snapshotLocked()
		}
	})
}
