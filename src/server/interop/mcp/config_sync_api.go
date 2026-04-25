package mcp

import (
	"encoding/json"
	"net/http"
	"github.com/onehumancorp/mono/src/server/auth"


)

type SyncAPIHandler struct {
	tool *ConfigSyncTool
}

func NewSyncAPIHandler(tool *ConfigSyncTool) *SyncAPIHandler {
	return &SyncAPIHandler{tool: tool}
}

func (h *SyncAPIHandler) RegisterRoutes(mux *http.ServeMux) {
	mux.Handle("/api/v1/sync/config/hash", auth.RequireRole("system", h.handleGetHash))
	mux.Handle("/api/v1/sync/config", auth.RequireRole("system", h.handlePutConfig))
}

func (h *SyncAPIHandler) handleGetHash(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Real implementation fetching from DB
	// We need to fetch the last known config from the queue or another table.
	// For now, we simulate fetching by looking for the latest synced tool config in hybrid_mcp_sync_queue
	query := `SELECT payload FROM hybrid_mcp_sync_queue WHERE tool_name = 'mcp_config_sync' ORDER BY synced_at DESC LIMIT 1`
	var payloadStr string
	err := h.tool.proxy.dbProvider.QueryRow(r.Context(), query).Scan(&payloadStr)

	var hash string
	if err != nil {
		hash, _ = h.tool.GetHash(make(map[string]interface{}))
	} else {
		var payload map[string]interface{}
		json.Unmarshal([]byte(payloadStr), &payload)
		if hVal, ok := payload["hash"].(string); ok {
			hash = hVal
		} else {
			hash, _ = h.tool.GetHash(make(map[string]interface{}))
		}
	}
	if err != nil {
		http.Error(w, "Error generating hash", http.StatusInternalServerError)
		return
	}

	resp := map[string]string{"hash": hash}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}

func (h *SyncAPIHandler) handlePutConfig(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPut {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var payload ConfigPayload
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}

	// Attempt push sync
	if err := h.tool.Execute(r.Context(), payload.ConfigData, "push"); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
