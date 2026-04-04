package orchestration

import (
	"context"
	"encoding/json"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type MeshAPIHandler struct {
	pool db.Provider
	mesh TeammateMesh
}

func NewMeshAPIHandler(pool db.Provider, mesh TeammateMesh) *MeshAPIHandler {
	return &MeshAPIHandler{
		pool: pool,
		mesh: mesh,
	}
}

// BroadcastHandler handles POST /api/mesh/broadcast
func (h *MeshAPIHandler) BroadcastHandler(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	// Track metrics
	if telemetry.MeshBroadcastCount != nil {
		telemetry.MeshBroadcastCount.Add(ctx, 1)
	}

	var task Task
	if err := json.NewDecoder(r.Body).Decode(&task); err != nil {
		http.Error(w, "Invalid JSON payload", http.StatusBadRequest)
		return
	}

	if task.AgentID == "" || task.Action == "" || task.Status == "" {
		http.Error(w, "agent_id, action, and status are required at the root level", http.StatusBadRequest)
		return
	}

	if err := h.mesh.BroadcastTask(ctx, task); err != nil {
		http.Error(w, "Failed to broadcast task: "+err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{"status": "success"})
}

// DirectMessageHandler handles POST /api/mesh/direct
func (h *MeshAPIHandler) DirectMessageHandler(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	var msg Message
	if err := json.NewDecoder(r.Body).Decode(&msg); err != nil {
		http.Error(w, "Invalid JSON payload", http.StatusBadRequest)
		return
	}

	if msg.ToAgent == "" || msg.Content == "" {
		http.Error(w, "to_agent and content are required", http.StatusBadRequest)
		return
	}

	orgID := r.Header.Get("X-Org-ID")
	var repo HubRepository
	if h.pool.IsSQLite() {
		repo = NewSqliteHubRepository(h.pool, orgID)
	} else {
		repo = NewPgHubRepository(h.pool, orgID)
	}

	msg.ID = generateID()
	msg.OccurredAt = time.Now()

	if err := repo.PushMessage(ctx, msg.ToAgent, msg); err != nil {
		http.Error(w, "Failed to push message: "+err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{"status": "success"})
}

// MailboxHandler handles GET /api/mesh/mailbox
func (h *MeshAPIHandler) MailboxHandler(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	agentID := r.URL.Query().Get("agent_id")
	if agentID == "" {
		http.Error(w, "agent_id is required", http.StatusBadRequest)
		return
	}

	orgID := r.Header.Get("X-Org-ID")
	var repo HubRepository
	if h.pool.IsSQLite() {
		repo = NewSqliteHubRepository(h.pool, orgID)
	} else {
		repo = NewPgHubRepository(h.pool, orgID)
	}

	msgs, err := repo.PopMessages(ctx, agentID)
	if err != nil {
		http.Error(w, "Failed to pop messages: "+err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(msgs)
}
