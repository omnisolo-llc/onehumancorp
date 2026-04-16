package billing

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/services/billing"
)

type Handler struct {
	Service *billing.Service
}

func (h *Handler) GetInvoices(w http.ResponseWriter, r *http.Request) {
	tenantID := r.URL.Query().Get("tenant_id")
	if tenantID == "" {
		http.Error(w, "tenant_id is required", http.StatusBadRequest)
		return
	}

	invoices, err := h.Service.GetInvoices(r.Context(), tenantID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(invoices)
}

func (h *Handler) RecordUsage(w http.ResponseWriter, r *http.Request) {
	var req struct {
		TenantID     string `json:"tenant_id"`
		ResourceType string `json:"resource_type"`
		Units        int64  `json:"units"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	if err := h.Service.RecordUsage(r.Context(), req.TenantID, req.ResourceType, req.Units); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}
