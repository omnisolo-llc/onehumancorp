package mesh

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type BroadcastRequest struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
	Content string `json:"content,omitempty"`
}

type CapabilitiesRequest struct {
	AgentID            string   `json:"agent_id"`
	SupportedSkills    []string `json:"supported_skills"`
	MaxConcurrentTasks int      `json:"max_concurrent_tasks"`
}

func HandleBroadcast(meshService TeammateMeshService) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		var req BroadcastRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Invalid request body", http.StatusBadRequest)
			return
		}

		if req.AgentID == "" || req.Action == "" || req.Status == "" {
			http.Error(w, "Missing required OHC-SIP root fields", http.StatusBadRequest)
			return
		}

		// Marshal it back to string for broadcasting
		intentBytes, err := json.Marshal(req)
		if err != nil {
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		if err := meshService.BroadcastIntent(r.Context(), string(intentBytes)); err != nil {
			http.Error(w, "Failed to broadcast", http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)
	}
}
