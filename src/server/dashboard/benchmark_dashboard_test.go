package dashboard

import (
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/billing"
	"github.com/onehumancorp/mono/src/server/domain"
	"github.com/onehumancorp/mono/src/server/orchestration"
)

func BenchmarkSnapshotLocked(b *testing.B) {
	org := domain.Organization{ID: "org-1"}
	hub := orchestration.NewHub()
	tracker := billing.NewTracker(nil)

	// Create some dummy agents, meetings, costs, and tasks
	for i := 0; i < 50; i++ {
		hub.RegisterAgent(orchestration.Agent{
			ID:             time.Now().String() + "agent",
			OrganizationID: "org-1",
		})
	}

	server := &Server{
		org:     org,
		hub:     hub,
		tracker: tracker,
	}

	server.mu.Lock()
	defer server.mu.Unlock()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = server.snapshotLocked()
	}
}
