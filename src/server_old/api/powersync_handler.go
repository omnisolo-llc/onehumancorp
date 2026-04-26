package api

import (
	"encoding/json"
	"net/http"
	"log/slog"

	"github.com/onehumancorp/mono/src/server/db"
)

type PowerSyncHandler struct {
	db *db.DB
}

func NewPowerSyncHandler(database *db.DB) *PowerSyncHandler {
	return &PowerSyncHandler{
		db: database,
	}
}

func (h *PowerSyncHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.Method == http.MethodPost && r.URL.Path == "/api/v1/sync/push" {
		h.handlePush(w, r)
		return
	}
	if r.Method == http.MethodGet && r.URL.Path == "/api/v1/sync/pull" {
		h.handlePull(w, r)
		return
	}
	http.NotFound(w, r)
}

func (h *PowerSyncHandler) handlePush(w http.ResponseWriter, r *http.Request) {
	var payload []map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	slog.Info("PowerSync received push payload", "count", len(payload))
	// In a real implementation we would process the CRDTs/rows here.

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{"status": "ok"})
}

func (h *PowerSyncHandler) handlePull(w http.ResponseWriter, r *http.Request) {
	slog.Info("PowerSync received pull request")
	// Return some mock modifications
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode([]map[string]interface{}{})
}
