package mesh

import (
	"encoding/json"
	"net/http"

	"onehumancorp/srcs/server/orchestration"
)

// APIHandler handles mesh-related API requests
type APIHandler struct {
	meshHub orchestration.MeshHub
}

// NewAPIHandler creates a new APIHandler
func NewAPIHandler(meshHub orchestration.MeshHub) *APIHandler {
	return &APIHandler{
		meshHub: meshHub,
	}
}

// HandleBroadcast handles the POST /api/mesh/broadcast request
func (h *APIHandler) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		Topic   string `json:"topic"`
		Message struct {
			AgentID string          `json:"agent_id"`
			Action  string          `json:"action"`
			Status  string          `json:"status"`
			Payload json.RawMessage `json:"payload"`
			MsgID   string          `json:"msg_id"`
		} `json:"message"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if req.Topic == "" {
		http.Error(w, "topic is required", http.StatusBadRequest)
		return
	}

	if req.Message.AgentID == "" || req.Message.Action == "" || req.Message.Status == "" {
		http.Error(w, "agent_id, action, and status are required fields", http.StatusBadRequest)
		return
	}

	messageBytes, err := json.Marshal(req.Message)
	if err != nil {
		http.Error(w, "Failed to marshal message", http.StatusInternalServerError)
		return
	}

	err = h.meshHub.Publish(r.Context(), req.Topic, messageBytes)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]bool{"success": true})
}

func (h *APIHandler) HandleCapabilities(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodPost:
		h.handleAdvertiseCapabilities(w, r)
	case http.MethodGet:
		h.handleDiscoverAgents(w, r)
	default:
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
	}
}

func (h *APIHandler) handleAdvertiseCapabilities(w http.ResponseWriter, r *http.Request) {
	var agent orchestration.Agent
	if err := json.NewDecoder(r.Body).Decode(&agent); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if agent.ID == "" {
		http.Error(w, "agent_id is required", http.StatusBadRequest)
		return
	}

	if err := h.meshHub.AdvertiseCapabilities(r.Context(), agent); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]bool{"success": true})
}

func (h *APIHandler) handleDiscoverAgents(w http.ResponseWriter, r *http.Request) {
	skill := r.URL.Query().Get("skill")
	if skill == "" {
		http.Error(w, "skill parameter is required", http.StatusBadRequest)
		return
	}

	agents, err := h.meshHub.DiscoverAgents(r.Context(), skill)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]interface{}{"agents": agents})
}
