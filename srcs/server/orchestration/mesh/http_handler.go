package mesh

import (
	"encoding/json"
	"io"
	"net/http"
)

type HTTPHandler struct {
	broker MeshBroker
}

func NewHTTPHandler(broker MeshBroker) *HTTPHandler {
	return &HTTPHandler{
		broker: broker,
	}
}

type broadcastRequest struct {
	Channel   string          `json:"channel"`
	EventType string          `json:"event_type"`
	Data      json.RawMessage `json:"data"`
}

func (h *HTTPHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	r.Body = http.MaxBytesReader(w, r.Body, 1<<20) // 1MB limit

	var req broadcastRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		if err == io.EOF {
			http.Error(w, "empty request body", http.StatusBadRequest)
		} else {
			http.Error(w, "invalid request body", http.StatusBadRequest)
		}
		return
	}

	if req.Channel == "" {
		http.Error(w, "channel is required", http.StatusBadRequest)
		return
	}

	payload, err := json.Marshal(map[string]interface{}{
		"event_type": req.EventType,
		"data":       req.Data,
	})
	if err != nil {
		http.Error(w, "failed to marshal payload", http.StatusInternalServerError)
		return
	}

	if err := h.broker.Broadcast(r.Context(), req.Channel, payload); err != nil {
		http.Error(w, "failed to broadcast", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
