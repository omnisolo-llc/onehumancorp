package dashboard

import (
    "testing"
    "github.com/onehumancorp/mono/src/server/domain"
    "github.com/onehumancorp/mono/src/server/orchestration"
    "github.com/onehumancorp/mono/src/server/billing"
)

func BenchmarkDashboardSnapshot(b *testing.B) {
    org := domain.Organization{ID: "org-1"}
    hub := orchestration.NewHub()
    defer hub.Close()

    tracker := billing.NewTracker(billing.DefaultCatalog)
    app := &Server{org: org, hub: hub, tracker: tracker}

    b.ResetTimer()
    for i := 0; i < b.N; i++ {
        _ = app.snapshot()
    }
}
