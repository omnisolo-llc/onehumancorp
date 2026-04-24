package orchestration

import (
	"encoding/json"
	"net/http"
)

type BroadcastPayload struct {
	AgentID   string                 `json:"agent_id"`
	Channel   string                 `json:"channel"`
	EventType string                 `json:"event_type"`
	Data      map[string]interface{} `json:"data"`
}

func HandleMeshBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method Not Allowed", http.StatusMethodNotAllowed)
		return
	}

	var payload BroadcastPayload
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "Bad Request", http.StatusBadRequest)
		return
	}

	if payload.AgentID == "" || payload.Channel == "" || payload.EventType == "" || payload.Data == nil {
		http.Error(w, "Bad Request", http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
}
