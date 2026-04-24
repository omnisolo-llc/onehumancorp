package dashboard

import (
	"testing"

	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/billing"
)

func BenchmarkSnapshot(b *testing.B) {
	srv := &Server{
		org: domain.Organization{ID: "org-1"},
		tracker: billing.NewTracker(billing.DefaultCatalog),
	}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		srv.snapshotLocked()
	}
}
