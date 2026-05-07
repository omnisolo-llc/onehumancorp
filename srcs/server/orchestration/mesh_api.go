package orchestration

import (
	"encoding/json"
	"net/http"
)

type MeshAPI struct {
	hub MeshHub
}

func NewMeshAPI(hub MeshHub) *MeshAPI {
	return &MeshAPI{
		hub: hub,
	}
}

type BroadcastRequest struct {
	Channel string          `json:"channel"`
	AgentID string          `json:"agent_id,omitempty"`
	Action  string          `json:"action,omitempty"`
	Status  string          `json:"status,omitempty"`
	Payload json.RawMessage `json:"payload,omitempty"`
}

type Message struct {
	Content string `json:"Content"`
}

func (api *MeshAPI) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req BroadcastRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if req.Channel == "" {
		http.Error(w, "Channel is required", http.StatusBadRequest)
		return
	}

	var dataBytes []byte
	if req.Channel == "mesh:tasks" || req.Channel == "mesh:coordination" {
		reqBytes, _ := json.Marshal(req)
		msg := Message{Content: string(reqBytes)}
		dataBytes, _ = json.Marshal(msg)
	} else {
		dataBytes, _ = json.Marshal(req)
	}

	err := api.hub.Publish(r.Context(), req.Channel, dataBytes)
	if err != nil {
		http.Error(w, "Failed to publish message", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
