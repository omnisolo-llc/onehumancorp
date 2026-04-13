package mesh

import (
	"log"
	"net/http"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true // Allow all origins for the mesh API
	},
}

func HandleWS(pubsub MeshPubSub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// Authenticate via SPIFFE context
		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return
		}

		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			log.Printf("Failed to upgrade connection: %v", err)
			return
		}
		defer conn.Close()

		topic := r.URL.Query().Get("topic")
		if topic == "" {
			topic = "global"
		}

		// Subscribe to mesh intents
		ch, err := pubsub.Subscribe(r.Context(), topic)
		if err != nil {
			log.Printf("Failed to subscribe: %v", err)
			return
		}

		done := make(chan struct{})

		// Read and discard incoming messages to keep connection alive
		go func() {
			defer close(done)
			for {
				if _, _, err := conn.ReadMessage(); err != nil {
					break
				}
			}
		}()

		// Write incoming mesh messages to WebSocket
		for {
			select {
			case msg, ok := <-ch:
				if !ok {
					return
				}
				if err := conn.WriteMessage(websocket.TextMessage, msg); err != nil {
					log.Printf("Failed to write message: %v", err)
					return
				}
			case <-done:
				return
			case <-r.Context().Done():
				return
			}
		}
	}
}
