package mesh

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type PublishRequest struct {
	Topic   string          `json:"topic"`
	Message json.RawMessage `json:"message"`
}

func HandlePublish(pubsub MeshPubSub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		// Authenticate via SPIFFE context
		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return
		}

		var req PublishRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "bad request", http.StatusBadRequest)
			return
		}

		topic := req.Topic
		if topic == "" {
			topic = "global"
		}

		if err := pubsub.Publish(r.Context(), topic, req.Message); err != nil {
			http.Error(w, "failed to broadcast", http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)
	}
}
