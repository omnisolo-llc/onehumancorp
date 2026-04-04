package orchestration

import (
	"encoding/json"
	"net/http"

	"github.com/google/uuid"
)

// HandleMeshDirect handles sending a direct message to a specific agent over the mesh.
func (h *Hub) HandleMeshDirect(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		ToAgent string `json:"to_agent"`
		Payload string `json:"payload"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	if req.ToAgent == "" {
		http.Error(w, "to_agent is required", http.StatusBadRequest)
		return
	}

	taskID := uuid.New().String()

	msg := Message{
		ID:        taskID,
		FromAgent: "system",
		ToAgent:   req.ToAgent,
		Type:      "mesh:direct",
		Content:   req.Payload,
	}

	// Deliver to the repository
	if h.repo != nil {
		if err := h.repo.PushMessage(r.Context(), req.ToAgent, msg); err != nil {
			http.Error(w, "failed to push message", http.StatusInternalServerError)
			return
		}
	} else {
	    // If no repo, just publish
	    _ = h.Publish(msg)
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok", "task_id":"` + taskID + `"}`))
}

// HandleMeshMailbox handles polling for mailbox messages.
func (h *Hub) HandleMeshMailbox(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	agentID := r.URL.Query().Get("agent_id")
	if agentID == "" {
		http.Error(w, "agent_id is required", http.StatusBadRequest)
		return
	}

	if h.repo == nil {
		w.Header().Set("Content-Type", "application/json")
		_, _ = w.Write([]byte(`{"messages":[]}`))
		return
	}

	messages, err := h.repo.PopMessages(r.Context(), agentID)
	if err != nil {
		http.Error(w, "failed to get mailbox messages", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	if len(messages) == 0 {
		_, _ = w.Write([]byte(`{"messages":[]}`))
		return
	}

	json.NewEncoder(w).Encode(map[string]interface{}{
		"messages": messages,
	})
}
