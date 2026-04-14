package mesh

import (
	"encoding/json"
	"net/http"
)

// MeshMessageRequest represents an incoming broadcast request.
type MeshMessageRequest struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
}

// BroadcastHandler handles POST /api/mesh/broadcast requests.
func BroadcastHandler(service TeammateMeshService) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		var req MeshMessageRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Bad request", http.StatusBadRequest)
			return
		}

		if req.AgentID == "" || req.Action == "" || req.Status == "" {
			http.Error(w, "Missing required OHC-SIP root fields", http.StatusBadRequest)
			return
		}

		intentBytes, err := json.Marshal(req)
		if err != nil {
			http.Error(w, "Internal server error", http.StatusInternalServerError)
			return
		}
		if err := service.BroadcastIntent(r.Context(), string(intentBytes)); err != nil {
			// Returning a static string to avoid returning raw error strings to clients.
			http.Error(w, "Internal server error", http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)
	}
}
