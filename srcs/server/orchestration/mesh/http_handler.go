package mesh

import (
	"encoding/json"
	"io"
	"net/http"
)

type BroadcastRequest struct {
	Channel   string                 `json:"channel"`
	EventType string                 `json:"event_type"`
	Data      map[string]interface{} `json:"data"`
}

func HandleBroadcast(broker MeshBroker) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		body, err := io.ReadAll(io.LimitReader(r.Body, 1024*1024))
		if err != nil {
			http.Error(w, "failed to read body", http.StatusInternalServerError)
			return
		}

		var req BroadcastRequest
		if err := json.Unmarshal(body, &req); err != nil {
			http.Error(w, "invalid request", http.StatusBadRequest)
			return
		}

		if req.Channel == "" {
			http.Error(w, "invalid channel", http.StatusBadRequest)
			return
		}

		payloadBytes, err := json.Marshal(req)
		if err != nil {
			http.Error(w, "failed to marshal payload", http.StatusInternalServerError)
			return
		}

		if err := broker.Broadcast(r.Context(), req.Channel, payloadBytes); err != nil {
			http.Error(w, "failed to broadcast", http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)
		_, _ = w.Write([]byte(`{"status":"ok"}`))
	}
}
