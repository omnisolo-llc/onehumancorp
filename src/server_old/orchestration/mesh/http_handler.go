package mesh

import (
    "encoding/json"
    "net/http"
)

type HTTPHandler struct {
    Broker MeshBroker
}

func NewHTTPHandler(broker MeshBroker) *HTTPHandler {
    return &HTTPHandler{Broker: broker}
}

func (h *HTTPHandler) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
    if r.Method != http.MethodPost {
        http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
        return
    }

    r.Body = http.MaxBytesReader(w, r.Body, 1024*1024)

    var payload struct {
        Channel   string          `json:"channel"`
        EventType string          `json:"event_type"`
        Data      json.RawMessage `json:"data"`
    }

    if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
        http.Error(w, "invalid request", http.StatusBadRequest)
        return
    }

    if payload.Channel == "" {
        http.Error(w, "invalid channel", http.StatusBadRequest)
        return
    }

    rawMsg, err := json.Marshal(payload)
    if err != nil {
        http.Error(w, "internal server error", http.StatusInternalServerError)
        return
    }

    if err := h.Broker.Broadcast(r.Context(), payload.Channel, rawMsg); err != nil {
        http.Error(w, "failed to broadcast", http.StatusInternalServerError)
        return
    }

    w.WriteHeader(http.StatusOK)
    _, _ = w.Write([]byte(`{"status":"ok"}`))
}
