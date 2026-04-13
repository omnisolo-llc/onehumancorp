package dashboard

import (
	"fmt"
	"net/http"
	"time"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true // Typically we'd check origin here
	},
}

// handleKairosStream provides a WebSocket stream
// for KAIROS Swarm Analytics to visualize Teammate Mesh and Tasks.
// Authentication is handled by the `auth.RequireRole("system", ...)` middleware in server.go,
// which validates the context before invoking this handler.
func (s *Server) handleKairosStream(w http.ResponseWriter, r *http.Request) {
	ws, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		fmt.Printf("WebSocket upgrade failed: %v\n", err)
		return
	}
	defer ws.Close()

	ctx := r.Context()
	ticker := time.NewTicker(2 * time.Second)
	defer ticker.Stop()

	var subChan <-chan struct{}
	var unsubscribe func()

	if s.hub != nil {
		subChan, unsubscribe = s.hub.Subscribe("system")
		defer unsubscribe()
	}

	// Send initial connection success message
	ws.WriteMessage(websocket.TextMessage, []byte(`{"event": "connected", "status": "ok"}`))

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if err := ws.WriteMessage(websocket.TextMessage, []byte(`{"event": "heartbeat"}`)); err != nil {
				return
			}
		case _, ok := <-subChan:
			if !ok {
				return
			}
			messages := s.hub.Inbox("system")
			for _, msg := range messages {
				// Filter specifically for KAIROS mesh channels
				if msg.Type == "mesh:tasks" || msg.Type == "mesh:coordination" {
					if err := ws.WriteMessage(websocket.TextMessage, []byte(msg.Content)); err != nil {
						return
					}
				}
			}
		}
	}
}
