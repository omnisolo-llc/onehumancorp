package dashboard

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

func BenchmarkHandleOnboardingMetrics(b *testing.B) {
	req, _ := http.NewRequest("GET", "/api/dashboard/onboarding/metrics", nil)
	handler := http.HandlerFunc(HandleOnboardingMetrics)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		rr := httptest.NewRecorder()
		handler.ServeHTTP(rr, req)
	}
}

func BenchmarkHandleAutoDreamSync(b *testing.B) {
	req, _ := http.NewRequest("POST", "/api/v1/autodream/sync", nil)
	handler := http.HandlerFunc(HandleAutoDreamSync)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		rr := httptest.NewRecorder()
		handler.ServeHTTP(rr, req)
	}
}

func BenchmarkHandleAutoDreamQuery(b *testing.B) {
	req, _ := http.NewRequest("GET", "/api/v1/autodream/query", nil)
	handler := http.HandlerFunc(HandleAutoDreamQuery)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		rr := httptest.NewRecorder()
		handler.ServeHTTP(rr, req)
	}
}
