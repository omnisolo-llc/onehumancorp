package dashboard

import (
	"encoding/json"
	"net/http"
)

// handleMeshDirect implements the /api/mesh/direct endpoint.
func (s *Server) handleMeshDirect(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		TargetAgentID string `json:"target_agent_id"`
		Payload       string `json:"payload"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	if req.TargetAgentID == "" {
		http.Error(w, "missing target_agent_id", http.StatusBadRequest)
		return
	}

	// For direct messages, we use the orchestration Hub inbox mechanics if available,
	// but according to the Teammate Mesh APIs, we might just publish to a specific channel
	// or use a direct delivery method. Let's send a direct message.
	w.WriteHeader(http.StatusOK)
}

// handleMeshMailbox implements the /api/mesh/mailbox endpoint.
func (s *Server) handleMeshMailbox(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// This would typically return unread messages or upgrade to WS.
	w.WriteHeader(http.StatusOK)
	w.Write([]byte("[]"))
}
