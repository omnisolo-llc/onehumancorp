package mesh

import (
	"encoding/json"
	"net/http"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// MeshBroadcastRequest represents the expected OHC-SIP JSON structure for mesh broadcasts.
type MeshBroadcastRequest struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
	Payload interface{} `json:"payload,omitempty"`
}

// MeshHandler handles REST requests for the Teammate Mesh.
type MeshHandler struct {
	meshService TeammateMeshService
}

// NewMeshHandler creates a new MeshHandler.
func NewMeshHandler(service TeammateMeshService) *MeshHandler {
	return &MeshHandler{
		meshService: service,
	}
}

// ServeHTTP implements the http.Handler interface.
func (h *MeshHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	if !strings.HasSuffix(r.URL.Path, "/broadcast") {
		http.Error(w, "Not found", http.StatusNotFound)
		return
	}

	h.handleBroadcast(w, r)
}

func (h *MeshHandler) handleBroadcast(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	var req MeshBroadcastRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	// Validate OHC-SIP requirements: agent_id, action, and status at root.
	if req.AgentID == "" || req.Action == "" || req.Status == "" {
		http.Error(w, "Missing OHC-SIP required fields: agent_id, action, status", http.StatusBadRequest)
		return
	}

	data, err := json.Marshal(req)
	if err != nil {
		http.Error(w, "Internal server error", http.StatusInternalServerError)
		return
	}

	if err := h.meshService.BroadcastIntent(ctx, string(data)); err != nil {
		http.Error(w, "Failed to broadcast to mesh", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{"status": "broadcasted"})
}
