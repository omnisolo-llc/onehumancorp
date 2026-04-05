package dashboard

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleStream(t *testing.T) {
	// Need to test SSE handler
	s := &Server{
		hub: orchestration.NewFakeHub(),
	}

	req := httptest.NewRequest(http.MethodGet, "/api/v1/stream", nil)
	// Inject fake claims to bypass auth
	req = req.WithContext(auth.ContextWithClaims(req.Context(), &auth.Claims{
		UserID: "user-1",
		Role:   "user",
	}))

	rr := httptest.NewRecorder()

	ctx, cancel := context.WithTimeout(req.Context(), 50*time.Millisecond)
	defer cancel()
	req = req.WithContext(ctx)

	s.handleStream(rr, req)

	res := rr.Result()
	if res.StatusCode != http.StatusOK {
		t.Errorf("expected status OK, got %v", res.StatusCode)
	}

	if res.Header.Get("Content-Type") != "text/event-stream" {
		t.Errorf("expected Content-Type text/event-stream, got %v", res.Header.Get("Content-Type"))
	}
}
