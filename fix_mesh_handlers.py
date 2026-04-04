import re

with open('srcs/server/dashboard/server.go', 'r') as f:
    content = f.read()

handlers_to_add = """
	// Teammate Mesh APIs
	mux.HandleFunc("/api/mesh/broadcast", server.handleMeshBroadcast)
	mux.HandleFunc("/api/mesh/direct", server.handleMeshDirect)
	mux.HandleFunc("/api/mesh/mailbox", server.handleMeshMailbox)
"""

content = re.sub(
    r'// Teammate Mesh APIs\s+mux\.HandleFunc\("/api/mesh/broadcast", server\.handleMeshBroadcast\)',
    handlers_to_add.strip(),
    content
)

mesh_funcs = """
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

	err := s.hub.Publish(orchestration.Message{
		ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
		FromAgent: "system",
		ToAgent:   req.TargetAgentID,
		Type:      "mesh:direct",
		Content:   req.Payload,
	})

	if err != nil {
		http.Error(w, "failed to send direct message", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok"}`))
}

func (s *Server) handleMeshMailbox(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	agentID := r.URL.Query().Get("agent_id")
	if agentID == "" {
		http.Error(w, "missing agent_id", http.StatusBadRequest)
		return
	}

	// For simplicity in polling mode, we might just return empty or connected status
	// In a real implementation we would upgrade to WebSocket or return queued messages.
	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok", "messages":[]}`))
}
"""

content = content.replace("func (s *Server) handleCosts", mesh_funcs + "\nfunc (s *Server) handleCosts")

with open('srcs/server/dashboard/server.go', 'w') as f:
    f.write(content)
