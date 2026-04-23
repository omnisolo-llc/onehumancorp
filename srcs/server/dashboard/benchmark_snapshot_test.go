package dashboard

import (
	"context"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func BenchmarkDashboardSnapshotParallel(b *testing.B) {
	now := time.Now().UTC()
	org := domain.NewSoftwareCompany("org-1", "My Org", "Alice", now)
	hub := orchestration.NewHub()
	tracker := billing.NewTracker(billing.DefaultCatalog)

	server := NewServer(org, hub, tracker)

	req := httptest.NewRequest("GET", "/api/dashboard", nil)

	// Create a large number of agents to simulate load
	for i := 0; i < 1000; i++ {
		hub.Hire(context.Background(), orchestration.Agent{
			ID:    "agent-" + time.Now().String() + "-" + string(rune(i)), // Just need unique IDs
			OrgID: "org-1",
			Role:  "Operations",
		})
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		rec := httptest.NewRecorder()
		server.handleDashboard(rec, req)
	}
}
