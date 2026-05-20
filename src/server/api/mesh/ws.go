package mesh

import (
	"context"
	"log"
	"net/http"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true // Allowing all origins for demo
	},
}

// WSHandler handles WebSocket connections
func WSHandler(pubsub MeshPubSub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// Securely authenticate agents using SPIFFE/SPIRE context
		// In a real environment with mTLS terminating proxy, X-Spiffe-ID would be injected by Envoy.
		// If terminating TLS locally, extract from TLS PeerCertificates.
		var spiffeID string
		if r.TLS != nil && len(r.TLS.PeerCertificates) > 0 {
			// simplified extraction
			spiffeID = r.TLS.PeerCertificates[0].URIs[0].String()
		} else {
			spiffeID = r.Header.Get("X-Spiffe-ID") // Fallback for reverse-proxy injection
		}

		if spiffeID == "" {
			http.Error(w, "Unauthorized: Missing SPIFFE ID", http.StatusUnauthorized)
			return
		}

		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			log.Println("WebSocket upgrade error:", err)
			return
		}
		defer conn.Close()

		topic := r.URL.Query().Get("topic")
		if topic == "" {
			topic = "default"
		}

		ctx, cancel := context.WithCancel(r.Context())
		defer cancel()

		ch, err := pubsub.Subscribe(ctx, topic)
		if err != nil {
			log.Println("Subscribe error:", err)
			return
		}

		// Read loop to handle control messages and connection drops
		go func() {
			defer cancel()
			for {
				if _, _, err := conn.ReadMessage(); err != nil {
					return
				}
			}
		}()

		for {
			select {
			case <-ctx.Done():
				return
			case msg := <-ch:
				if err := conn.WriteJSON(msg); err != nil {
					log.Println("WebSocket write error:", err)
					return
				}
			}
		}
	}
}
