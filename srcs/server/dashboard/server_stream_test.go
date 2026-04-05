package dashboard

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleStream(t *testing.T) {
	org := domain.Organization{ID: "test-org"}
	hub := &orchestration.Hub{}

	server := &Server{
		org: org,
		hub: hub,
	}

	req, err := http.NewRequest(http.MethodGet, "/api/v1/stream", nil)
	if err != nil {
		t.Fatal(err)
	}

	// Add context with organization
	ctx := req.Context()
	ctx = auth.ContextWithClaims(ctx, &auth.Claims{OrganizationID: "test-org"})
	req = req.WithContext(ctx)

	// Since httptest.ResponseRecorder does not implement http.Flusher in older Go versions natively the same way,
	// we will run it but cancel the context quickly to avoid blocking
	ctx, cancel := context.WithCancel(req.Context())
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()

	go func() {
		time.Sleep(10 * time.Millisecond)
		cancel()
	}()

	server.handleStream(rr, req)

	// The recorder should have at least the connected event
	if rr.Code != http.StatusOK {
		t.Errorf("expected status OK, got %v", rr.Code)
	}

	if rr.Header().Get("Content-Type") != "text/event-stream" {
		t.Errorf("expected Content-Type text/event-stream, got %v", rr.Header().Get("Content-Type"))
	}

	body := rr.Body.String()
	if body == "" {
		t.Errorf("expected non-empty body")
	}
}
