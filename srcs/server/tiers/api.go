package tiers

import (
	"encoding/json"
	"net/http"
)

// APIHandler handles tier related requests
type APIHandler struct {
	svc *TierService
}

// NewAPIHandler creates a new APIHandler
func NewAPIHandler(svc *TierService) *APIHandler {
	return &APIHandler{svc: svc}
}

// HandleCheckLimit handles requests to check limits
func (h *APIHandler) HandleCheckLimit(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	tenantID := r.URL.Query().Get("tenant_id")
	metric := r.URL.Query().Get("metric")

	if tenantID == "" || metric == "" {
		http.Error(w, "Missing required parameters", http.StatusBadRequest)
		return
	}

	allowed, err := h.svc.CheckLimit(r.Context(), tenantID, metric, 1) // default increment of 1 for api check
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"allowed": allowed,
	})
}
