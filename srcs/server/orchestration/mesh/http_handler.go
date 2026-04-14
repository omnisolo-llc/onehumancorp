package mesh

import (
	"encoding/json"
	"io"
	"net/http"
)

// BroadcastRequest represents the JSON payload expected by the broadcast endpoint.
type BroadcastRequest struct {
	Channel   string          `json:"channel"`
	EventType string          `json:"event_type"`
	Data      json.RawMessage `json:"data"`
}

// BroadcastHandler provides an HTTP handler for broadcasting events.
type BroadcastHandler struct {
	broker MeshBroker
}

// NewBroadcastHandler creates a new BroadcastHandler.
func NewBroadcastHandler(broker MeshBroker) *BroadcastHandler {
	return &BroadcastHandler{
		broker: broker,
	}
}

// ServeHTTP handles the POST /api/mesh/v2/broadcast requests.
func (h *BroadcastHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method Not Allowed", http.StatusMethodNotAllowed)
		return
	}

	// Limit payload sizes to 1MB to prevent memory exhaustion
	r.Body = http.MaxBytesReader(w, r.Body, 1<<20) // 1MB limit

	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Payload Too Large", http.StatusRequestEntityTooLarge)
		return
	}
	defer r.Body.Close()

	var req BroadcastRequest
	if err := json.Unmarshal(body, &req); err != nil {
		http.Error(w, "Bad Request", http.StatusBadRequest)
		return
	}

	if req.Channel == "" || req.EventType == "" {
		http.Error(w, "Bad Request: missing channel or event_type", http.StatusBadRequest)
		return
	}

	err = h.broker.Broadcast(r.Context(), req.Channel, req.Data)
	if err != nil {
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"ok"}`))
}
