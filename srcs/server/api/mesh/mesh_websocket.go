package mesh

import (
	"log/slog"
	"net/http"
	"time"

	"github.com/gorilla/websocket"
)

const (
	writeWait      = 10 * time.Second
	pongWait       = 60 * time.Second
	pingPeriod     = (pongWait * 9) / 10
	maxMessageSize = 512
)

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
	// Using default CheckOrigin which enforces same-origin policy, preventing CSWSH
}

type WebSocketMeshHandler struct {
	meshService TeammateMeshService
}

func NewWebSocketMeshHandler(meshService TeammateMeshService) *WebSocketMeshHandler {
	return &WebSocketMeshHandler{meshService: meshService}
}

func (h *WebSocketMeshHandler) HandleWebSocket(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		slog.Error("failed to upgrade to websocket", "error", err)
		return
	}
	defer conn.Close()

	ctx := r.Context()
	subChan, err := h.meshService.Subscribe(ctx)
	if err != nil {
		slog.Error("failed to subscribe to mesh", "error", err)
		conn.WriteMessage(websocket.TextMessage, []byte(`{"error": "subscription failed"}`))
		return
	}

	conn.SetReadLimit(maxMessageSize)
	conn.SetReadDeadline(time.Now().Add(pongWait))
	conn.SetPongHandler(func(string) error { conn.SetReadDeadline(time.Now().Add(pongWait)); return nil })

	// Goroutine to read from mesh and write to websocket
	go func() {
		ticker := time.NewTicker(pingPeriod)
		defer func() {
			ticker.Stop()
			conn.Close()
		}()

		for {
			select {
			case msg, ok := <-subChan:
				conn.SetWriteDeadline(time.Now().Add(writeWait))
				if !ok {
					conn.WriteMessage(websocket.CloseMessage, []byte{})
					return
				}
				if err := conn.WriteMessage(websocket.TextMessage, []byte(msg)); err != nil {
					slog.Error("failed to write to websocket", "error", err)
					return
				}
			case <-ticker.C:
				conn.SetWriteDeadline(time.Now().Add(writeWait))
				if err := conn.WriteMessage(websocket.PingMessage, nil); err != nil {
					return
				}
			case <-ctx.Done():
				return
			}
		}
	}()

	// Read from websocket and broadcast to mesh
	for {
		_, message, err := conn.ReadMessage()
		if err != nil {
			if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
				slog.Error("unexpected close error", "error", err)
			}
			break
		}
		if err := h.meshService.BroadcastIntent(ctx, string(message)); err != nil {
			slog.Error("failed to broadcast to mesh", "error", err)
		}
	}
}
