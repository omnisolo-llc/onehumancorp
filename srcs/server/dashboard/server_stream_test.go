package dashboard

import (
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHandleStream(t *testing.T) {
	s := &Server{}
	req := httptest.NewRequest("GET", "/api/v1/stream", nil)

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	req = req.WithContext(auth.ContextWithClaims(ctx, &auth.Claims{OrganizationID: "test-org"}))
	w := httptest.NewRecorder()

	done := make(chan struct{})
	go func() {
		s.handleStream(w, req)
		close(done)
	}()

	select {
	case <-done:
	case <-time.After(3 * time.Second):
		t.Fatal("handleStream did not finish in time")
	}

	body := w.Body.String()

	if !strings.Contains(body, "AgentHired") {
		t.Errorf("Expected stream to contain AgentHired, got %s", body)
	}
	if !strings.Contains(body, "TaskCompleted") {
		t.Errorf("Expected stream to contain TaskCompleted, got %s", body)
	}
	if !strings.Contains(body, "AgentFired") {
		t.Errorf("Expected stream to contain AgentFired, got %s", body)
	}

	if w.Header().Get("Content-Type") != "text/event-stream" {
		t.Errorf("Expected Content-Type text/event-stream, got %s", w.Header().Get("Content-Type"))
	}
}
