package orchestration

import (
	"encoding/json"
	"net/http"
)

// MeshBroadcastRequest represents the payload for broadcasting a message to the Teammate Mesh.
type MeshBroadcastRequest struct {
	AgentID string          `json:"agent_id"`
	Action  string          `json:"action"`
	Status  string          `json:"status"`
	Channel string          `json:"channel"`
	Payload json.RawMessage `json:"payload,omitempty"`
}

// HandleMeshBroadcast handles the POST /api/mesh/broadcast endpoint.
func (s *HubServiceServer) HandleMeshBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method Not Allowed", http.StatusMethodNotAllowed)
		return
	}

	var req MeshBroadcastRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Bad Request: invalid JSON", http.StatusBadRequest)
		return
	}

	if req.AgentID == "" || req.Action == "" || req.Status == "" || req.Channel == "" {
		http.Error(w, "Bad Request: missing required fields", http.StatusBadRequest)
		return
	}

	channel := req.Channel
	// Ensure we only broadcast to mesh channels
	if channel != "mesh:tasks" && channel != "mesh:coordination" {
		http.Error(w, "Bad Request: invalid channel", http.StatusBadRequest)
		return
	}

	// Prepare payload with root level fields
	msgMap := map[string]interface{}{
		"agent_id": req.AgentID,
		"action":   req.Action,
		"status":   req.Status,
	}

	if len(req.Payload) > 0 {
		msgMap["payload"] = req.Payload
	}

	if s.hub != nil && s.hub.centrifugeNode != nil {
		s.hub.centrifugeNode.PublishTaskBroadcast(channel, msgMap)
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"success":true}`))
}
