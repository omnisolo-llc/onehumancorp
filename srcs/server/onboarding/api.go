package onboarding

import (
	"encoding/json"
	"net/http"
	"strings"
)

type APIHandler struct {
	service *Service
}

func NewAPIHandler(service *Service) *APIHandler {
	return &APIHandler{service: service}
}

func getTenantIDFromAuth(r *http.Request) string {
	// Parse x-spiffe-id header like spiffe://onehumancorp.io/org-1/agent-1
	spiffe := r.Header.Get("x-spiffe-id")
	if spiffe != "" {
		parts := strings.Split(strings.TrimPrefix(spiffe, "spiffe://onehumancorp.io/"), "/")
		if len(parts) > 0 && parts[0] != "" {
			return parts[0]
		}
	}

	// Fallback to Bearer token logic if needed or return empty
	auth := r.Header.Get("Authorization")
	if strings.HasPrefix(auth, "Bearer ") {
		// Mock token extraction for now since Go side doesn't have full JWT decoder yet
		// We'll rely on spiffe id
	}
	return ""
}

func (h *APIHandler) HandleStartOnboarding(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req OnboardingRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	res, err := h.service.StartOnboarding(r.Context(), req)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusAccepted)
	json.NewEncoder(w).Encode(res)
}

func (h *APIHandler) HandleGetStatus(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// tenantID := r.URL.Query().Get("tenant_id")
	tenantID := getTenantIDFromAuth(r)
	if tenantID == "" {
		http.Error(w, "Missing or invalid tenant identity in session", http.StatusUnauthorized)
		return
	}

	res, err := h.service.GetOnboardingStatus(r.Context(), tenantID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(res)
}
