package sync

import (
	"context"
	"encoding/json"
	"net/http"
)

// HTTPHandler exposes the sync endpoint.
type HTTPHandler struct {
	service SyncService
}

// NewHTTPHandler creates a new HTTP handler.
func NewHTTPHandler(service SyncService) *HTTPHandler {
	return &HTTPHandler{service: service}
}

// HandleSync is the handler for /api/v1/sync/mcp-deltas
func (h *HTTPHandler) HandleSync(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method Not Allowed", http.StatusMethodNotAllowed)
		return
	}

	// Retrieve TenantID from Request Context natively in Go frameworks,
	// here we simulate injecting it if not already present based on auth headers to stay architecturally correct.
	tenantID, ok := r.Context().Value("tenant_id").(string)
	if !ok || tenantID == "" {
		tenantID = r.Header.Get("X-Tenant-ID") // Auth middleware extraction point simulation
		if tenantID == "" {
			w.WriteHeader(http.StatusUnauthorized)
			json.NewEncoder(w).Encode(ErrorResponse{Error: "Unauthorized", Description: "Missing Tenant from Context", Code: 401})
			return
		}
	}

	ctx := context.WithValue(r.Context(), "tenant_id", tenantID)

	var deltas []SyncDelta
	if err := json.NewDecoder(r.Body).Decode(&deltas); err != nil {
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode(ErrorResponse{Error: "Bad Request", Description: "Invalid JSON", Code: 400})
		return
	}

	if err := h.service.SyncDeltas(ctx, deltas); err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		json.NewEncoder(w).Encode(ErrorResponse{Error: "Internal Server Error", Description: err.Error(), Code: 500})
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(SyncResponse{Success: true, Message: "Synced successfully", Synced: len(deltas)})
}
