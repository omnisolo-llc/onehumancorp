package mesh

import (
	"encoding/json"
	"net/http"

	"onehumancorp/srcs/server/orchestration"
)

type MeshHandler struct {
	Transport orchestration.MeshTransport
}

func NewMeshHandler(transport orchestration.MeshTransport) *MeshHandler {
	return &MeshHandler{Transport: transport}
}

func (h *MeshHandler) Broadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var msg orchestration.MeshMessage
	if err := json.NewDecoder(r.Body).Decode(&msg); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if msg.AgentID == "" || msg.Action == "" || msg.Status == "" {
		http.Error(w, "Missing required OHC-SIP fields (agent_id, action, status)", http.StatusBadRequest)
		return
	}

	channel := msg.Channel
	if channel == "" {
		channel = "mesh:broadcast"
	}

	data, err := json.Marshal(msg)
	if err != nil {
		http.Error(w, "Failed to marshal message", http.StatusInternalServerError)
		return
	}

	if err := h.Transport.Publish(r.Context(), channel, data); err != nil {
		http.Error(w, "Failed to publish message", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func (h *MeshHandler) Capabilities(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	skill := r.URL.Query().Get("skill")
	if skill == "" {
		http.Error(w, "Missing skill parameter", http.StatusBadRequest)
		return
	}

	agents, err := h.Transport.DiscoverAgents(r.Context(), skill)
	if err != nil {
		http.Error(w, "Failed to discover agents", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(agents)
}
