package mesh

import (
	"context"
	"log"
	"net/http"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true // Allow all origins for the mesh, or restrict based on config
	},
}

type WSHandler struct {
	pubsub MeshPubSub
}

func NewWSHandler(pubsub MeshPubSub) *WSHandler {
	return &WSHandler{pubsub: pubsub}
}

func (h *WSHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	ctx, cancel := context.WithCancel(r.Context())
	defer cancel()

	spiffeID, err := orchestration.ExtractSPIFFEID(ctx)
	if err != nil {
		http.Error(w, "Unauthorized: missing or invalid SPIFFE ID", http.StatusUnauthorized)
		return
	}

	topic := r.URL.Query().Get("topic")
	if topic == "" {
		http.Error(w, "Missing 'topic' query parameter", http.StatusBadRequest)
		return
	}

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Printf("Failed to upgrade to WebSocket for %s: %v", spiffeID, err)
		return
	}
	defer conn.Close()

	ch, unsubscribe, err := h.pubsub.Subscribe(ctx, topic)
	if err != nil {
		log.Printf("Failed to subscribe to topic %s: %v", topic, err)
		return
	}
	defer unsubscribe()

	// Handle disconnects and read pump
	go func() {
		defer cancel() // cancel the context when read pump exits, signaling write pump to exit
		for {
			_, _, err := conn.ReadMessage()
			if err != nil {
				// Client disconnected
				break
			}
		}
	}()

	// Write pump
	for {
		select {
		case msg, ok := <-ch:
			if !ok {
				// Channel closed
				conn.WriteMessage(websocket.CloseMessage, []byte{})
				return
			}
			err := conn.WriteMessage(websocket.TextMessage, msg)
			if err != nil {
				log.Printf("Failed to write message to %s: %v", spiffeID, err)
				return
			}
		case <-ctx.Done():
			conn.WriteMessage(websocket.CloseMessage, []byte{})
			return
		}
	}
}
