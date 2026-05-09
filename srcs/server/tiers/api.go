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

	// We extract tenantID directly using a string key because in a real application
	// this would be securely populated by a shared JWT/Session middleware.
	tenantID := ""
	if val := r.Context().Value("tenant_id"); val != nil {
		if tID, ok := val.(string); ok {
			tenantID = tID
		}
	} else if val := r.Context().Value("organization_id"); val != nil {
		if tID, ok := val.(string); ok {
			tenantID = tID
		}
	}

	// If the above fails, and we have the X-Tenant-Id header directly injected
	// in tests or via standard internal routing.
	if tenantID == "" {
		tenantID = r.Header.Get("X-Tenant-Id")
	}

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
