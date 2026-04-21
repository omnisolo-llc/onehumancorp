package mesh

import (
	"context"
	"encoding/json"
	"net/http"
)

type PublishHandler struct {
	pubsub MeshPubSub
}

func NewPublishHandler(pubsub MeshPubSub) *PublishHandler {
	return &PublishHandler{pubsub: pubsub}
}

type PublishRequest struct {
	Topic   string          `json:"topic"`
	Message json.RawMessage `json:"message"`
}

func (h *PublishHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req PublishRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if req.Topic == "" {
		http.Error(w, "Topic is required", http.StatusBadRequest)
		return
	}

	if err := h.pubsub.Publish(context.Background(), req.Topic, req.Message); err != nil {
		http.Error(w, "Failed to publish", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"success"}`))
}
