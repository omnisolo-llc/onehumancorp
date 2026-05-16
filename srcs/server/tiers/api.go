package tiers

import (
	"encoding/json"
	"net/http"

	"onehumancorp/srcs/server/onboarding"
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

	tenantID, ok := r.Context().Value(onboarding.TenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized: missing or invalid tenant session", http.StatusUnauthorized)
		return
	}
	metric := r.URL.Query().Get("metric")

	if metric == "" {
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
