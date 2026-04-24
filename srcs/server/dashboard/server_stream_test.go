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
	app, _, _, err := newTestServer(t)
	if err != nil {
		t.Fatalf("failed to setup server: %v", err)
	}

	req := httptest.NewRequest("GET", "/api/v1/stream", nil)

	// Simulate client disconnection after a short delay
	ctx, cancel := context.WithTimeout(context.Background(), 100*time.Millisecond)
	defer cancel()

	req = req.WithContext(auth.ContextWithClaims(ctx, &auth.Claims{OrganizationID: "test-org", Roles: []string{"system"}}))
	w := httptest.NewRecorder()

	done := make(chan struct{})
	go func() {
		app.handleStream(w, req)
		close(done)
	}()

	// Simulate pushing an event to the mesh channel
	if app.hub != nil {
		go func() {
			time.Sleep(10 * time.Millisecond)
			if app.hub.TeammateMesh() != nil {
				_ = app.hub.TeammateMesh().Publish(context.Background(), "mesh:tasks", []byte(`{"event":"AgentHired","status":"INFO"}`))
			}

			// Because the test environment doesn't automatically loop TeammateMesh to hub inboxes,
			// let's directly publish to the Hub so it triggers the subscriber check in handleStream.
			app.hub.LogEvent(`{"event":"AgentHired","status":"INFO"}`)
		}()
	}

	select {
	case <-done:
	case <-time.After(3 * time.Second):
		t.Fatal("handleStream did not finish in time")
	}

	body := w.Body.String()

	// Since we mock the Teammate Mesh and publish it asynchronously,
	// we just need to ensure the connection sets SSE headers and processes correctly.
	if w.Header().Get("Content-Type") != "text/event-stream" {
		t.Errorf("Expected Content-Type text/event-stream, got %s", w.Header().Get("Content-Type"))
	}

	if !strings.Contains(body, "AgentHired") {
		t.Errorf("Expected stream to contain AgentHired, got %s", body)
	}
}
