package mesh

import (
	"encoding/json"
	"io"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type PublishRequest struct {
	Topic   string          `json:"topic"`
	Message json.RawMessage `json:"message"`
}

type PublishHandler struct {
	pubsub MeshPubSub
}

func NewPublishHandler(pubsub MeshPubSub) *PublishHandler {
	return &PublishHandler{pubsub: pubsub}
}

func (h *PublishHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	ctx := r.Context()

	// Optional but recommended: authenticate the publisher
	_, err := orchestration.ExtractSPIFFEID(ctx)
	if err != nil {
		http.Error(w, "Unauthorized: missing or invalid SPIFFE ID", http.StatusUnauthorized)
		return
	}

	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Failed to read request body", http.StatusBadRequest)
		return
	}
	defer r.Body.Close()

	var req PublishRequest
	if err := json.Unmarshal(body, &req); err != nil {
		http.Error(w, "Invalid JSON payload", http.StatusBadRequest)
		return
	}

	if req.Topic == "" {
		http.Error(w, "Missing 'topic' in request", http.StatusBadRequest)
		return
	}

	if err := h.pubsub.Publish(ctx, req.Topic, req.Message); err != nil {
		http.Error(w, "Failed to publish message", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"published"}`))
}
