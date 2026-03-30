package dashboard

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/billing"
	"github.com/onehumancorp/mono/srcs/domain"
	"github.com/onehumancorp/mono/srcs/orchestration"
)

func BenchmarkHandleScaleStream(b *testing.B) {
	org := domain.Organization{ID: "test-org"}
	hub := orchestration.NewHub()
	tracker := billing.NewTracker(map[string]billing.Price{})
	serverHandler := NewServer(org, hub, tracker)

	// The NewServer method returns an http.Handler.
	// Since we know the implementation is a chi.Mux or chi.Router wrapping Server,
	// or similar. Wait, NewServer returns telemetry.Middleware(auth.Middleware(store)(mux))
	// We should just use the handler directly.
	req := httptest.NewRequest(http.MethodGet, "/api/v1/scale/stream", nil)

	b.ResetTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		rr := httptest.NewRecorder()
		serverHandler.ServeHTTP(rr, req)
	}
}
