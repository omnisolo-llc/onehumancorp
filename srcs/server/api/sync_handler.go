package api

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// SyncHandler handles the incoming cloud synchronization payloads.
type SyncHandler struct {
	dbProvider db.Provider
}

// NewSyncHandler creates a new SyncHandler.
func NewSyncHandler(dbProvider db.Provider) *SyncHandler {
	return &SyncHandler{
		dbProvider: dbProvider,
	}
}

// HandleSyncMissions is the HTTP handler for POST /api/sync/missions.
func (h *SyncHandler) HandleSyncMissions(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method Not Allowed", http.StatusMethodNotAllowed)
		return
	}

	var payload struct {
		ID      string          `json:"id"`
		Status  string          `json:"status"`
		Payload json.RawMessage `json:"payload"`
	}

	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "Bad Request", http.StatusBadRequest)
		return
	}

	if payload.ID == "" || payload.Status == "" {
		http.Error(w, "Bad Request: missing required fields", http.StatusBadRequest)
		return
	}

	query := `
		INSERT INTO agent_missions (id, status, payload)
		VALUES ($1, $2, $3)
		ON CONFLICT (id) DO UPDATE SET status = EXCLUDED.status, payload = EXCLUDED.payload
	`
	if h.dbProvider.IsSQLite() {
		query = `
			INSERT INTO agent_missions (id, status, payload)
			VALUES ($1, $2, $3)
			ON CONFLICT (id) DO UPDATE SET status = excluded.status, payload = excluded.payload
		`
	}

	_, err := h.dbProvider.Exec(r.Context(), query, payload.ID, payload.Status, string(payload.Payload))
	if err != nil {
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
