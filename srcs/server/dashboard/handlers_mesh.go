package dashboard

import (
	"encoding/json"
	"net/http"
)

type meshBroadcastRequest struct {
	Channel string          `json:"channel"`
	Payload json.RawMessage `json:"payload"`
}

// handleMeshBroadcast allows agents to publish messages to the mesh
// (mesh:tasks or mesh:coordination).
func (s *Server) handleMeshBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req meshBroadcastRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	if req.Channel == "" || req.Payload == nil {
		http.Error(w, "missing channel or payload", http.StatusBadRequest)
		return
	}

	_ = r.Context()

	// In Cloud Mode (with Redis), this relies on the shared PubSub capabilities.
	// We broadcast tasks directly to Centrifuge to fulfill the root-level requirement for `mesh:tasks`.
	if req.Channel == "mesh:tasks" {
		if cnNode := s.hub.CentrifugeNode(); cnNode != nil {
			var parsedPayload map[string]interface{}
			if err := json.Unmarshal(req.Payload, &parsedPayload); err != nil {
				http.Error(w, "invalid JSON payload", http.StatusBadRequest)
				return
			}

			// Ensure it has agent_id, action, and status at the root.
			if _, ok := parsedPayload["agent_id"]; !ok {
				http.Error(w, "missing agent_id", http.StatusBadRequest)
				return
			}
			if _, ok := parsedPayload["action"]; !ok {
				http.Error(w, "missing action", http.StatusBadRequest)
				return
			}
			if _, ok := parsedPayload["status"]; !ok {
				http.Error(w, "missing status", http.StatusBadRequest)
				return
			}

			pubData, _ := json.Marshal(parsedPayload)
			_ = cnNode.Publish(req.Channel, pubData)
		}
	} else if req.Channel == "mesh:coordination" {
		if cnNode := s.hub.CentrifugeNode(); cnNode != nil {
			_ = cnNode.Publish(req.Channel, req.Payload)
		}
	}

	// Ensure we also broadcast internally for any backend listeners if needed.
	// Since TeammateMesh is not natively in dashboard server struct directly right now (it's in the hub or via Centrifuge node for UI),
	// this serves as the API ingress.

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	_ = json.NewEncoder(w).Encode(map[string]string{"status": "broadcasted"})
}
