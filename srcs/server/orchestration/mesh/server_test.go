package mesh

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestMeshGateway_BroadcastHandler(t *testing.T) {
	broadcaster := NewChannelBroadcaster()
	gateway := &MeshGateway{broadcaster: broadcaster}

	event := MeshEvent{
		EventID:   "evt-123",
		AgentID:   "spiffe://example.org/agent/1",
		Action:    "TaskTransition",
		Status:    "running",
		Timestamp: "2026-04-14T15:00:00Z",
	}
	body, _ := json.Marshal(event)

	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewReader(body))
	// Simulate SPIFFEMiddleware
	req = req.WithContext(context.WithValue(req.Context(), spiffeContextKey, "spiffe://example.org/agent/1"))

	w := httptest.NewRecorder()
	gateway.BroadcastHandler(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("Expected status 200, got %d", w.Code)
	}

	select {
	case received := <-broadcaster.ch:
		if received.EventID != event.EventID {
			t.Errorf("Expected event ID %s, got %s", event.EventID, received.EventID)
		}
	default:
		t.Error("Event not broadcasted")
	}
}
