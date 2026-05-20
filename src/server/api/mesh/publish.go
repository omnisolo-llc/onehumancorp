package mesh

import (
	"encoding/json"
	"net/http"
)

// PublishRequest defines the request payload for publishing a message
type PublishRequest struct {
	Topic   string            `json:"topic"`
	Message TeammateMeshEvent `json:"message"`
}

// PublishHandler handles HTTP requests to publish a message
func PublishHandler(pubsub MeshPubSub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		var req PublishRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Invalid request body", http.StatusBadRequest)
			return
		}

		if err := pubsub.Publish(r.Context(), req.Topic, req.Message); err != nil {
			http.Error(w, "Failed to publish message", http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(map[string]bool{"success": true})
	}
}
