package mesh

import (
	"log/slog"
	"net/http"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
	CheckOrigin: func(r *http.Request) bool {
		origin := r.Header.Get("Origin")
		// Allow specific origins or matching logic. For demo/internal, we might accept specific trusted origins or localhost
		if origin == "" || origin == "http://localhost:8080" || origin == "http://localhost:3000" {
			return true
		}
		return false
	},
}

func HandleWebSocket(meshService TeammateMeshService) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			slog.Error("Failed to upgrade WebSocket connection", "error", err)
			return
		}
		defer conn.Close()

		ctx := r.Context()
		out, err := meshService.Subscribe(ctx)
		if err != nil {
			slog.Error("Failed to subscribe to mesh", "error", err)
			conn.WriteMessage(websocket.CloseMessage, websocket.FormatCloseMessage(websocket.CloseInternalServerErr, "Subscription failed"))
			return
		}

		// Required by gorilla/websocket to process ping/pong/close control messages
		go func() {
			for {
				if _, _, err := conn.NextReader(); err != nil {
					conn.Close()
					break
				}
			}
		}()

		for {
			select {
			case msg, ok := <-out:
				if !ok {
					// channel closed
					conn.WriteMessage(websocket.CloseMessage, websocket.FormatCloseMessage(websocket.CloseNormalClosure, "Subscription channel closed"))
					return
				}
				err = conn.WriteMessage(websocket.TextMessage, []byte(msg))
				if err != nil {
					slog.Error("Failed to write to WebSocket", "error", err)
					return
				}
			case <-ctx.Done():
				conn.WriteMessage(websocket.CloseMessage, websocket.FormatCloseMessage(websocket.CloseNormalClosure, "Context done"))
				return
			}
		}
	}
}
