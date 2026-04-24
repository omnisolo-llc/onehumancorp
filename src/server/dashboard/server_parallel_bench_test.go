package dashboard

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/src/server/billing"
	"github.com/onehumancorp/mono/src/server/domain"
)

func BenchmarkSnapshotLocked(b *testing.B) {
	org := domain.Organization{ID: "org-bench", Name: "Benchmark Org"}

	s := NewServer(org, nil, billing.NewTracker(nil))

	b.ResetTimer()
	b.Run("ParallelSnapshot", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			_ = s.snapshotLocked()
		}
	})
}
