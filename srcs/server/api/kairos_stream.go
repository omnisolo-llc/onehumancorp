package api

import (
	"log/slog"
	"net/http"
	"strings"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"go.opentelemetry.io/otel"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		origin := r.Header.Get("Origin")
		if origin == "" {
			return true // Allow non-browser clients (e.g. mobile apps, tests)
		}
		// Allow typical local development origins and explicitly configured ones.
		// Note: A more robust production check might validate against a list of allowed origins.
		return strings.HasPrefix(origin, "http://localhost") || strings.HasPrefix(origin, "https://localhost")
	},
}

// KairosStreamHandler provides a WebSocket endpoint to stream Teammate Mesh events.
func KairosStreamHandler(hub *orchestration.Hub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/api").Start(r.Context(), "KairosStreamHandler")
		defer span.End()

		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			slog.Error("failed to upgrade to websocket", "error", err)
			return
		}
		defer conn.Close()

		if hub == nil {
			slog.Error("hub is nil in KairosStreamHandler")
			conn.WriteMessage(websocket.TextMessage, []byte(`{"error":"hub not configured"}`))
			return
		}

		mesh := hub.CentrifugeNode()
		if mesh == nil {
			slog.Warn("centrifuge node is nil in KairosStreamHandler")
			conn.WriteMessage(websocket.TextMessage, []byte(`{"error":"mesh not configured"}`))
			return
		}

		// Subscribe to teammate mesh events for streaming
		subChan, unsubscribe := hub.Subscribe("system")
		defer unsubscribe()

		// Stream messages to client
		go func() {
			for {
				select {
				case <-ctx.Done():
					return
				case _, ok := <-subChan:
					if !ok {
						return
					}
					messages := hub.Inbox("system")
					for _, msg := range messages {
						eventStr := `{"event":"TaskBroadcast","status":"INFO"}`
						if msg.Type == "mesh:tasks" {
							eventStr = msg.Content
						}
						if err := conn.WriteMessage(websocket.TextMessage, []byte(eventStr)); err != nil {
							slog.Error("failed to write websocket message", "error", err)
							return
						}
					}
				}
			}
		}()

		// Wait for connection to close
		for {
			_, _, err := conn.ReadMessage()
			if err != nil {
				break
			}
		}
	}
}
