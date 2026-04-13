package dashboard

import (
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/gorilla/websocket"
)

func TestHandleKairosStream(t *testing.T) {
	hub := orchestration.NewHub()

	server := &Server{
		org:       domain.Organization{ID: "test-org"},
		hub:       hub,
	}

	mux := http.NewServeMux()
	mux.HandleFunc("/api/kairos/stream", server.handleKairosStream)

	ts := httptest.NewServer(mux)
	defer ts.Close()

	// In the real code auth middleware checks this, but our endpoint skips it
	// because we mocked out the ValidateToken checks for the test context
	wsURL := "ws" + strings.TrimPrefix(ts.URL, "http") + "/api/kairos/stream"
	ws, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("Failed to connect to websocket: %v", err)
	}
	defer ws.Close()

	// Read connection message
	_, msg, err := ws.ReadMessage()
	if err != nil {
		t.Fatalf("Failed to receive connection message: %v", err)
	}

	// Send message to hub
	go func() {
		time.Sleep(100 * time.Millisecond)
		hub.Publish(orchestration.Message{
			Content: `{"status": "mesh broadcast"}`,
			Type:    "mesh:coordination",
		})
	}()

	// Wait for the broadcast message (might get a heartbeat first)
	for i := 0; i < 2; i++ {
		_, msg, err = ws.ReadMessage()
		if err != nil {
			t.Fatalf("Failed to receive broadcast message: %v", err)
		}
		if string(msg) == `{"status": "mesh broadcast"}` {
			break
		}
	}
}
