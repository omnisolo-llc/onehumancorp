package mesh

import (
	"context"
	"log"
	"net/http"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
	CheckOrigin: func(r *http.Request) bool {
		return true // Allowing all origins for simplicity in this example
	},
}

type WSHandler struct {
	pubsub MeshPubSub
}

func NewWSHandler(pubsub MeshPubSub) *WSHandler {
	return &WSHandler{pubsub: pubsub}
}

func (h *WSHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
    // SPIFFE/SPIRE authentication should happen here or in a middleware
    // We assume authentication is handled and valid here for the implementation

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Println("Upgrade error:", err)
		return
	}
	defer conn.Close()

	topic := r.URL.Query().Get("topic")
	if topic == "" {
		topic = "default"
	}

	ctx, cancel := context.WithCancel(r.Context())
	defer cancel()

	ch, err := h.pubsub.Subscribe(ctx, topic)
	if err != nil {
		log.Println("Subscribe error:", err)
		return
	}

	// Read loop to keep connection alive and handle client disconnects
	go func() {
		defer cancel()
		for {
			if _, _, err := conn.ReadMessage(); err != nil {
				break
			}
		}
	}()

	// Write loop to send messages to client
	for {
		select {
		case <-ctx.Done():
			return
		case msg, ok := <-ch:
			if !ok {
				return
			}
			if err := conn.WriteMessage(websocket.TextMessage, msg); err != nil {
				log.Println("Write error:", err)
				return
			}
		}
	}
}
