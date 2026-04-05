package auth

import "net/http"

// RegisterPowerSyncRoutes registers the endpoints required by PowerSync.
func (h *Handlers) RegisterPowerSyncRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/api/auth/powersync/token", RequireRole("", h.HandlePowerSyncToken))
	mux.HandleFunc("/api/auth/powersync/jwks", h.HandlePowerSyncJWKS)
}
