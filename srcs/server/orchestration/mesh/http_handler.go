package mesh

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"io"
)

type MeshBroker interface {
	Broadcast(ctx context.Context, channel string, payload []byte) error
}

type HTTPHandler struct {
	broker MeshBroker
}

func NewHTTPHandler(broker MeshBroker) *HTTPHandler {
	return &HTTPHandler{
		broker: broker,
	}
}

func (h *HTTPHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	body, err := io.ReadAll(io.LimitReader(r.Body, 1024*1024))
	if err != nil {
		http.Error(w, "failed to read body", http.StatusInternalServerError)
		return
	}

	var req struct {
		Channel   string                 `json:"channel"`
		EventType string                 `json:"event_type"`
		Data      map[string]interface{} `json:"data"`
	}

	if err := json.Unmarshal(body, &req); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	if req.Channel == "" {
		http.Error(w, "invalid channel", http.StatusBadRequest)
		return
	}

	payloadBytes, err := json.Marshal(req.Data)
	if err != nil {
		http.Error(w, "failed to marshal payload", http.StatusInternalServerError)
		return
	}

	if err := h.broker.Broadcast(r.Context(), req.Channel, payloadBytes); err != nil {
		http.Error(w, fmt.Sprintf("failed to broadcast: %v", err), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok"}`))
}
