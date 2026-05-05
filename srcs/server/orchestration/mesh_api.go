package orchestration

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"strings"
)

type SIPPayload struct {
	AgentID   string          `json:"agent_id"`
	Channel   string          `json:"channel"`
	EventType string          `json:"event_type"`
	Data      json.RawMessage `json:"data"`
}

type MeshAPI struct {
	Transport Transport
}

type Transport interface {
	Broadcast(channel string, payload []byte) error
}

func (api *MeshAPI) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	bodyBytes, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Failed to read request body", http.StatusBadRequest)
		return
	}
	r.Body.Close()

	r.Body = io.NopCloser(bytes.NewBuffer(bodyBytes))

	var payload SIPPayload
	if err := json.NewDecoder(bytes.NewBuffer(bodyBytes)).Decode(&payload); err != nil {
		http.Error(w, "Invalid JSON", http.StatusBadRequest)
		return
	}

	if payload.AgentID == "" || payload.Channel == "" || payload.EventType == "" || payload.Data == nil {
		http.Error(w, "Missing SIP fields", http.StatusBadRequest)
		return
	}

	if !strings.HasPrefix(payload.AgentID, "spiffe://") {
		http.Error(w, "agent_id must be a valid SPIFFE ID", http.StatusBadRequest)
		return
	}

	if payload.Channel != "mesh:tasks" && payload.Channel != "mesh:coordination" && payload.Channel != "mesh:presence" {
		http.Error(w, "Invalid channel", http.StatusBadRequest)
		return
	}

	if api.Transport != nil {
		err := api.Transport.Broadcast(payload.Channel, bodyBytes)
		if err != nil {
			http.Error(w, "Failed to broadcast", http.StatusInternalServerError)
			return
		}
	}

	w.WriteHeader(http.StatusOK)
}

func (api *MeshAPI) HandlePublish(w http.ResponseWriter, r *http.Request) {
	api.HandleBroadcast(w, r)
}
